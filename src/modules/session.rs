use crate::colors;
use crate::constants::Layout;
use crate::utils::{spaces, unknown};

use super::Module;

use unicode_width::UnicodeWidthStr;

pub struct Session {
    label: &'static str,
    username_ok: bool,
    hostname_ok: bool,
    username: String,
    hostname: String,
}

impl Session {
    pub fn new(username: &str, hostname: &str) -> Self {
        Self {
            label: if unknown(username).is_none() && unknown(hostname).is_none() {
                "[ok]"
            } else {
                "[fail]"
            },
            username_ok: unknown(username).is_none(),
            hostname_ok: unknown(hostname).is_none(),
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
            spaces(Layout::PADDING),
            colors::status(&label, self.username_ok && self.hostname_ok),
            colors::session("SESSION", self.username_ok && self.hostname_ok),
            colors::username(&self.username, self.username_ok),
            colors::hostname(&self.hostname, self.hostname_ok)
        )
    }

    fn width(&self, gap: usize) -> usize {
        format!(
            "{}{:<gap$}{} {}@{}",
            spaces(Layout::PADDING),
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
