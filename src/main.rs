mod plug;
mod settings;
use settings::load_config;
use plug::*;


fn main() {
    // get config path from args
    let config_path = std::env::args().nth(1).or(Some("config.yml".to_string())).unwrap();
    let config = load_config(&config_path);
    let mut manifest = PluginManifest::load_plugins(&config);
    manifest.run(&config);
}
