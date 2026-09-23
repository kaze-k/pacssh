use crate::constants::Layout;
use crate::modules::Module;
use crate::utils::spaces;

use super::borderer::Borderer;
use unicode_width::UnicodeWidthStr;

pub struct Renderer {
    borderer: Borderer,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            borderer: Borderer::new(),
        }
    }

    pub fn render(&self, lines: &Vec<Box<dyn Module>>) {
        let gap = self.gap(lines);
        let width = self.width(lines, gap);

        println!("\n");

        println!("{}{}", spaces(Layout::INDENT), self.borderer.top(width));

        for (index, line) in lines.iter().enumerate() {
            let line_width = line.width(gap);
            let width = width.saturating_sub(line_width) + Layout::PADDING;
            println!(
                "{}{}",
                spaces(Layout::INDENT),
                self.borderer
                    .line(&line.render(gap), width, index, lines.len())
            );
        }

        println!("{}{}", spaces(Layout::INDENT), self.borderer.bottom(width));

        println!("\n");
    }

    fn width(&self, lines: &Vec<Box<dyn Module>>, gap: usize) -> usize {
        lines.iter().map(|line| line.width(gap)).max().unwrap_or(0)
    }

    fn gap(&self, lines: &Vec<Box<dyn Module>>) -> usize {
        let max_gap = lines
            .iter()
            .map(|line| line.label().unwrap_or_default().width())
            .max()
            .unwrap_or(0);

        max_gap + Layout::STATUS_GAP
    }
}
