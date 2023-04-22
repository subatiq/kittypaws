mod plug;
mod settings;
mod stdout_styling;
mod intervals;

use settings::load_config;
use plug::start_main_loop;


fn main() {
    // parse args
    let config_path = std::env::args().nth(1).or(Some("config.yml".to_string())).unwrap();
    let bind_string = std::env::args().nth(2).or(Some("127.0.0.1:9185".to_string())).unwrap();

    // init prometheus client
    let binding = bind_string.as_str().parse().unwrap();
    let _exporter = prometheus_exporter::start(binding).unwrap();

    let config = load_config(&config_path);
    start_main_loop(&config)
}
