use crate::plug::{unwrap_home_path, CallablePlugin, PluginInterface, PLUGINS_PATH};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use crate::plugin_logger::PluginLogger;

struct BashCommand {
    executable: PathBuf,
}

impl PluginInterface for BashCommand {
    fn run(&self, name: &str, config: &HashMap<String, String>) -> Result<Box<dyn Display>, String> {
        // get a string of args and values from hashmap
        let mut child = Command::new("bash")
            .envs(config)
            .arg("-C")
            .arg(self.executable.to_str().unwrap())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to start process {}");

        // Wrap streams into a buffered stream, then box it
        let stdout = Box::new(BufReader::new(child.stdout.take().expect("Stdout is unavailable")));
        let stderr = Box::new(BufReader::new(child.stderr.take().expect("Stderr is unavailable")));

        // Init logger
        let plugin_logger = PluginLogger::new(
            name,
            vec![stdout, stderr]
        );

        // Run the plugin
        let status = child.wait();
        let exit_code = status.unwrap().code().or(Some(-1)).unwrap();
        plugin_logger.stop();

        let msg = format!(
            "{} command exited with code {}",
            self.executable.to_str().unwrap(),
            exit_code
        );

        let result: Result<Box<dyn Display>, String> = {
            match exit_code {
                0 => Ok(Box::new(msg)),
                _ => Err(msg),
            }
        };

        // print non-error result
        // return error message
        result
    }
}

pub fn load(name: &str) -> Result<CallablePlugin, String> {
    let plugins_path = unwrap_home_path(PLUGINS_PATH);
    let plugins_dirname = plugins_path
        .to_str()
        .expect("Can't find home directory for the current user");

    let entrypoint_path = format!("{}/{}/run.sh", &plugins_dirname, name);
    let path_to_main = Path::new(&entrypoint_path);

    match path_to_main.exists() {
        true => Ok(Box::new(BashCommand {
            executable: path_to_main.to_path_buf(),
        })),
        false => Err(format!("No main.py found for plugin: {}", name)),
    }
}
