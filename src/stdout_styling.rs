use chrono::{DateTime, Utc};
use std::ops::{Range, Sub};
use std::time::SystemTime;

trait Diff<T> {
    fn get_diff(&self) -> T;
}

impl<T: Sub<Output = T> + Copy> Diff<T> for Range<T> {
    fn get_diff(&self) -> T {
        self.end - self.start
    }
}

fn get_plugname_format(name: &str) -> String {
    format!("[{}]", name)
}

fn get_current_iso_time() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now = now.to_rfc3339();

    now.split_once('.').unwrap().0.to_string()
}

fn get_datetime_format() -> String {
    let current_time = get_current_iso_time();
    format!("({})", current_time)
}

pub fn style_line(plugname: String, message: String) -> String {
    let name_part = get_plugname_format(&plugname);
    let dt_part = get_datetime_format();
    let line = format!("{}\t{}\t{}", name_part, dt_part, message);
    line.to_string()
}
