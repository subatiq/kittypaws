use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

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
            let plugconfig = config.get(name).unwrap();
            plugin.run(&plugconfig);
        }
    }
}


fn get_files_list(path: PathBuf) -> Vec<String> {
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

        let language = detect_language(get_files_list(plugpath));
        plugins.push(Plugin { name, language });
    }
    plugins
}


pub fn load_plugins(plugins: Vec<Plugin>, manifest: &mut PluginManifest) {
    for plugin in plugins {
        let lib = libloading::Library::new(format!("target/debug/lib{}.dylib", plugin.name))
            .expect("load library");
        match plugin.language {
            PluginLanguage::RUST => {
            let interface: libloading::Symbol<extern "Rust" fn() -> Box<dyn PluginInterface>> = unsafe { lib.get(b"new_service") }
            .expect("load symbol");
            
            manifest.register(plugin.name, interface());
            }
            _ => {}
        }
    }
}



