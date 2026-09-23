use crate::colors;
use crate::constants::Layout;
use crate::utils::{spaces, unknown};

use super::Module;

use unicode_width::UnicodeWidthStr;

pub struct To {
    label: &'static str,
    ip_ok: bool,
    port_ok: bool,
    ip: String,
    port: String,
}

impl To {
    pub fn new(ip: &str, port: &str) -> Self {
        Self {
            label: "TO",
            ip_ok: unknown(ip).is_none(),
            port_ok: unknown(port).is_none(),
            ip: ip.to_string(),
            port: port.to_string(),
        }
    }
}

impl Module for To {
    fn render(&self, gap: usize) -> String {
        let label = format!("{:<gap$}", self.label);

        format!(
            "{}{}{}:{}",
            spaces(Layout::PADDING),
            colors::label(&label, self.ip_ok && self.port_ok),
            colors::info(&self.ip, self.ip_ok),
            colors::info(&self.port, self.port_ok)
        )
    }

    fn width(&self, gap: usize) -> usize {
        format!(
            "{}{:gap$}{}:{}",
            spaces(Layout::PADDING),
            self.label,
            self.ip,
            self.port
        )
        .width()
    }

    fn label(&self) -> Option<&str> {
        Some(self.label)
    }
}
