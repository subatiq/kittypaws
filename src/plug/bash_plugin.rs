use std::process::Command;
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use crate::plug::{unwrap_home_path, PluginInterface, CallablePlugin, PLUGINS_PATH};


struct BashCommand {
    executable: PathBuf,
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

        println!("{}", String::from_utf8(output.stdout).unwrap());
        Ok(())
    }
}

pub fn load(name: &str) -> Result<CallablePlugin, String> {
    let plugins_path = unwrap_home_path(PLUGINS_PATH);
    let plugins_dirname = plugins_path.to_str().expect("Can't find home directory for the current user");

    let entrypoint_path = format!("{}/{}/run.sh", &plugins_dirname, name);
    let path_to_main = Path::new(&entrypoint_path);

    match path_to_main.exists() {
        true => Ok(Box::new(BashCommand { executable : path_to_main.to_path_buf() })),
        false => Err(format!("No main.py found for plugin: {}", name))
    }
}

