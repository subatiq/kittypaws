use std::io::BufRead;
use std::process::Command;
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use crate::plug::{unwrap_home_path, PluginInterface, CallablePlugin, PLUGINS_PATH};

use super::StatusValue;


struct BashCommand {
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

    fn status(&self, config: &HashMap<String, String>) -> Result<HashMap<String, StatusValue>, String> {
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
            println!("Status stderr: {}", String::from_utf8(output.stderr).unwrap());

            let mut status = HashMap::new();
            for key_value in output.stdout.lines() {
                if let Ok(key_value) = key_value {
                    if let Some((key, value)) = key_value.split_once("=") {
                        let mut parsed_value = StatusValue::String(value.to_string());
                        if let Ok(value) = value.parse::<i32>() {
                            parsed_value = StatusValue::Int(value);
                        }
                        else if let Ok(value) = value.parse::<f32>() {
                            parsed_value = StatusValue::Float(value);
                        }
                        status.insert(key.to_string(), parsed_value);
                    }
                }
            }
            return Ok(status);
        }

        unreachable!()
    }
}

pub fn load(name: &str) -> Result<CallablePlugin, String> {
    let plugins_path = unwrap_home_path(PLUGINS_PATH);
    let plugins_dirname = plugins_path.to_str().expect("Can't find home directory for the current user");

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

    Ok(Box::new(BashCommand { executable, status_checker }))
}

