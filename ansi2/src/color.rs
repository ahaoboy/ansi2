use crate::theme::{ColorTable, COLOR256};

pub fn get_hex((r, g, b): (u8, u8, u8)) -> String {
    format!("#{r:02X}{g:02X}{b:02X}")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Color8 {
    pub fn class_name(&self) -> &'static str {
        match self {
            Color8::Black => "c0",
            Color8::Red => "c1",
            Color8::Green => "c2",
            Color8::Yellow => "c3",
            Color8::Blue => "c4",
            Color8::Magenta => "c5",
            Color8::Cyan => "c6",
            Color8::White => "c7",
            Color8::BrightBlack => "ca",
            Color8::BrightRed => "cb",
            Color8::BrightGreen => "cc",
            Color8::BrightYellow => "cd",
            Color8::BrightBlue => "ce",
            Color8::BrightMagenta => "cf",
            Color8::BrightCyan => "cg",
            Color8::BrightWhite => "ch",
        }
    }
    pub fn bg_class_name(&self) -> &'static str {
        match self {
            Color8::Black => "b0",
            Color8::Red => "b1",
            Color8::Green => "b2",
            Color8::Yellow => "b3",
            Color8::Blue => "b4",
            Color8::Magenta => "b5",
            Color8::Cyan => "b6",
            Color8::White => "b7",
            Color8::BrightBlack => "ba",
            Color8::BrightRed => "bb",
            Color8::BrightGreen => "bc",
            Color8::BrightYellow => "bd",
            Color8::BrightBlue => "be",
            Color8::BrightMagenta => "bf",
            Color8::BrightCyan => "bg",
            Color8::BrightWhite => "bh",
        }
    }

    pub fn get_hex<T: ColorTable>(&self, theme: T) -> String {
        match self {
            Color8::Black => get_hex(theme.black()),
            Color8::Red => get_hex(theme.red()),
            Color8::Green => get_hex(theme.green()),
            Color8::Yellow => get_hex(theme.yellow()),
            Color8::Blue => get_hex(theme.blue()),
            Color8::Magenta => get_hex(theme.magenta()),
            Color8::Cyan => get_hex(theme.cyan()),
            Color8::White => get_hex(theme.white()),
            Color8::BrightBlack => get_hex(theme.bright_black()),
            Color8::BrightRed => get_hex(theme.bright_red()),
            Color8::BrightGreen => get_hex(theme.bright_green()),
            Color8::BrightYellow => get_hex(theme.bright_yellow()),
            Color8::BrightBlue => get_hex(theme.bright_blue()),
            Color8::BrightMagenta => get_hex(theme.bright_magenta()),
            Color8::BrightCyan => get_hex(theme.bright_cyan()),
            Color8::BrightWhite => get_hex(theme.bright_white()),
        }
    }

    pub fn from_u8(n: u8) -> Color8 {
        match n {
            30 | 40 => Color8::Black,
            31 | 41 => Color8::Red,
            32 | 42 => Color8::Green,
            33 | 43 => Color8::Yellow,
            34 | 44 => Color8::Blue,
            35 | 45 => Color8::Magenta,
            36 | 46 => Color8::Cyan,
            37 | 47 => Color8::White,

            90 | 100 => Color8::BrightBlack,
            91 | 101 => Color8::BrightRed,
            92 | 102 => Color8::BrightGreen,
            93 | 103 => Color8::BrightYellow,
            94 | 104 => Color8::BrightBlue,
            95 | 105 => Color8::BrightMagenta,
            96 | 106 => Color8::BrightCyan,
            97 | 107 => Color8::BrightWhite,
            _ => Color8::Black,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Color8::Black => 30,
            Color8::Red => 31,
            Color8::Green => 32,
            Color8::Yellow => 33,
            Color8::Blue => 34,
            Color8::Magenta => 35,
            Color8::Cyan => 36,
            Color8::White => 37,
            Color8::BrightBlack => 90,
            Color8::BrightRed => 91,
            Color8::BrightGreen => 92,
            Color8::BrightYellow => 93,
            Color8::BrightBlue => 94,
            Color8::BrightMagenta => 95,
            Color8::BrightCyan => 96,
            Color8::BrightWhite => 97,
        }
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
    pub fn class_name(&self) -> String {
        match self {
            AnsiColor::Default => "D".into(),
            AnsiColor::Color8(n) => n.class_name().to_string(),
            AnsiColor::Rgb(r, g, b) => format!("c{r:02X}{g:02X}{b:02X}"),
            AnsiColor::Color256(c) => format!("c{c:02X}"),
        }
    }

    pub fn bg_class_name(&self) -> String {
        match self {
            AnsiColor::Default => "D".into(),
            AnsiColor::Color8(n) => n.bg_class_name().to_string(),
            AnsiColor::Rgb(r, g, b) => format!("b{r:02X}{g:02X}{b:02X}"),
            AnsiColor::Color256(c) => format!("b{c:02X}"),
        }
    }

    pub fn get_hex<T: ColorTable>(&self, theme: T) -> String {
        match self {
            AnsiColor::Default => "#00000000".into(),
            AnsiColor::Color8(n) => n.get_hex(theme),
            AnsiColor::Rgb(r, g, b) => get_hex((*r, *g, *b)),
            AnsiColor::Color256(c) => get_hex(COLOR256[*c as usize]),
        }
    }

    pub fn from_u8(n: u8) -> AnsiColor {
        match n {
            30..=37 | 40..=47 | 90..=97 | 100..=107 => AnsiColor::Color8(Color8::from_u8(n)),
            _ => AnsiColor::Default,
        }
    }
    pub fn is_default(&self) -> bool {
        matches!(self, AnsiColor::Default)
    }
}
