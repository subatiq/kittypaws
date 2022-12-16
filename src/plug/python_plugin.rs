use std::fs;
use std::path::Path;
use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::{PyModule, PyList};
use crate::plug::{unwrap_home_path, PluginInterface, CallablePlugin, PluginLoadError, PLUGINS_PATH};


impl PluginInterface for pyo3::Py<PyAny> {
    fn run(&self, config: &HashMap<String, String>) {
        let mut pyconfig = HashMap::new();
        pyconfig.insert("config", &config);
        Python::with_gil(|py| self.call(py, (), Some(pyconfig.into_py_dict(py)))).unwrap();
    }

}

pub fn load(name: &str) -> Result<CallablePlugin, PluginLoadError> {
    let plugins_path = unwrap_home_path(PLUGINS_PATH);
    let plugins_dirname = plugins_path.to_str().unwrap();

    let entrypoint_path = format!("{}/{}/main.py", &plugins_dirname, name);
    let path_to_main = Path::new(&entrypoint_path);

    if path_to_main.exists(){
        match fs::read_to_string(&path_to_main) {
            Ok(code) => {
                let app = Python::with_gil(|py| {
                    let syspath: &PyList = py.import("sys").unwrap()
                        .getattr("path").unwrap()
                        .downcast::<PyList>().unwrap();

                    syspath.insert(0, &path_to_main).unwrap();

                    let app: Py<PyAny> = PyModule::from_code(py, &code, "", "").unwrap()
                        .getattr("run").unwrap()
                        .into();
                    return app;

                });
                return Ok(Box::new(app) as CallablePlugin);
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

}

