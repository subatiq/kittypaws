use std::fmt::Display;
use std::sync::Mutex;
use std::collections::HashMap;
use std::ops::{Range, Sub};
use chrono::{DateTime, Utc};
use std::time::SystemTime;

use lazy_static::lazy_static;

type Color = String;

lazy_static! {
    static ref PLUG2COLOR: Mutex<HashMap<String, Color>> = Mutex::new(HashMap::new());
}

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

fn get_associated_color(plugname: &str) -> Color {
    let mut color_mapping = PLUG2COLOR.lock().unwrap();
    match color_mapping.get(plugname) {
        Some(color) => color.to_string(),
        None => {
            let ansi_code = PALETTE_RANGE.get_diff() - color_mapping.len() as u8 % PALETTE_RANGE.get_diff();
            let color = format!("38;5;{}", ansi_code);
            color_mapping.insert(plugname.to_string(), color.to_string());
            color.to_string()
        }
    }
}

pub fn style_line(plugname: &str, message: impl Display) -> String {
    let name_part = get_plugname_format(plugname);
    let dt_part = get_datetime_format();
    let line = format!("{}\t{}\t{}", name_part, dt_part, message);
    let line = color_line(&line, &get_associated_color(plugname));
    line.to_string()
}
