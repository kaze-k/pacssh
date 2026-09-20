use crate::{colors, constants::Layout, utils};
use unicode_width::UnicodeWidthStr;

use super::Module;

pub struct Session {
    label: &'static str,
    is_valid: bool,
    username_valid: bool,
    hostname_valid: bool,
    username: String,
    hostname: String,
}

impl Session {
    pub fn new(username: &str, hostname: &str) -> Self {
        let is_valid = !utils::isunknown(username) && !utils::isunknown(hostname);

        Self {
            label: if is_valid { "[ok]" } else { "[fail]" },
            is_valid: is_valid,
            username_valid: !utils::isunknown(username),
            hostname_valid: !utils::isunknown(hostname),
            username: username.to_string(),
            hostname: hostname.to_string(),
        }
    }
}

impl Module for Session {
    fn render(&self, gap: usize) -> String {
        let label = format!("{:<gap$}", self.label);

        format!(
            "{}{}{} {}@{}",
            utils::spaces(Layout::PADDING),
            colors::status(&label, self.is_valid),
            colors::session("SESSION", self.is_valid),
            colors::username(&self.username, self.username_valid),
            colors::hostname(&self.hostname, self.hostname_valid)
        )
    }

    fn width(&self, gap: usize) -> usize {
        format!(
            "{}{:<gap$}{} {}@{}",
            utils::spaces(Layout::PADDING),
            self.label,
            "SESSION",
            self.username,
            self.hostname
        )
        .width()
    }

    fn label(&self) -> Option<&str> {
        Some(self.label)
    }
}
