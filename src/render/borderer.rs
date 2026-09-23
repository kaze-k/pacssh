use crate::colors;
use crate::constants::{Border, Color, Icon, Layout};
use crate::utils::spaces;
use unicode_width::UnicodeWidthStr;

pub struct Borderer;

impl Borderer {
    pub fn new() -> Self {
        Self
    }

    pub fn top(&self, width: usize) -> String {
        let width = width + Layout::PADDING
            - Icon::PACMAN.to_string().width()
            - Border::HORIZONTAL.to_string().repeat(2).width()
            - 2;
        let border = format!(
            "{}{} {} {}{}",
            Border::TOP_LEFT,
            Border::HORIZONTAL.to_string().repeat(2),
            Icon::PACMAN,
            Border::HORIZONTAL.to_string().repeat(width),
            Border::TOP_RIGHT,
        );

        colors::gradient_string(&border, Color::BLUE, Color::CYAN)
    }

    pub fn bottom(&self, width: usize) -> String {
        let width = width + Layout::PADDING
            - Icon::GHOST.to_string().width()
            - Border::HORIZONTAL.to_string().repeat(2).width()
            - 2;
        let border = format!(
            "{}{} {} {}{}",
            Border::BOTTOM_LEFT,
            Border::HORIZONTAL.to_string().repeat(width),
            Icon::GHOST,
            Border::HORIZONTAL.to_string().repeat(2),
            Border::BOTTOM_RIGHT,
        );

        colors::gradient_string(&border, Color::ORANGE, Color::PINK)
    }

    pub fn line(&self, content: &str, width: usize, index: usize, height: usize) -> String {
        format!(
            "{}{}{}{}",
            colors::gradient_char(Border::VERTICAL, index, height, Color::BLUE, Color::ORANGE),
            content,
            spaces(width),
            colors::gradient_char(Border::VERTICAL, index, height, Color::CYAN, Color::PINK)
        )
    }
}
