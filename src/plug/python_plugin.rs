use std::fs;
use std::fmt::Display;
use std::path::Path;
use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::{PyModule, PyList};
use crate::plug::{unwrap_home_path, PluginInterface, CallablePlugin, PLUGINS_PATH};


impl PluginInterface for pyo3::Py<PyAny> {
    fn run(&self, name: &str, config: &HashMap<String, String>) -> Result<Box<dyn Display>, String> {
        let mut pyconfig = HashMap::new();
        pyconfig.insert("config", &config);
        let result = Python::with_gil(|py| {
            self.call(py, (), Some(pyconfig.into_py_dict(py)))
        });
        match result {
            Ok(_) => Ok(Box::new(format!("{} finished execution", &name))),
            Err(err) => Err(format!("{}", err))
        }
    }
}

pub fn load(name: &str) -> Result<CallablePlugin, String> {
    let plugins_path = unwrap_home_path(PLUGINS_PATH);
    let plugins_dirname = plugins_path.to_str().expect("Can't find home directory for the current user");

    let entrypoint_path = format!("{}/{}/main.py", &plugins_dirname, name);
    let path_to_main = Path::new(&entrypoint_path);

    if !path_to_main.exists() {
        println!();
        return Err(format!("No main.py found for plugin: {}", name));
    }
    match fs::read_to_string(&path_to_main) {
        Ok(code) => {
            let app = Python::with_gil(|py| {
                let syspath: &PyList = py.import("sys").expect("Python can't import sys module. Nobody knows why")
                    .getattr("path").unwrap()
                    .downcast::<PyList>().expect("Somehow sys.path is not a valid pyhon list");

                syspath.insert(0, &path_to_main).expect("Can't insert to Python path");

                let app: Py<PyAny> = PyModule::from_code(py, &code, "", "")
                    .expect(&format!("Can't find main.py for plugin {}", name))
                    .getattr("run")
                    .expect(&format!("Can't find run function in main.py for plugin {}", name))
                    .into();
                return app;

            });
            return Ok(Box::new(app) as CallablePlugin);
        },
        Err(_) => {
            return Err("Could not read main.py code".to_string());
        }
    }
}

