use std::fs;
use std::thread::JoinHandle;
use std::thread;
use std::sync::{Arc, Mutex};
use std::fmt;
use std::error;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::{PyModule, PyList};

const PLUGINS_PATH: &str = "~/.kittypaws/plugins";
type CallablePlugin = Box<dyn PluginInterface + Send + Sync + 'static>;
type PluginConfig = HashMap<String, HashMap<String, String>>;

#[derive(Debug)]
pub enum PluginLanguage {
    RUST,
    PYTHON
}

pub struct Plugin {
    pub name: String,
    pub language: PluginLanguage
}


pub trait PluginInterface {
    fn run(&self, config: &HashMap<String, String>);
}


impl PluginInterface for pyo3::Py<PyAny> {
    fn run(&self, config: &HashMap<String, String>) {
        let mut pyconfig = HashMap::new();
        pyconfig.insert("config", &config);
        Python::with_gil(|py| self.call(py, (), Some(pyconfig.into_py_dict(py)))).unwrap();
    }
}


pub struct PluginManifest {
    plugins: HashMap<String, CallablePlugin>,
}



impl PluginManifest {
    pub fn load_plugins(config: &PluginConfig) -> PluginManifest {
        let mut manifest = PluginManifest {
            plugins: HashMap::new()
        };
        load_plugins(get_plug_list(&config), &mut manifest);
        manifest
    }

    pub fn register(&mut self, name: String, interface: CallablePlugin) {
        self.plugins.insert(name, interface);
    }

    fn run_plugin(&self, plugin: CallablePlugin, config: HashMap<String, String>) -> JoinHandle<()> {
        let self_copy = Arc::new(Mutex::new(plugin)).clone();
        return thread::spawn(move || {
            self_copy.lock().unwrap().run(&config);
        })
    }

    pub fn run(&mut self, config: &HashMap<String, HashMap<String, String>>) {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        let keys = self.plugins.keys().into_iter().map(|x| x.to_string()).collect::<Vec<String>>();

        for name in keys {
            match config.get(&name) {
                Some(plugconfig) => {
                    dbg!(&plugconfig);
                    let plugin = self.plugins.remove(&name).unwrap();
                    let plugconfig = plugconfig.clone();
                    let thread = self.run_plugin(plugin, plugconfig);
                    handles.push(thread);
                },
                None => {
                    dbg!(name);
                }
            }
        }

        for handle in handles {
            match handle.join() {
                Ok(_) => {},
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
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
        return fs::read_dir(path).unwrap()
            .map(|x| x.unwrap()
            .file_name()
            .to_str().unwrap()
            .to_string()).collect::<Vec<String>>();
}

fn detect_language(internal_files: Vec<String>) -> PluginLanguage {
    if internal_files.iter().any(|x| x.ends_with(".so") || x.ends_with(".dylib")) {
        return PluginLanguage::RUST;
    }
    return PluginLanguage::PYTHON;
}


pub fn get_plug_list(config: &PluginConfig) -> Vec<Plugin> {
    let mut plugins = Vec::new();
    let path = unwrap_home_path(PLUGINS_PATH);

    match fs::read_dir(&path) {
        Ok(paths) =>
            for path in paths {
                let plugpath = path.unwrap().path();
                let name = plugpath.file_name().unwrap().to_str().unwrap().to_string();

                if !config.contains_key(&name) {
                    continue;
                }

                let language = detect_language(get_files_list(&plugpath));
                println!("Found plugin: {} with language: {:?}", name, language);
                plugins.push(Plugin { name, language });
            },
        Err(_) => {
            println!("No plugins found: No such directory: {:?}", &path);
            std::process::exit(1);
        }
    };

    return plugins;

}


#[derive(Debug)]
enum PluginLoadError {
    StructureError,
}

impl fmt::Display for PluginLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PluginLoadError::StructureError => write!(f, "Plugin structure error"),
        }
    }
}

impl error::Error for PluginLoadError {
    fn description(&self) -> &str {
        match *self {
            PluginLoadError::StructureError => "Plugin structure error",
        }
    }
}


fn load_python_plugin(name: &str, manifest: &mut PluginManifest) -> Result<(), PluginLoadError> {
    let plugins_path = unwrap_home_path(PLUGINS_PATH);
    let plugins_dirname = plugins_path.to_str().unwrap();

    let entrypoint_path = format!("{}/{}/main.py", &plugins_dirname, name);
    let path_to_main = Path::new(&entrypoint_path);

    if path_to_main.exists(){
        match fs::read_to_string(&path_to_main) {
            Ok(code) => {
                Python::with_gil(|py| {
                    let syspath: &PyList = py.import("sys").unwrap()
                        .getattr("path").unwrap()
                        .downcast::<PyList>().unwrap();

                    syspath.insert(0, &path_to_main).unwrap();

                    let app: Py<PyAny> = PyModule::from_code(py, &code, "", "").unwrap()
                        .getattr("run").unwrap()
                        .into();

                    manifest.register(name.to_string(), Box::new(app) as CallablePlugin);
                });
            },
            Err(_) => {
                println!("Could not read plugin code");
                return Err(PluginLoadError::StructureError);
            }
        }
    }
    else {
        println!("No main.py found in plugin: {}", name);
        return Err(PluginLoadError::StructureError);
    }

    return Ok(());

}


fn load_rust_plugin(name: &str, manifest: &mut PluginManifest) {
    let plugins_path = unwrap_home_path(PLUGINS_PATH);
    let plugins_dirname = plugins_path.to_str().unwrap();

    let lib = libloading::Library::new(format!("{}/{}/lib{}.dylib", &plugins_dirname, name, name))
        .expect("load library");
    let interface: libloading::Symbol<extern "Rust" fn() -> CallablePlugin> = unsafe { lib.get(b"new_service") }
    .expect("load symbol");

    manifest.register(name.to_string(), interface() as CallablePlugin);
}


pub fn load_plugins(plugins: Vec<Plugin>, manifest: &mut PluginManifest) {
    for plugin in plugins {
        match plugin.language {
            PluginLanguage::RUST => {
                load_rust_plugin(&plugin.name, manifest);
            },
            PluginLanguage::PYTHON => {
                match load_python_plugin(&plugin.name, manifest)
                {
                    Ok(_) => {},
                    Err(e) => println!("Error loading plugin: {}", e),
                }
            }
        }
    }
}



