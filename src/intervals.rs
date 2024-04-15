use rand::Rng;
use std::thread;
use std::time::Duration;
use paws_config::FrequencyOptions;


pub fn wait_duration(duration: Duration) {
    thread::sleep(duration);
}

fn get_wait_time(frequency: &FrequencyOptions) -> Option<Duration> {
    match &frequency {
        FrequencyOptions::Once => None,
        FrequencyOptions::Fixed(duration) => Some(duration.as_std()),
        FrequencyOptions::Random(range) => Some(
            Duration::from_secs(
            rand::thread_rng()
                .gen_range(range.min.as_std().as_secs()..range.max.as_std().as_secs())
            )
        )
    }
}

pub fn time_till_next_run(frequency: &FrequencyOptions) -> Option<Duration> {
    match get_wait_time(frequency) {
        Some(duration) => {
            wait_duration(duration);
            Some(duration)
        },
        None => None
    }
}
