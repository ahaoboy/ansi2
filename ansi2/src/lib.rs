pub mod lex;
pub mod theme;

use lex::{parse_ansi, Token};
use theme::ColorTable;

#[derive(Debug, Clone)]
pub struct AnsiColor(pub u32);

impl AnsiColor {
    pub fn new(c: u32) -> Self {
        AnsiColor(c)
    }

    pub fn to_rgb(&self, th: impl ColorTable) -> String {
        match self.0 {
            30 | 40 => format!("rgb{:?}", th.black()),
            31 | 41 => format!("rgb{:?}", th.red()),
            32 | 42 => format!("rgb{:?}", th.green()),
            33 | 43 => format!("rgb{:?}", th.yellow()),
            34 | 44 => format!("rgb{:?}", th.blue()),
            35 | 45 => format!("rgb{:?}", th.magenta()),
            36 | 46 => format!("rgb{:?}", th.cyan()),
            37 | 47 => format!("rgb{:?}", th.white()),

            90 | 100 => format!("rgb{:?}", th.bright_black()),
            91 | 101 => format!("rgb{:?}", th.bright_red()),
            92 | 102 => format!("rgb{:?}", th.bright_green()),
            93 | 103 => format!("rgb{:?}", th.bright_yellow()),
            94 | 104 => format!("rgb{:?}", th.bright_blue()),
            95 | 105 => format!("rgb{:?}", th.bright_magenta()),
            96 | 106 => format!("rgb{:?}", th.bright_cyan()),
            97 | 107 => format!("rgb{:?}", th.bright_white()),
            _ => format!("rgb{:?}", th.white()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub bg_color: AnsiColor,
    pub color: AnsiColor,
    pub bold: bool,
    pub char: char,
}

#[derive(Debug, Clone)]
pub struct Canvas {
    pub pixels: Vec<Vec<Node>>,
    pub w: usize,
    pub h: usize,
}

fn set_node(v: &mut Vec<Vec<Node>>, node: Node, x: usize, y: usize) {
    while y >= v.len() {
        v.push(Vec::new());
    }

    let row = &mut v[y];
    while x >= row.len() {
        let empty = Node {
            bg_color: AnsiColor(0),
            color: AnsiColor(0),
            bold: false,
            char: ' ',
        };
        row.push(empty);
    }

    row[x] = node;
}

impl Canvas {
    pub fn new(s: &str) -> Self {
        let (_, lex) = parse_ansi(s).unwrap();
        let mut cur_x = 0;
        let mut cur_y = 0;
        let mut cur_c = 0;
        let mut cur_bg_c = 0;
        let mut bold = false;
        let mut w = 0;
        let mut h = 0;
        let mut pixels = Vec::new();

        for i in lex {
            match i {
                Token::LineFeed => {
                    cur_y += 1;
                    cur_x = 0;
                }
                Token::Char(c) => {
                    let node = Node {
                        char: c,
                        bg_color: AnsiColor::new(cur_bg_c),
                        color: AnsiColor::new(cur_c),
                        bold,
                    };
                    set_node(&mut pixels, node, cur_x, cur_y);
                    cur_x += 1;
                }
                Token::ColorBackground(c) => cur_bg_c = c,
                Token::ColorForeground(c) => cur_c = c,
                Token::Bold => bold = true,
                Token::ColorReset => {
                    bold = false;
                    cur_c = 0;
                    cur_bg_c = 0;
                }
                Token::CursorUp(c) => cur_y = cur_y.saturating_sub(c as usize),
                Token::CursorDown(c) => {
                    cur_y += c as usize;
                }
                Token::CursorBack(c) => cur_x = cur_x.saturating_sub(c as usize),
                Token::CursorForward(c) => {
                    cur_x += c as usize;
                }
                Token::Backspace => cur_x = cur_x.saturating_sub(1),
                Token::Tab => {
                    let tail = cur_x & 7;
                    if tail == 0 {
                        cur_x += 8
                    } else {
                        cur_x += 8 - tail;
                    }
                }

                Token::CarriageReturn => cur_x = 0,

                Token::CursorNextLine(n) => {
                    cur_y += n as usize;
                    cur_x = 0;
                }
                Token::CursorPreviousLine(n) => {
                    cur_y = cur_y.saturating_sub(n as usize);
                    cur_x = 0;
                }
                Token::CursorHorizontalAbsolute(n) => cur_y = n as usize,
                Token::CursorPosition(x, y) => {
                    cur_x = x as usize;
                    cur_y = y as usize;
                }

                _ => {}
            }

            w = w.max(cur_x + 1);
            h = h.max(cur_y + 1);
        }

        Canvas { pixels, w, h }
    }
}
