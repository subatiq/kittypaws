type Color = String;

const palette: [&'static str; 16] = [
    "0;30",
    "0;31",
    "0;32",
    "0;33",
    "0;34",
    "0;35",
    "0;36",
    "0;37",
    "1;30",
    "1;31",
    "1;32",
    "1;33",
    "1;34",
    "1;35",
    "1;36",
    "1;37",
];

fn add_plugname(line: &str, name: &str) -> String {
    format!("[{}] {}", name, line)
}

fn color_line(line: &str, color: &Color) -> String {
    format!("{}{}\033[0m", color, line)
}

fn get_associated_color(plugname: &str) -> Color {
    todo!();
}

pub fn style_line(plugname: &str) {
    let color = get_associated_color(&plugname);
    todo!();
}
