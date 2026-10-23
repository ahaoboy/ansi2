use crate::theme::{self, Rgb, Theme};

pub fn hex((r, g, b): Rgb) -> String {
    format!("#{r:02X}{g:02X}{b:02X}")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color8 {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

const COLOR8: [Color8; 16] = [
    Color8::Black,
    Color8::Red,
    Color8::Green,
    Color8::Yellow,
    Color8::Blue,
    Color8::Magenta,
    Color8::Cyan,
    Color8::White,
    Color8::BrightBlack,
    Color8::BrightRed,
    Color8::BrightGreen,
    Color8::BrightYellow,
    Color8::BrightBlue,
    Color8::BrightMagenta,
    Color8::BrightCyan,
    Color8::BrightWhite,
];

const FG_CLASSES: [&str; 16] = [
    "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "ca", "cb", "cc", "cd", "ce", "cf", "cg", "ch",
];

const BG_CLASSES: [&str; 16] = [
    "b0", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "ba", "bb", "bc", "bd", "be", "bf", "bg", "bh",
];

const FG_CODES: [u8; 16] = [
    30, 31, 32, 33, 34, 35, 36, 37, 90, 91, 92, 93, 94, 95, 96, 97,
];

impl Color8 {
    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn class(self) -> &'static str {
        FG_CLASSES[self.index()]
    }

    pub const fn bg_class(self) -> &'static str {
        BG_CLASSES[self.index()]
    }

    pub const fn to_u8(self) -> u8 {
        FG_CODES[self.index()]
    }

    pub fn hex(self, theme: Theme) -> String {
        hex(theme.color(self.index()))
    }

    pub fn from_u8(value: u8) -> Self {
        let index = match value {
            30..=37 => value - 30,
            40..=47 => value - 40,
            90..=97 => value - 90 + 8,
            100..=107 => value - 100 + 8,
            _ => 0,
        };
        COLOR8[index as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AnsiColor {
    #[default]
    Default,
    Color8(Color8),
    Color256(u8),
    Rgb(u8, u8, u8),
}

impl AnsiColor {
    pub fn class(&self) -> String {
        match self {
            Self::Default => "D".into(),
            Self::Color8(color) => color.class().into(),
            Self::Rgb(r, g, b) => format!("c{r:02X}{g:02X}{b:02X}"),
            Self::Color256(index) => format!("c{index:02X}"),
        }
    }

    pub fn bg_class(&self) -> String {
        match self {
            Self::Default => "D".into(),
            Self::Color8(color) => color.bg_class().into(),
            Self::Rgb(r, g, b) => format!("b{r:02X}{g:02X}{b:02X}"),
            Self::Color256(index) => format!("b{index:02X}"),
        }
    }

    pub fn hex(&self, theme: Theme) -> String {
        match self {
            Self::Default => "#00000000".into(),
            Self::Color8(color) => color.hex(theme),
            Self::Rgb(r, g, b) => hex((*r, *g, *b)),
            Self::Color256(index) => hex(theme::color256(*index)),
        }
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            30..=37 | 40..=47 | 90..=97 | 100..=107 => Self::Color8(Color8::from_u8(value)),
            _ => Self::Default,
        }
    }

    pub const fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }
}
