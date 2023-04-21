mod plug;
mod settings;
mod stdout_styling;
mod intervals;

use settings::load_config;
use plug::start_main_loop;


fn main() {
    let binding = "127.0.0.1:9185".parse().unwrap();
    let exporter = prometheus_exporter::start(binding).unwrap();
    // get config path from args
    let config_path = std::env::args().nth(1).or(Some("config.yml".to_string())).unwrap();
    let config = load_config(&config_path);
    start_main_loop(&config)
}
