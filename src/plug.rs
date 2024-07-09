use std::fs;
mod bash_plugin;
mod python_plugin;
use bash_plugin::load as load_sh_plugin;
use chrono::{DateTime, Utc};
use python_plugin::load as load_py_plugin;

use crate::intervals::{time_till_next_run, wait_duration};
use crate::stdout_styling::style_line;
use paws_config::{Duration as ConfigDuration, KittypawsConfig, PluginConfig};
use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

const PLUGINS_PATH: &str = "~/.kittypaws/plugins";
pub type CallablePlugin = Box<dyn PluginInterface + Send + Sync + 'static>;

#[derive(Debug)]
pub enum PluginLanguage {
    Python,
    Bash,
}

#[derive(Debug)]
pub enum StatusValue {
    Int(i32),
    Float(f32),
    String(String),
}


pub trait PluginInterface {
    fn run(&self, config: &HashMap<String, String>) -> Result<(), String>;
    fn status(&self, config: &HashMap<String, String>) -> Result<HashMap<String, StatusValue>, String>;
}

#[derive(Debug)]
pub enum StartupMode {
    Immediatelly,
    Delayed(Duration),
    AfterInterval,
}

impl From<paws_config::StartupOptions> for StartupMode {
    fn from(value: paws_config::StartupOptions) -> Self {
        match value {
            paws_config::StartupOptions::Hot => StartupMode::Immediatelly,
            paws_config::StartupOptions::Cold => StartupMode::AfterInterval,
            paws_config::StartupOptions::Delayed(duration) => {
                StartupMode::Delayed(duration.as_std())
            }
        }
    }
}

fn call_plugin(name: &str, plugin: &CallablePlugin, config: &HashMap<String, String>) {
    println!("{}", style_line(name.to_string(), "Running...".to_string()));
    if let Err(err) = plugin.run(config) {
        panic!("Error while running plugin {}: {}", name, err);
    }
}

fn get_status(name: &str, plugin: &CallablePlugin, config: &HashMap<String, String>) {
    println!(
        "{}",
        style_line(name.to_string(), "Fetching status...".to_string())
    );
    match plugin.status(config) {
        Ok(status) => {
            println!(
                "{}",
                style_line(name.to_string(), format!("Status: {:?}", status))
            );
        }
        Err(err) => panic!("Error while running plugin {}: {}", name, err),
    }
}

fn start_execution_loop(
    plugin: CallablePlugin,
    config: PluginConfig,
    loop_duration: &Option<ConfigDuration>,
) -> JoinHandle<()> {
    let startup = config.startup.into();
    let mut deadline: Option<DateTime<Utc>> = None;

    if let Some(loop_duration) = loop_duration {
        deadline = Some(Utc::now() + loop_duration.as_chrono());
    }
    thread::spawn(move || {
        match startup {
            StartupMode::Delayed(delay) => wait_duration(delay),
            StartupMode::Immediatelly => {}
            StartupMode::AfterInterval => {
                time_till_next_run(&config.frequency);
            }
        };

        loop {
            call_plugin(
                &config.name,
                &plugin,
                &config.options.clone().unwrap_or_default(),
            );
            if time_till_next_run(&config.frequency).is_none() {
                break;
            }
            if let Some(deadline) = deadline {
                if Utc::now() > deadline {
                    break;
                }
            }
        }
    })
}

fn start_status_loop(
    plugin: CallablePlugin,
    config: PluginConfig,
    loop_duration: &Option<ConfigDuration>,
) -> Option<JoinHandle<()>> {
    if let Some(monitoring_config) = config.monitoring {
        let mut deadline: Option<DateTime<Utc>> = None;

        if let Some(loop_duration) = loop_duration {
            deadline = Some(Utc::now() + loop_duration.as_chrono());
        }

        return Some(thread::spawn(move || loop {
            let status = get_status(
                &config.name,
                &plugin,
                &config.options.clone().unwrap_or_default(),
            );
            println!("Status {:?}", status);
            if time_till_next_run(&monitoring_config.frequency).is_none() {
                break;
            }
            if let Some(deadline) = deadline {
                if Utc::now() > deadline {
                    break;
                }
            }
        }));
    }

    None
}

pub fn start_main_loop(config: KittypawsConfig) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for plugconf in config.plugins {
        // TODO: Stop this uglyness
        match load_plugin(&plugconf.name) {
            Ok(plugin) => {
                if let Some(status_thread) =
                    start_status_loop(plugin, plugconf.clone(), &config.duration)
                {
                    handles.push(status_thread);
                }
            }
            Err(err) => println!("! WARNING: {}", err),
        }
        match load_plugin(&plugconf.name) {
            Ok(plugin) => {
                let exec_thread = start_execution_loop(plugin, plugconf, &config.duration);

                handles.push(exec_thread);
            }
            Err(err) => println!("! WARNING: {}", err),
        }
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            println!("Error: {:?}", e);
        }
    }
}

fn unwrap_home_path(path: &str) -> PathBuf {
    if path.starts_with('~') {
        match std::env::var("HOME") {
            Ok(home) => PathBuf::from(&path.replace('~', &home)),
            Err(_) => {
                println!("Could not find home directory");
                PathBuf::from(path)
            }
        }
    } else {
        PathBuf::from(path)
    }
}

fn get_files_list(path: &PathBuf) -> Vec<String> {
    dbg!(path);
    return fs::read_dir(path)
        .unwrap()
        .map(|x| x.unwrap().file_name().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
}

fn get_path_to_plugin(name: &str) -> PathBuf {
    let path_to_plugins = unwrap_home_path(PLUGINS_PATH);
    path_to_plugins.join(name)
}

fn detect_language(name: &str) -> PluginLanguage {
    let path_to_plugin = get_path_to_plugin(name);
    if get_files_list(&path_to_plugin).contains(&"run.sh".to_string()) {
        return PluginLanguage::Bash;
    }

    PluginLanguage::Python
}

fn load_plugin(name: &str) -> Result<CallablePlugin, String> {
    match detect_language(name) {
        PluginLanguage::Python => load_py_plugin(name),
        PluginLanguage::Bash => load_sh_plugin(name),
    }
}
