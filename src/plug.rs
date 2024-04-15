use std::fs;
mod python_plugin;
mod bash_plugin;
use python_plugin::load as load_py_plugin;
use bash_plugin::load as load_sh_plugin;

use std::thread::JoinHandle;
use std::thread;
use std::path::PathBuf;
use std::collections::HashMap;
use std::time::Duration;
use paws_config::{KittypawsConfig, PluginConfig};
use crate::intervals::{wait_for_next_run, wait_duration};
use crate::stdout_styling::style_line;


const PLUGINS_PATH: &str = "~/.kittypaws/plugins";
pub type CallablePlugin = Box<dyn PluginInterface + Send + Sync + 'static>;


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


impl From<paws_config::StartupOptions> for StartupMode {
    fn from(value: paws_config::StartupOptions) -> Self {
        match value {
            paws_config::StartupOptions::Hot => StartupMode::Immediatelly,
            paws_config::StartupOptions::Cold => StartupMode::AfterInterval,
            paws_config::StartupOptions::Delayed(duration) => StartupMode::Delayed(duration.as_std())
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

fn start_plugin_loop(plugin: CallablePlugin, config: PluginConfig) -> JoinHandle<()> {
    let startup = config.startup.into();

    return thread::spawn(move || {
        match startup {
            StartupMode::Delayed(delay) => wait_duration(delay),
            StartupMode::Immediatelly => {}
            StartupMode::AfterInterval => {
                wait_for_next_run(&config.frequency);
            }
        };

        loop {
            call_plugin(&config.name, &plugin, &config.options.clone().unwrap_or(HashMap::new()));
            match wait_for_next_run(&config.frequency) {
                None => break,
                _ => {}
            }
        }
    })
}

pub fn start_main_loop(config: KittypawsConfig) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for plugconf in config.plugins {
        match load_plugin(&plugconf.name) {
            Ok(plugin) => {
                let thread = start_plugin_loop(plugin, plugconf);

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
