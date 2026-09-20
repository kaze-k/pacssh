use crate::types::Rgb;

pub struct Layout;
impl Layout {
    pub const INDENT: usize = 4;
    pub const PADDING: usize = 4;

    pub const STATUS_GAP: usize = 2;
}

pub struct Icon;
impl Icon {
    pub const PACMAN: &str = "󰮯";
    pub const GHOST: &str = "󰊠";
}

pub struct Color;
impl Color {
    pub const START: Rgb = (96, 165, 250);
    pub const END: Rgb = (217, 70, 239);

    pub const ORANGE: Rgb = (255, 165, 0);
    pub const PINK: Rgb = (255, 105, 180);
}

pub struct Border;
impl Border {
    pub const TOP_LEFT: &str = "╭";
    pub const TOP_RIGHT: &str = "╮";
    pub const BOTTOM_RIGHT: &str = "╯";
    pub const BOTTOM_LEFT: &str = "╰";

    pub const HORIZONTAL: &str = "─";
    pub const VERTICAL: &str = "│";
}
