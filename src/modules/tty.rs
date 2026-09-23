use crate::colors;
use crate::constants::Layout;
use crate::utils::{spaces, unknown};

use super::Module;

use unicode_width::UnicodeWidthStr;

pub struct Tty {
    label: &'static str,
    ok: bool,
    name: String,
}

impl Tty {
    pub fn new(name: &str) -> Self {
        Self {
            label: "TTY",
            ok: unknown(name).is_none(),
            name: name.to_string(),
        }
    }
}

impl Module for Tty {
    fn render(&self, gap: usize) -> String {
        let label = format!("{:<gap$}", self.label);

        format!(
            "{}{}{}",
            spaces(Layout::PADDING),
            colors::label(&label, self.ok),
            colors::info(&self.name, self.ok)
        )
    }

    fn width(&self, gap: usize) -> usize {
        format!(
            "{}{:<gap$}{}",
            spaces(Layout::PADDING),
            self.label,
            self.name
        )
        .width()
    }

    fn label(&self) -> Option<&str> {
        Some(self.label)
    }
}
