use std::ops::{Range, Sub};
use chrono::{DateTime, Utc};
use std::time::SystemTime;

type Color = String;

static PALETTE_RANGE: Range<u8> = 69..219;

trait Diff<T> {
    fn get_diff(&self) -> T;
}

impl<T: Sub<Output = T> + Copy> Diff<T> for Range<T> {
    fn get_diff(&self) -> T
    {
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

    format!("{}", now.split_once(".").unwrap().0)
}

fn get_datetime_format() -> String {
    let current_time = get_current_iso_time();
    format!("({})", current_time)
}

fn color_line(line: &str, color: &Color) -> String {
    format!("\x1b[{}m{}\x1b[0m", color, line)
}

pub fn style_line(plugname: String, message: String) -> String {
    let name_part = get_plugname_format(&plugname);
    let dt_part = get_datetime_format();
    let line = format!("{}\t{}\t{}", name_part, dt_part, message);
    line.to_string()
}
