use paws_config::MonitoringOptions;
use telegraf::protocol::{Field, Tag};
use telegraf::FieldData;

use crate::plug::{unwrap_home_path, CallablePlugin, PluginInterface, PLUGINS_PATH};
use std::collections::HashMap;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::Command;

struct BashCommand {
    name: String,
    executable: PathBuf,
    status_checker: Option<PathBuf>,
}

impl PluginInterface for BashCommand {
    fn run(&self, config: &HashMap<String, String>) -> Result<(), String> {
        // get a string of args and values from hashmap
        let output = Command::new("bash")
            .envs(config)
            .arg("-C")
            .arg(self.executable.to_str().unwrap())
            // .args(&args)
            .output()
            .expect("failed to execute process {}");

        #[cfg(debug_assertions)]
        println!("Run stdout: {}", String::from_utf8(output.stdout).unwrap());
        #[cfg(debug_assertions)]
        println!("Run stderr: {}", String::from_utf8(output.stderr).unwrap());

        Ok(())
    }

    fn status(
        &self,
        config: &HashMap<String, String>,
        monitoring_config: &Option<MonitoringOptions>,
    ) -> Result<telegraf::Point, String> {
        // get a string of args and values from hashmap
        if let Some(command) = &self.status_checker {
            let output = Command::new("bash")
                .envs(config)
                .arg("-C")
                .arg(command.to_str().unwrap())
                // .args(&args)
                .output()
                .expect("failed to execute process {}");

            #[cfg(debug_assertions)]
            println!(
                "Status stderr: {}",
                String::from_utf8(output.stderr).unwrap()
            );

            let mut fields = Vec::new();
            for key_value in output.stdout.lines() {
                if let Ok(key_value) = key_value {
                    if let Some((key, value)) = key_value.split_once("=") {
                        let parsed_value = if let Ok(value) = value.parse::<i64>() {
                            FieldData::Number(value)
                        } else if let Ok(value) = value.parse::<f64>() {
                            FieldData::Float(value)
                        } else {
                            FieldData::Str(value.to_string())
                        };
                        fields.push(Field {
                            name: key.to_string(),
                            value: parsed_value,
                        });
                    }
                }
            }

            let mut tags = vec![Tag {
                name: "name".to_string(),
                value: self.name.clone(),
            }];
            if let Some(monitoring_config) = monitoring_config {
                if let Some(extra_tags) = monitoring_config.extra_tags.clone() {
                    tags.extend(extra_tags.iter().map(|(key, value)| Tag {
                        name: key.to_owned(),
                        value: value.to_owned(),
                    }));
                }
            }

            return Ok(telegraf::Point {
                measurement: "kittypaws".to_owned(),
                tags,
                fields,
                timestamp: None,
            });
        }

        unreachable!()
    }
}

pub fn load(name: &str) -> Result<CallablePlugin, String> {
    let plugins_path = unwrap_home_path(PLUGINS_PATH);
    let plugins_dirname = plugins_path
        .to_str()
        .expect("Can't find home directory for the current user");

    let entrypoint_path = format!("{}/{}/run.sh", &plugins_dirname, name);
    let path_to_main = Path::new(&entrypoint_path);

    let entrypoint_path = format!("{}/{}/status.sh", &plugins_dirname, name);
    let path_to_status = Path::new(&entrypoint_path);
    let executable = path_to_main.to_path_buf();

    let mut status_checker = None;
    if path_to_status.exists() {
        status_checker = Some(path_to_status.to_path_buf());
    }

    if !path_to_main.exists() {
        return Err(format!("No main.py found for plugin: {}", name));
    }

    Ok(Box::new(BashCommand {
        name: name.to_string(),
        executable,
        status_checker,
    }))
}
