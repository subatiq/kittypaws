mod plug;
mod settings;
mod stdout_styling;
mod intervals;
mod plugin_logger;

use settings::load_config;
use plug::start_main_loop;


fn main() {
    // get config path from args
    let config_path = std::env::args().nth(1).or(Some("config.yml".to_string())).unwrap();
    let config = load_config(&config_path);
    start_main_loop(&config)
}
