use std::io::{self, IoSlice, Write};

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
        let indent = spaces(Layout::INDENT);
        let mut contents = Vec::with_capacity(lines.len() + 2);

        contents.push(format!("{}{}", indent, self.borderer.top(width)));

        for (index, line) in lines.iter().enumerate() {
            let line_width = line.width(gap);
            let width = width.saturating_sub(line_width) + Layout::PADDING;
            contents.push(format!(
                "{}{}",
                indent,
                self.borderer
                    .line(&line.render(gap), width, index, lines.len())
            ));
        }

        contents.push(format!("{}{}", indent, self.borderer.bottom(width)));

        self.output(contents)
            .expect("failed to write contents to stdout");
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

    fn output(&self, mut contents: Vec<String>) -> io::Result<()> {
        let mut buffers: Vec<IoSlice> = vec![];

        buffers.push(IoSlice::new("\n".as_bytes()));
        buffers.push(IoSlice::new("\n".as_bytes()));

        for content in &mut contents {
            content.push('\n');
            buffers.push(IoSlice::new(content.as_bytes()));
        }

        buffers.push(IoSlice::new("\n".as_bytes()));
        buffers.push(IoSlice::new("\n".as_bytes()));

        let mut stdout = io::stdout();

        stdout.write_vectored(&buffers)?;
        stdout.flush()?;

        Ok(())
    }
}
