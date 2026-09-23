use crate::constants::Color;
use crate::types::RGB;

use owo_colors::OwoColorize;

pub fn status(text: &str, ok: bool) -> String {
    if ok {
        text.bright_green().bold().to_string()
    } else {
        text.bright_red().bold().to_string()
    }
}

pub fn message(text: &str, ok: bool) -> String {
    if ok {
        text.bright_white().bold().to_string()
    } else {
        text.red().bold().to_string()
    }
}

pub fn session(text: &str, ok: bool) -> String {
    if ok {
        text.white().bold().to_string()
    } else {
        text.red().bold().to_string()
    }
}

pub fn username(text: &str, ok: bool) -> String {
    if ok {
        let (r, g, b) = Color::ORANGE;
        text.truecolor(r, g, b).bold().to_string()
    } else {
        text.red().bold().to_string()
    }
}

pub fn hostname(text: &str, ok: bool) -> String {
    if ok {
        let (r, g, b) = Color::PINK;
        text.truecolor(r, g, b).bold().to_string()
    } else {
        text.red().bold().to_string()
    }
}

pub fn label(text: &str, ok: bool) -> String {
    if ok {
        text.bright_cyan().bold().to_string()
    } else {
        text.bright_red().bold().to_string()
    }
}

pub fn info(text: &str, ok: bool) -> String {
    if ok {
        text.to_string()
    } else {
        text.red().to_string()
    }
}

fn color_at(i: usize, len: usize, start: RGB, end: RGB) -> RGB {
    let t = if len <= 1 {
        0.0
    } else {
        i as f32 / (len - 1) as f32
    };

    let r = start.0 as f32 + (end.0 as f32 - start.0 as f32) * t;
    let g = start.1 as f32 + (end.1 as f32 - start.1 as f32) * t;
    let b = start.2 as f32 + (end.2 as f32 - start.2 as f32) * t;

    (r as u8, g as u8, b as u8)
}

pub fn gradient_char(ch: char, i: usize, len: usize, start: RGB, end: RGB) -> String {
    let (r, g, b) = color_at(i, len, start, end);

    ch.to_string().truecolor(r, g, b).to_string()
}

pub fn gradient_string(string: &str, start: RGB, end: RGB) -> String {
    let chars: Vec<char> = string.chars().collect();
    let len = chars.len();

    chars
        .iter()
        .enumerate()
        .map(|(i, ch)| {
            let (r, g, b) = color_at(i, len, start, end);

            ch.to_string().truecolor(r, g, b).to_string()
        })
        .collect()
}
