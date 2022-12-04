use std::fs;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::{PyModule, PyList};

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
    plugins: HashMap<String, Box<dyn PluginInterface>>,
}

impl PluginManifest {
    pub fn from_discovered_plugins() -> PluginManifest {
        let mut manifest = PluginManifest {
            plugins: HashMap::new()
        };
        load_plugins(get_plug_list(), &mut manifest);
        manifest
    }

    pub fn register(&mut self, name: String, interface: Box<dyn PluginInterface>) {
        self.plugins.insert(name, interface);
    }

    pub fn run(&self, config: HashMap<String, HashMap<String, String>>) {
        for (name, plugin) in &self.plugins {
            match config.get(name) {
                Some(n) => plugin.run(&n),
                None => {}
            }
        }
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
    if internal_files.contains(&"Cargo.toml".to_string()) {
        return PluginLanguage::RUST;
    }
    return PluginLanguage::PYTHON;
}


pub fn get_plug_list() -> Vec<Plugin> {
    let mut plugins = Vec::new();
    let paths = fs::read_dir("./plugins").unwrap();
    for path in paths {
        let plugpath = path.unwrap().path();
        let name = plugpath.file_name().unwrap().to_str().unwrap().to_string();

        let language = detect_language(get_files_list(&plugpath));
        println!("Found plugin: {} with language: {:?}", name, language);
        plugins.push(Plugin { name, language });
    }
    plugins
}


pub fn load_plugins(plugins: Vec<Plugin>, manifest: &mut PluginManifest) {
    for plugin in plugins {
        match plugin.language {
            PluginLanguage::RUST => {
            let lib = libloading::Library::new(format!("target/debug/lib{}.dylib", plugin.name))
                .expect("load library");
            let interface: libloading::Symbol<extern "Rust" fn() -> Box<dyn PluginInterface>> = unsafe { lib.get(b"new_service") }
            .expect("load symbol");
            
            manifest.register(plugin.name, interface());
            },
            PluginLanguage::PYTHON => {
                let entrypoint_path = format!("plugins/{}/main.py", plugin.name);
                let pypath = Path::new(&entrypoint_path);
                let py_app = fs::read_to_string(&pypath).expect("read main.py");
                Python::with_gil(|py| {
                    let syspath: &PyList = py.import("sys").unwrap()
                        .getattr("path").unwrap()
                        .downcast::<PyList>().unwrap();
                    syspath.insert(0, &pypath).unwrap();
                    let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "").unwrap()
                        .getattr("run").unwrap()
                        .into();
                    manifest.register(plugin.name, Box::new(app));
                    // app.call0(py)
                });
            }
        }
    }
}



