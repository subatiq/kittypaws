use rand::Rng;
use std::thread;
use std::ops::Range;
use std::time::Duration;
use std::collections::HashMap;
use crate::settings::FromConfig;
use iso8601_duration::Duration as ISODuration;


#[derive(Debug)]
pub enum Frequency {
    Fixed(Duration),
    Random(Range<Duration>),
    Once
}



impl FromConfig<Frequency> for Frequency {
    fn from_config(config: &HashMap<String, String>) -> Result<Frequency, String> {
        if !config.contains_key("frequency") {
            return Ok(Frequency::Once);
        }

        match config.get("frequency").unwrap().to_lowercase().as_str() {
            "once" => Ok(Frequency::Once),
            "fixed" => {
                if let Some(interval) = config.get("interval") {
                    let interval = ISODuration::parse(interval).expect("interval has ISO8601 format").to_std();
                    return Ok(Frequency::Fixed(interval));
                }
                Err("Can't find interval in config".to_string())

            },
            "random" => {
                let min_interval = config.get("min_interval").expect("Config has min_interval");
                let max_interval = config.get("max_interval").expect("Config has max_interval");
                let min_duration = ISODuration::parse(min_interval).expect("min_interval has ISO8601 format").to_std();
                let max_duration = ISODuration::parse(max_interval).expect("min_interval has ISO8601 format").to_std();

                if min_duration > max_duration {
                    return Err(format!("min_interval {} should be less or equal than max_interval {}", &min_interval, &min_interval));
                }


                Ok(Frequency::Random(min_duration..max_duration))
            }
            _ => Ok(Frequency::Once)
        }
    }
}



pub fn wait_duration(duration: Duration) {
    thread::sleep(duration);
}

fn get_wait_time(frequency: &Frequency) -> Option<Duration> {
    match &frequency {
        Frequency::Once => None,
        Frequency::Fixed(duration) => Some(*duration),
        Frequency::Random(range) => Some(
            Duration::from_secs(
            rand::thread_rng()
                .gen_range(range.start.as_secs()..range.end.as_secs())
            )
        )
    }

}

pub fn wait_for_next_run(frequency: &Frequency) {
    match get_wait_time(&frequency) {
        Some(duration) => wait_duration(duration),
        None => {}
    }
}
