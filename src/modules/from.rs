use crate::{colors, constants::Layout, utils};
use unicode_width::UnicodeWidthStr;

use super::Module;

pub struct From {
    label: &'static str,
    ok: bool,
    ip_ok: bool,
    port_ok: bool,
    ip: String,
    port: String,
}

impl From {
    pub fn new(ip: &str, port: &str) -> Self {
        Self {
            label: "FROM",
            ok: !utils::isunknown(ip) && !utils::isunknown(port),
            ip_ok: !utils::isunknown(ip),
            port_ok: !utils::isunknown(port),
            ip: ip.to_string(),
            port: port.to_string(),
        }
    }
}

impl Module for From {
    fn render(&self, gap: usize) -> String {
        let label = format!("{:<gap$}", self.label);

        format!(
            "{}{}{}:{}",
            utils::spaces(Layout::PADDING),
            colors::label(&label, self.ok),
            colors::info(&self.ip, self.ip_ok),
            colors::info(&self.port, self.port_ok),
        )
    }

    fn width(&self, gap: usize) -> usize {
        format!(
            "{}{:<gap$}{}:{}",
            utils::spaces(Layout::PADDING),
            self.label,
            self.ip,
            self.port,
        )
        .width()
    }

    fn label(&self) -> Option<&str> {
        Some(self.label)
    }
}
