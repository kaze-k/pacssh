use crate::colors;
use crate::constants::Layout;
use crate::utils::spaces;
use unicode_width::UnicodeWidthStr;

use super::Module;

pub struct Login {
    label: &'static str,
    is_ssh: bool,
}

impl Login {
    pub fn new(is_ssh: bool) -> Self {
        let label = if is_ssh { "[ok]" } else { "[fail]" };

        Self { label, is_ssh }
    }
}

impl Module for Login {
    fn render(&self, gap: usize) -> String {
        let message = if self.is_ssh {
            "SSH LOGIN SUCCESSFUL"
        } else {
            "SSH LOGIN FAILED"
        };

        let label = format!("{:<gap$}", self.label);

        format!(
            "{}{}{}",
            spaces(Layout::PADDING),
            colors::status(&label, self.is_ssh),
            colors::message(message, self.is_ssh)
        )
    }

    fn width(&self, gap: usize) -> usize {
        let (status, message) = if self.is_ssh {
            ("[ok]", "SSH LOGIN SUCCESSFUL")
        } else {
            ("[fail]", "SSH LOGIN FAILED")
        };

        format!("{}{:<gap$}{}", spaces(Layout::PADDING), status, message).width()
    }

    fn label(&self) -> Option<&str> {
        Some(self.label)
    }
}
