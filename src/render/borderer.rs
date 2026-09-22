use crate::colors;
use crate::constants;
use crate::utils::spaces;
use unicode_width::UnicodeWidthStr;

pub struct Borderer;

impl Borderer {
    pub fn new() -> Self {
        Self
    }

    pub fn top(&self, width: usize) -> String {
        let width = width + constants::Layout::PADDING
            - constants::Icon::PACMAN.width()
            - constants::Border::HORIZONTAL.repeat(2).width()
            - 2;
        let border = format!(
            "{}{} {} {}{}",
            constants::Border::TOP_LEFT,
            constants::Border::HORIZONTAL.repeat(2),
            constants::Icon::PACMAN,
            constants::Border::HORIZONTAL.repeat(width),
            constants::Border::TOP_RIGHT,
        );

        colors::gradient(&border, constants::Color::START, constants::Color::END)
    }

    pub fn bottom(&self, width: usize) -> String {
        let width = width + constants::Layout::PADDING
            - constants::Icon::GHOST.width()
            - constants::Border::HORIZONTAL.repeat(2).width()
            - 2;
        let border = format!(
            "{}{} {} {}{}",
            constants::Border::BOTTOM_LEFT,
            constants::Border::HORIZONTAL.repeat(width),
            constants::Icon::GHOST,
            constants::Border::HORIZONTAL.repeat(2),
            constants::Border::BOTTOM_RIGHT,
        );

        colors::gradient(&border, constants::Color::END, constants::Color::START)
    }

    pub fn line(&self, content: &str, width: usize) -> String {
        format!(
            "{}{}{}{}",
            colors::gradient(
                constants::Border::VERTICAL,
                constants::Color::START,
                constants::Color::END
            ),
            content,
            spaces(width),
            colors::gradient(
                constants::Border::VERTICAL,
                constants::Color::END,
                constants::Color::START
            )
        )
    }
}
