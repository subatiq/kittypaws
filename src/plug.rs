use std::fs;
use tokio::task;
use std::sync::mpsc::{channel, Sender, Receiver};
use tokio::task::JoinHandle;
use std::fmt;
use std::error;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::{PyModule, PyList};

const PLUGINS_PATH: &str = "~/.kittypaws/plugins";
type CallablePlugin = Box<dyn PluginInterface + Send>;

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
    fn clone_dyn(&self) -> CallablePlugin;
}


impl Clone for CallablePlugin {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}


impl PluginInterface for pyo3::Py<PyAny> {
    fn run(&self, config: &HashMap<String, String>) {
        let mut pyconfig = HashMap::new();
        pyconfig.insert("config", &config);
        Python::with_gil(|py| self.call(py, (), Some(pyconfig.into_py_dict(py)))).unwrap();
    }

    fn clone_dyn(&self) -> CallablePlugin {
        Box::new(self.clone())
    }
}


pub struct PluginManifest {
    plugins: HashMap<String, CallablePlugin>,
}



impl PluginManifest {
    pub fn from_discovered_plugins() -> PluginManifest {
        let mut manifest = PluginManifest {
            plugins: HashMap::new()
        };
        load_plugins(get_plug_list(), &mut manifest);
        manifest
    }

    pub fn register(&mut self, name: String, interface: CallablePlugin) {
        self.plugins.insert(name, interface);
    }

    pub async fn run(&self, config: &HashMap<String, HashMap<String, String>>) {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for (name, plugin) in self.plugins.iter() {
            match config.get(name) {
                Some(plugconfig) => {
                    let plugin: CallablePlugin = plugin.clone();
                    let plugconfig = plugconfig.clone();

                    handles.push(tokio::spawn(async move {
                        plugin.run(&plugconfig);
                    }));
                },
                None => {}
            }
        }

        for handle in handles {
            tokio::join!(handle);
        }
    }
}


fn unwrap_home_path(path: &str) -> PathBuf {
    if path.starts_with("~") {
        let home = std::env::var("HOME").unwrap();
        PathBuf::from(&path.replace("~", &home))
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


pub fn get_plug_list() -> Vec<Plugin> {
    let mut plugins = Vec::new();
    let path = unwrap_home_path(PLUGINS_PATH);

    match fs::read_dir(&path) {
        Ok(paths) =>
            for path in paths {
                let plugpath = path.unwrap().path();
                let name = plugpath.file_name().unwrap().to_str().unwrap().to_string();

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
        let py_app = fs::read_to_string(&path_to_main).unwrap();
        
        Python::with_gil(|py| {
            let syspath: &PyList = py.import("sys").unwrap()
                .getattr("path").unwrap()
                .downcast::<PyList>().unwrap();

            syspath.insert(0, &path_to_main).unwrap();

            let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "").unwrap()
                .getattr("run").unwrap()
                .into();

            manifest.register(name.to_string(), Box::new(app) as CallablePlugin);
        });
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
    let interface: libloading::Symbol<extern "Rust" fn() -> Box<dyn PluginInterface + Send>> = unsafe { lib.get(b"new_service") }
    .expect("load symbol");
    
    manifest.register(name.to_string(), interface());
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



