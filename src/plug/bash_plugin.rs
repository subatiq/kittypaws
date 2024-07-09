use crate::plug::{unwrap_home_path, CallablePlugin, PluginInterface, PLUGINS_PATH};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

struct BashCommand {
    run_script: PathBuf,
    stop_script: Option<PathBuf>,
}

impl PluginInterface for BashCommand {
    fn run(&self, config: &HashMap<String, String>) -> Result<(), String> {
        // get a string of args and values from hashmap
        let output = Command::new("bash")
            .envs(config)
            .arg("-C")
            .arg(self.run_script.to_str().unwrap())
            // .args(&args)
            .output()
            .expect("failed to execute process {}");

        println!("{}", String::from_utf8(output.stdout).unwrap());
        Ok(())
    }

    fn stop(&self, config: &HashMap<String, String>) -> Result<(), String> {
        let output = Command::new("bash")
            .envs(config)
            .arg("-C")
            .arg(self.run_script.to_str().unwrap())
            // .args(&args)
            .output()
            .expect("failed to execute process {}");

        println!("{}", String::from_utf8(output.stdout).unwrap());
        Ok(())
    }
}

pub fn load(name: &str) -> Result<CallablePlugin, String> {
    let plugins_path = unwrap_home_path(PLUGINS_PATH);
    let plugins_dirname = plugins_path
        .to_str()
        .expect("Can't find home directory for the current user");

    let path_to_run = PathBuf::from_str(&format!("{}/{}/run.sh", &plugins_dirname, name)).unwrap();
    let path_to_stop =
        PathBuf::from_str(&format!("{}/{}/stop.sh", &plugins_dirname, name)).unwrap();

    let path_to_stop = if path_to_stop.exists() {
        Some(path_to_stop)
    } else {
        None
    };

    match path_to_run.exists() {
        true => Ok(Box::new(BashCommand {
            run_script: path_to_run,
            stop_script: path_to_stop,
        })),
        false => Err(format!("No main.py found for plugin: {}", name)),
    }
}
