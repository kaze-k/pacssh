use crate::{colors, constants::Layout, utils};
use unicode_width::UnicodeWidthStr;

use super::Module;

pub struct Tty {
    label: &'static str,
    ok: bool,
    name: String,
}

impl Tty {
    pub fn new(name: &str) -> Self {
        Self {
            label: "TTY",
            ok: !utils::isunknown(name),
            name: name.to_string(),
        }
    }
}

impl Module for Tty {
    fn render(&self, gap: usize) -> String {
        let label = format!("{:<gap$}", self.label);

        format!(
            "{}{}{}",
            utils::spaces(Layout::PADDING),
            colors::label(&label, self.ok),
            colors::info(&self.name, self.ok)
        )
    }

    fn width(&self, gap: usize) -> usize {
        format!(
            "{}{:<gap$}{}",
            utils::spaces(Layout::PADDING),
            self.label,
            self.name
        )
        .width()
    }

    fn label(&self) -> Option<&str> {
        Some(self.label)
    }
}
