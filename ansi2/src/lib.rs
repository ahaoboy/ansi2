pub mod html;
pub mod lex;
pub mod svg;
pub mod theme;
use lex::{parse_ansi, Token};
use theme::ColorTable;

#[derive(Debug, Clone)]
pub struct AnsiColor(pub u32);

impl AnsiColor {
    pub fn new(c: u32) -> Self {
        AnsiColor(c)
    }

    pub fn name(&self) -> String {
        match self.0 {
            30 | 40 => "black".into(),
            31 | 41 => "red".into(),
            32 | 42 => "green".into(),
            33 | 43 => "yellow".into(),
            34 | 44 => "blue".into(),
            35 | 45 => "magenta".into(),
            36 | 46 => "cyan".into(),
            37 | 47 => "white".into(),

            90 | 100 => "bright_black".into(),
            91 | 101 => "bright_red".into(),
            92 | 102 => "bright_green".into(),
            93 | 103 => "bright_yellow".into(),
            94 | 104 => "bright_blue".into(),
            95 | 105 => "bright_magenta".into(),
            96 | 106 => "bright_cyan".into(),
            97 | 107 => "bright_white".into(),
            _ => "white".into(),
        }
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
    pub blink: bool,
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
            blink: false,
        };
        row.push(empty);
    }

    row[x] = node;
}

impl Canvas {
    pub fn new(s: &str, max_width: Option<usize>) -> Self {
        let (_, lex) = parse_ansi(s).unwrap();
        let mut cur_x = 0;
        let mut cur_y = 0;
        let mut cur_c = 0;
        let mut cur_bg_c = 0;
        let mut bold = false;
        let mut blink = false;
        let mut blink_c = 0;
        let mut w = 0;
        let mut h = 0;
        let mut pixels = Vec::new();
        let max_width = max_width.unwrap_or(std::usize::MAX);

        for i in lex {
            let mut reset_all = || {
                bold = false;
                cur_bg_c = 0;
                cur_c = 0;
                blink = false;
                blink_c = 0;
            };

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
                        blink,
                    };
                    if cur_x >= max_width {
                        cur_x = 0;
                        cur_y += 1;
                    }
                    set_node(&mut pixels, node, cur_x, cur_y);
                    cur_x += 1;
                }
                Token::ColorBackground(c) => cur_bg_c = c,
                Token::ColorForeground(c) => cur_c = c,
                Token::Bold => bold = true,
                Token::ColorReset => {
                    reset_all();
                }
                Token::CursorUp(c) => cur_y = cur_y.saturating_sub(c as usize),
                Token::CursorDown(c) => {
                    cur_y += c as usize;
                }
                Token::CursorBack(c) => cur_x = cur_x.saturating_sub(c as usize),
                Token::CursorForward(c) => {
                    cur_x += c as usize;
                    if cur_x >= max_width {
                        cur_x %= max_width;
                        cur_y += 1;
                    }
                }
                Token::Backspace => cur_x = cur_x.saturating_sub(1),
                Token::Tab => {
                    let tail = cur_x & 7;
                    if tail == 0 {
                        cur_x += 8
                    } else {
                        cur_x += 8 - tail;
                    }

                    if cur_x >= max_width {
                        cur_x %= max_width;
                        cur_y += 1;
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

                Token::Sgr2(ctrl, background) => {
                    match ctrl {
                        0 => reset_all(),
                        1 => bold = true,
                        5 => blink = true,
                        _ => {}
                    }
                    match background {
                        30..=37 | 90..=97 => cur_c = background,
                        40..=47 | 100..=107 => cur_bg_c = background,
                        _ => {}
                    }
                }
                Token::Sgr3(ctrl, front, background) => {
                    match ctrl {
                        0 => reset_all(),
                        1 => bold = true,
                        5 => blink = true,
                        _ => {}
                    }
                    cur_c = front;
                    cur_bg_c = background;
                }
                Token::Sgr4(reset, ctrl, front, background) => {
                    if reset == 0 {
                        reset_all();
                    }
                    match ctrl {
                        0 => reset_all(),
                        1 => {
                            bold = true;
                            cur_c = front;
                            cur_bg_c = background;
                        }
                        5 => {
                            blink = true;
                            cur_bg_c = front;
                            blink_c = background;
                        }
                        _ => {
                            cur_c = front;
                            cur_bg_c = background;
                        }
                    }
                }

                Token::SlowBlink => blink = true,
                Token::RapidBlink => blink = true,
                _ => {}
            }

            w = w.max(cur_x + 1);
            h = h.max(cur_y + 1);
        }

        Canvas { pixels, w, h }
    }
}

#[cfg(test)]
mod test {
    use crate::lex::parse_ansi;

    #[test]
    fn test() {
        let s = "[0;5;35;45m";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }
}
