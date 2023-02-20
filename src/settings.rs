use config::Config;
use std::collections::HashMap;

pub type PluginsConfig = Vec<HashMap<String, HashMap<String, String>>>;

pub trait FromConfig<T> {
    fn from_config(config: &HashMap<String, String>) -> Result<T, String>;
}


pub fn load_config(path: &str) -> PluginsConfig {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name(path))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    return settings
        .try_deserialize::<HashMap<String, PluginsConfig>>()
        .expect("Config file is not valid")
        .get("plugins")
        .expect("Plugins config not found")
        .to_vec();
}
