use crate::color::AnsiColor;

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub bg_color: AnsiColor,
    pub color: AnsiColor,
    pub bold: bool,
    pub blink: bool,
    pub text: String,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub hide: bool,
    pub strike: bool,

    // sourcemap
    pub bg_color_r: (usize, usize),
    pub color_r: (usize, usize),
    pub bold_r: (usize, usize),
    pub blink_r: (usize, usize),
    pub text_r: (usize, usize),
    pub dim_r: (usize, usize),
    pub italic_r: (usize, usize),
    pub underline_r: (usize, usize),
    pub hide_r: (usize, usize),
    pub strike_r: (usize, usize),
}

// ignore sourcemap fields
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.bg_color == other.bg_color
            && self.color == other.color
            && self.bold == other.bold
            && self.blink == other.blink
            && self.text == other.text
            && self.dim == other.dim
            && self.italic == other.italic
            && self.underline == other.underline
            && self.hide == other.hide
            && self.strike == other.strike
    }
}

impl Eq for Node {}

impl Node {
    pub fn same_style(&self, other: &Node) -> bool {
        self.bg_color == other.bg_color
            && self.color == other.color
            && self.bold == other.bold
            && self.blink == other.blink
            && self.dim == other.dim
            && self.italic == other.italic
            && self.underline == other.underline
            && self.hide == other.hide
            && self.strike == other.strike
    }
}
