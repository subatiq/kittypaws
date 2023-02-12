use std::process::{Command, Child, Stdio, ChildStdout};
use std::path::{Path,PathBuf};
use std::collections::HashMap;
use crate::plug::{unwrap_home_path, PluginInterface, CallablePlugin, PLUGINS_PATH};
use std::io::Read;
use chrono::Local;


struct BashCommand {
    executable: PathBuf,
}

fn get_process_status(child: &mut Child) -> Option<Result<i32, String>> {
    // Checks the exit code of a process, and returns None if it is still running.
    // If there is an error, it returns the error
    match child.try_wait() {
        // -1 === process was killed by a signal
        Ok(Some(status)) => Some(Ok(status.code().unwrap_or(-1))),
        Err(msg) => Some(Err(msg.to_string())),
        Ok(None) => None,
    }
}
fn detect_line(bytes: &Vec<u8>) -> Option<String> {
    if bytes.len() > 0 && bytes[bytes.len() - 1] == b'\n' {
        return Some(String::from_utf8(bytes[..bytes.len() - 1].to_vec()).unwrap());
    }
    None
}

fn log(line: &str) {
    println!("[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), line);
}

fn try_get_line(stdout: &mut ChildStdout, collected: &mut Vec<u8>) -> Option<String>{
    let bytebuff: &mut [u8; 1] = &mut [0; 1];
    match stdout.read_exact(bytebuff) {
        Ok(_) => {
            collected.push(bytebuff[0]);
        },
        Err(_) => {
            // println!("Error: {}", e);
        }
    }

    match detect_line(&collected) {
        Some(string) => {
            return Some(string);
        }
        None => None,
    }
}

impl PluginInterface for BashCommand {
    fn run(&self, config: &HashMap<String, String>) -> Result<(), String> {
        // get a string of args and values from hashmap
        let mut child = Command::new("bash")
             .envs(config)
             .arg("-C")
             .arg(self.executable.to_str().unwrap())
             .stdout(Stdio::piped())
             // .args(&args)
             .spawn()
             .expect("failed to start process {}");
        let mut stdout = child.stdout.take().expect("Stdout is available");

        let mut status;
        let mut line: Vec<u8> = vec![];

        loop {
            match try_get_line(&mut stdout, &mut line) {
                Some(string) => {
                    log(&string);
                    line = vec![];
                }
                None => {}
            }
            status = get_process_status(&mut child);
            if let Some(_) = status {
                break;
            }
        }
        let status = status.unwrap();
        let result: Result<String, String> = match status {
            Ok(code) => {
                let msg = format!("{} command exited with code {}",self.executable.to_str().unwrap(), code);
                match code {
                    0 => Ok(msg),
                    _ => Err(msg)
                }
            },
            Err(msg) => Err(msg),
        };


        // print non-error result
        // return error message
        match &result {
            Ok(msg) => println!("{}", msg),
            _ => {}
        }
        result.map(|_| ())
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

