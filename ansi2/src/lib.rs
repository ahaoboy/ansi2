pub mod html;
pub mod lex;
pub mod svg;
pub mod text;
pub mod theme;
use lex::{parse_ansi, AnsiColor, Token};

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
            bg_color: AnsiColor::Color8(0),
            color: AnsiColor::Color8(0),
            bold: false,
            char: ' ',
            blink: false,
        };
        row.push(empty);
    }

    row[x] = node;
}

impl Canvas {
    pub fn new<S: AsRef<str>>(str: S, max_width: Option<usize>) -> Self {
        let s = str.as_ref();
        let (_, lex) = parse_ansi(s).unwrap();
        let mut cur_x = 0;
        let mut cur_y = 0;
        let mut cur_c = AnsiColor::Color8(0);
        let mut cur_bg_c = AnsiColor::Color8(0);
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
                cur_bg_c = AnsiColor::Color8(0);
                cur_c = AnsiColor::Color8(0);
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
                        bg_color: cur_bg_c,
                        color: cur_c,
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
                Token::ColorFgBg(fg, bg) => {
                    cur_bg_c = bg;
                    cur_c = fg;
                }
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
                        30..=37 | 90..=97 => cur_c = AnsiColor::Color8(background),
                        40..=47 | 100..=107 => cur_bg_c = AnsiColor::Color8(background),
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
                    cur_c = AnsiColor::Color8(front);
                    cur_bg_c = AnsiColor::Color8(background);
                }
                Token::Sgr4(reset, ctrl, front, background) => {
                    if reset == 0 {
                        reset_all();
                    }
                    match ctrl {
                        0 => reset_all(),
                        1 => {
                            bold = true;
                            cur_c = AnsiColor::Color8(front);
                            cur_bg_c = AnsiColor::Color8(background);
                        }
                        5 => {
                            blink = true;
                            cur_bg_c = AnsiColor::Color8(front);
                            blink_c = background;
                        }
                        _ => {
                            cur_c = AnsiColor::Color8(front);
                            cur_bg_c = AnsiColor::Color8(background);
                        }
                    }
                }

                Token::SlowBlink => blink = true,
                Token::RapidBlink => blink = true,
                Token::ColorInvert => {
                    (cur_bg_c, cur_c) = (cur_c, cur_bg_c);
                }
                Token::NormalIntensity => bold = false,
                Token::NotReversed => {
                    (cur_bg_c, cur_c) = (cur_c, cur_bg_c);
                }
                Token::ColorDefaultForeground => {
                    cur_c = AnsiColor::Color8(0);
                }
                Token::ColorDefaultBackground => {
                    cur_bg_c = AnsiColor::Color8(0);
                }
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

    #[test]
    fn test_starship() {
        let s = "[?2004h]0;/c/wt[30m(B[m[J[K";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);

        let s = "[38;2;218;98;125m[48;2;218;98;125;30mwin[38;2;218;98;125m[30mC:/wt [48;2;252;161;125;38;2;218;98;125m[48;2;134;187;216;38;2;252;161;125m[48;2;6;150;154;38;2;134;187;216m[48;2;51;101;138;38;2;6;150;154m[0m[K";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_vitest_bench() {
        let s = "[36m[7m[1m BENCH [22m[27m[39m [36mSummary[39m";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }
}
