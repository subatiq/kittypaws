use std::fs;
mod python_plugin;
mod bash_plugin;
use python_plugin::load as load_py_plugin;
use bash_plugin::load as load_sh_plugin;

use std::thread::JoinHandle;
use std::thread;
use std::sync::Mutex;
use std::path::PathBuf;
use std::collections::HashMap;
use std::time::Duration;
use iso8601_duration::Duration as ISODuration;
use crate::settings::{PluginsConfig, FromConfig};
use crate::intervals::{Frequency, wait_for_next_run, wait_duration};
use crate::stdout_styling::style_line;
use lazy_static::lazy_static;


const PLUGINS_PATH: &str = "~/.kittypaws/plugins";
pub type CallablePlugin = Box<dyn PluginInterface + Send + Sync + 'static>;
lazy_static! {
    static ref STDOUT_MUTEX: Mutex<()> = Mutex::new(());
}


#[derive(Debug)]
pub enum PluginLanguage {
    PYTHON,
    BASH,
}

pub trait PluginInterface {
    fn run(&self, config: &HashMap<String, String>) -> Result<(), String>;
}


#[derive(Debug)]
pub enum StartupMode {
    Immediatelly,
    Delayed(Duration),
    AfterInterval
}


impl FromConfig<StartupMode> for StartupMode {
    fn from_config(config: &HashMap<String, String>) -> Result<StartupMode, String> {
        if !config.contains_key("startup") {
            return Ok(StartupMode::AfterInterval);
        }

        match config.get("startup").unwrap().as_str() {
            "hot" => Ok(StartupMode::Immediatelly),
            "cold" => Ok(StartupMode::AfterInterval),
            duration => {
                let duration = ISODuration::parse(duration).expect("duration isn't in ISO8601 format").to_std();
                return Ok(StartupMode::Delayed(duration));
            }
        }

    }
}



fn call_plugin(name: &str, plugin: &CallablePlugin, config: &HashMap<String, String>) {
    println!("{}", style_line(name.to_string(), "Running...".to_string()));
    match plugin.run(&config) {
        Err(err) => panic!("Error while running plugin {}: {}", name, err),
        _ => {}
    };
}

fn start_plugin_loop(name: String, plugin: CallablePlugin, config: HashMap<String, String>) -> JoinHandle<()> {
    let run_frequency = Frequency::from_config(&config).expect("Frequency is poorly configured");
    let startup = StartupMode::from_config(&config).expect("StartupMode is poorly configured");

    return thread::spawn(move || {
        match startup {
            StartupMode::Delayed(delay) => wait_duration(delay),
            StartupMode::Immediatelly => {}
            StartupMode::AfterInterval => {
                wait_for_next_run(&run_frequency);
            }
        };

        loop {
            call_plugin(&name, &plugin, &config);
            match wait_for_next_run(&run_frequency) {
                None => break,
                _ => {}
            }
        }
    })
}

pub fn start_main_loop(config: &PluginsConfig) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for plugconf in config {
        let name = plugconf.keys().last().expect("Name of the plugin is not specified properly in the config");
        let plugconfig = plugconf.get(name).expect(&format!("Can't parse config for plugin {}", &name));

        match load_plugin(&name) {
            Ok(plugin) => {
                let plugconfig = plugconfig.clone();
                let thread = start_plugin_loop(name.to_string(), plugin, plugconfig);

                handles.push(thread);
            }
            Err(err) => println!("! WARNING: {}", err)
        }
    }


    for handle in handles {
        match handle.join() {
            Err(e) => {
                println!("Error: {:?}", e);
            },
            _ => {}
        }
    }
}


fn unwrap_home_path(path: &str) -> PathBuf {
    if path.starts_with("~") {
        match std::env::var("HOME") {
            Ok(home) => {
                PathBuf::from(&path.replace("~", &home))
            }
            Err(_) => {
                println!("Could not find home directory");
                PathBuf::from(path)
            }
        }
    }
    else {
        PathBuf::from(path)
    }
}

fn get_files_list(path: &PathBuf) -> Vec<String> {
    dbg!(path);
    return fs::read_dir(path).unwrap()
        .map(|x| x.unwrap()
        .file_name()
        .to_str().unwrap()
        .to_string()).collect::<Vec<String>>();
}

fn get_path_to_plugin(name: &str) -> PathBuf {
    let path_to_plugins = unwrap_home_path(PLUGINS_PATH);
    return path_to_plugins.join(name);
}


fn detect_language(name: &str) -> PluginLanguage {
    let path_to_plugin = get_path_to_plugin(&name);
    if get_files_list(&path_to_plugin).contains(&"run.sh".to_string()) {
        return PluginLanguage::BASH
    }

    return PluginLanguage::PYTHON;
}


fn load_plugin(name: &str) -> Result<CallablePlugin, String> {
    match detect_language(name) {
        PluginLanguage::PYTHON => load_py_plugin(&name),
        PluginLanguage::BASH => load_sh_plugin(&name)
    }
}
