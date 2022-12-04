mod plug;
mod settings;
use settings::load_config;
use plug::*;


fn main() {
    let manifest = PluginManifest::from_discovered_plugins();
    let config = load_config("configs/restarting.yml");
    println!("{:?}", config);
    manifest.run(config);
}
