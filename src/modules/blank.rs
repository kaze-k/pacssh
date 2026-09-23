use super::Module;

use unicode_width::UnicodeWidthStr;

#[derive(Clone, Copy)]
pub struct Blank;

impl Blank {
    pub fn new() -> Self {
        Self
    }
}

impl Module for Blank {
    fn render(&self, _gap: usize) -> String {
        String::new()
    }

    fn width(&self, _gap: usize) -> usize {
        String::new().width()
    }

    fn label(&self) -> Option<&str> {
        None
    }
}
