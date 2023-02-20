use crate::plug::{unwrap_home_path, CallablePlugin, PluginInterface, PLUGINS_PATH};
use chrono::Local;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdout, Command, Stdio};

struct BashCommand {
    executable: PathBuf,
}

enum ProcessStatus {
    Running,
    Finished(i32),
}

fn get_process_status(child: &mut Child) -> Result<ProcessStatus, String> {
    match child.try_wait() {
        // -1 === process was killed by a signal
        Ok(Some(status)) => Ok(ProcessStatus::Finished(status.code().unwrap_or(-1))),
        Ok(None) => Ok(ProcessStatus::Running),
        Err(msg) => Err(format!("{:?}", msg)),
    }
}
fn detect_line(bytes: &Vec<u8>) -> Option<String> {
    if bytes.len() <= 0 || bytes[bytes.len() - 1] != b'\n' {
        return None;
    }
    Some(String::from_utf8(bytes[..bytes.len() - 1].to_vec()).unwrap())
}

fn log(line: &str) {
    println!("[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), line);
}

fn try_get_line(
    stdout: &mut ChildStdout,
    collected: &mut Vec<u8>,
) -> Result<Option<String>, String> {
    let bytebuff: &mut [u8; 1] = &mut [0; 1];

    match stdout.read_exact(bytebuff) {
        Ok(_) => {}
        Err(eof) if eof.kind() == ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(format!("{}", e)),
    }

    collected.push(bytebuff[0]);
    Ok(detect_line(&collected))
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
        let mut stdout = child.stdout.take().expect("Stdout is unavailable");

        let mut status;
        let mut line: Vec<u8> = vec![];

        loop {
            match try_get_line(&mut stdout, &mut line) {
                Ok(None) => {}
                Ok(Some(string)) => {
                    log(&string);
                    line = vec![];
                }
                Err(e) => {
                    println!("{}", e)
                }
            }
            status = get_process_status(&mut child);
            if let Ok(ProcessStatus::Finished(_)) = status {
                break;
            }
        }

        let exit_code = match status? {
            ProcessStatus::Finished(status) => status,
            _ => unreachable!(),
        };

        let msg = format!(
            "{} command exited with code {}",
            self.executable.to_str().unwrap(),
            exit_code
        );

        let result: Result<String, String> = {
            match exit_code {
                0 => Ok(msg),
                _ => Err(msg),
            }
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
