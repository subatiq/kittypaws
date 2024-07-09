use crate::plug::{unwrap_home_path, CallablePlugin, PluginInterface, PLUGINS_PATH};
use indexmap::IndexMap;
use rustpython::vm::{self, PyRef, PyObjectRef};
use rustpython::vm::builtins::{PyStr, PyStrRef};
use rustpython::vm::function::{IntoFuncArgs, PosArgs, KwArgs, FuncArgs, ArgMapping};
use std::collections::HashMap;
use std::path::Path;

pub struct PythonPluginScript {
    name: String,
}

struct ConfigArg(HashMap<String, String>);

impl Into<KwArgs> for ConfigArg {
    fn into(self) -> KwArgs {
        let mut map = IndexMap::new();
        for (key, value) in self.0 {
            let value = PyStr::from(value);
            map.insert(key, value);
        }

        KwArgs::new(map)
    }
}

impl PluginInterface for PythonPluginScript {
    fn run(&self, config: &HashMap<String, String>) -> Result<(), String> {
        let mut settings = vm::Settings::default();
        settings.debug = true;
        settings
            .path_list
            .push(unwrap_home_path(PLUGINS_PATH).join(&self.name).to_str().unwrap().to_string());
        let interpreter = rustpython::InterpreterConfig::new()
            .settings(settings)
            .init_stdlib()
            .interpreter();


        interpreter.enter(|vm| {
            let module = vm.import("main", 0).unwrap();
            module.get_attr("run", vm).unwrap().call(config.into(), vm);

        });

        Ok(())
    }

    fn stop(&self, config: &HashMap<String, String>) -> Result<(), String> {
        Ok(())
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

    Ok(Box::new(PythonPluginScript {
        name: name.to_string(),
    }) as CallablePlugin)
}
