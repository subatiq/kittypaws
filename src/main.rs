mod intervals;
mod plug;
mod settings;
mod stdout_styling;

use plug::start_main_loop;
use paws_config::load_config;

fn main() {
    // get config path from args
    let config_path = std::env::args()
        .nth(1)
        .or(Some("paws.yml".to_string()))
        .unwrap();

    let config = load_config(&config_path);
    start_main_loop(config)
}
