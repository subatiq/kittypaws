use crate::plug::{unwrap_home_path, CallablePlugin, PluginInterface, PLUGINS_PATH};
use paws_config::MonitoringOptions;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::{PyList, PyModule};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

impl PluginInterface for pyo3::Py<PyAny> {
    fn run(&self, config: &HashMap<String, String>) -> Result<(), String> {
        let mut pyconfig = HashMap::new();
        pyconfig.insert("config", &config);

        if let Err(err) = Python::with_gil(|py| self.call(py, (), Some(pyconfig.into_py_dict(py))))
        {
            return Err(format!("{}", err));
        }

        Ok(())
    }

    fn status(&self, _: &HashMap<String, String>, _: &Option<MonitoringOptions>) -> Result<telegraf::Point, String> {
        unimplemented!("Python plugins do not support status checks now")
    }
}

pub fn load(name: &str) -> Result<CallablePlugin, String> {
    let plugins_path = unwrap_home_path(PLUGINS_PATH);
    let plugins_dirname = plugins_path
        .to_str()
        .expect("Can't find home directory for the current user");

    let entrypoint_path = format!("{}/{}/main.py", &plugins_dirname, name);
    let path_to_main = Path::new(&entrypoint_path);

    if !path_to_main.exists() {
        println!();
        return Err(format!("No main.py found for plugin: {}", name));
    }
    match fs::read_to_string(path_to_main) {
        Ok(code) => {
            let app = Python::with_gil(|py| {
                let syspath: &PyList = py
                    .import("sys")
                    .expect("Python can't import sys module. Nobody knows why")
                    .getattr("path")
                    .unwrap()
                    .downcast::<PyList>()
                    .expect("Somehow sys.path is not a valid pyhon list");

                syspath
                    .insert(0, path_to_main)
                    .expect("Can't insert to Python path");

                let app: Py<PyAny> = PyModule::from_code(py, &code, "", "")
                    .unwrap_or_else(|_| panic!("Can't find main.py for plugin {}", name))
                    .getattr("run")
                    .unwrap_or_else(|_| panic!("Can't find run function in main.py for plugin {}", name))
                    .into();
                app
            });
            Ok(Box::new(app) as CallablePlugin)
        }
        Err(_) => Err("Could not read main.py code".to_string()),
    }
}
