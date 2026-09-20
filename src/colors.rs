use crate::{constants::Color, types::Rgb};
use colored::Colorize;
// use owo_colors::OwoColorize;

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

pub fn gradient(text: &str, start: Rgb, end: Rgb) -> String {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    chars
        .iter()
        .enumerate()
        .map(|(i, ch)| {
            let t = if len <=1 {
                0.0
            } else {
                i as f32 / (len - 1) as f32
            };

            let r = start.0 as f32 + (end.0 as f32 - start.0 as f32) * t;
            let g = start.1 as f32 + (end.1 as f32 - start.1 as f32) * t;
            let b = start.2 as f32 + (end.2 as f32 - start.2 as f32) * t;

            ch.to_string()
                .truecolor(r as u8, g as u8, b as u8)
                .to_string()
        })
    .collect()
}
