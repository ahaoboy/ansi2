pub mod css;
pub mod html;
pub mod lex;
pub mod svg;
pub mod text;
pub mod theme;
use std::collections::VecDeque;

use lex::{parse_ansi, AnsiColor, Token};

#[derive(Debug, Clone)]
pub struct Node {
    pub bg_color: AnsiColor,
    pub color: AnsiColor,
    pub bold: bool,
    pub blink: bool,
    pub char: char,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub hide: bool,
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
            dim: false,
            italic: false,
            underline: false,
            hide: false,
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
        let mut dim = false;
        let mut italic = false;
        let mut underline = false;
        let mut blink = false;
        let mut blink_c = 0;
        let mut w = 0;
        let mut h = 0;
        let mut pixels = Vec::new();
        let mut hide = false;
        let max_width = max_width.unwrap_or(usize::MAX);

        let mut q = VecDeque::from(lex);

        while let Some(i) = q.pop_front() {
            let mut reset_all = || {
                bold = false;
                dim = false;
                italic = false;
                underline = false;

                cur_bg_c = AnsiColor::Color8(0);
                cur_c = AnsiColor::Color8(0);
                blink = false;
                blink_c = 0;
                hide = false;
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
                        dim,
                        italic,
                        underline,
                        hide,
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
                Token::Italic => {
                    italic = true;
                }
                Token::Underline => {
                    underline = true;
                }
                Token::Dim => {
                    dim = true;
                }
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
                Token::CursorHorizontalAbsolute(n) => cur_x = (n - 1).max(0) as usize,
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
                Token::Sgr4(reset, ctrl, a, b) => {
                    if reset == 0 {
                        reset_all();
                    }
                    match ctrl {
                        0 => reset_all(),
                        1 => {
                            bold = true;
                            cur_c = AnsiColor::Color8(a);
                            cur_bg_c = AnsiColor::Color8(b);
                        }
                        5 => {
                            blink = true;
                            cur_bg_c = AnsiColor::Color256(a);
                            blink_c = b;
                        }
                        48 => match a {
                            5 => {
                                cur_bg_c = AnsiColor::Color256(b);
                            }
                            _ => {
                                cur_bg_c = AnsiColor::Color8(b);
                            }
                        },
                        38 => match a {
                            5 => {
                                cur_c = AnsiColor::Color256(b);
                            }
                            _ => {
                                cur_c = AnsiColor::Color8(b);
                            }
                        },
                        _ => {
                            cur_c = AnsiColor::Color8(a);
                            cur_bg_c = AnsiColor::Color8(b);
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

                Token::Link(_, title) => match parse_ansi(&title) {
                    Ok((_, tokens)) => {
                        // FIXME: Avoid the influence of styles in link on subsequent characters
                        q.push_front(Token::ColorReset);
                        for i in tokens.into_iter().rev() {
                            underline = true;
                            q.push_front(i);
                        }
                    }
                    Err(_) => {
                        for i in title.chars() {
                            if i == '\n' {
                                cur_x = 0;
                                cur_y += 1;
                                continue;
                            }

                            let node = Node {
                                char: i,
                                bg_color: cur_bg_c,
                                color: cur_c,
                                bold,
                                blink,
                                dim,
                                italic,
                                underline: true,
                                hide,
                            };

                            if cur_x >= max_width {
                                cur_x = 0;
                                cur_y += 1;
                            }
                            set_node(&mut pixels, node, cur_x, cur_y);
                            cur_x += 1;
                        }
                    }
                },
                Token::CursorHide => {
                    hide = true;
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
        let s = "\x1b[0;5;35;45m";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }
    #[test]
    fn test_reset() {
        let s = "\x1b[m\x1b";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_starship() {
        let s = "\x1b[?2004h\x1b]0;/c/wt\x1b[30m\x1b(B\x1b[m\x1b[J\x1b[K";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);

        let s = "\x1b[38;2;218;98;125m\x1b[48;2;218;98;125;30mwin\x1b[38;2;218;98;125m\x1b[30mC:/wt \x1b[48;2;252;161;125;38;2;218;98;125m\x1b[48;2;134;187;216;38;2;252;161;125m\x1b[48;2;6;150;154;38;2;134;187;216m\x1b[48;2;51;101;138;38;2;6;150;154m\x1b[0m\x1b[K";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_vitest_bench() {
        let s = "\x1b[36m\x1b[7m\x1b[1m BENCH \x1b[22m\x1b[27m\x1b[39m \x1b[36mSummary\x1b[39m";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_fastfetch() {
        let s = "\x1b[1G\x1b[19A\x1b[47C";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_color256() {
        let s = "\x1b[38;5;99ma\x1b[48;5;99mb";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_color24() {
        let s = "\x1b[38;2;0;0;114m";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }
    #[test]
    fn test_base() {
        let s =
            "\x1b[30mblack\x1b[0m    \x1b[90mbright black\x1b[0m     \x1b[40mblack\x1b[0m    \x1b[100mbright black\x1b[0m";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_link() {
        let s =
            "\x1b]8;;file:///Users/xxx/src/new-nu-parser/Cargo.toml\x1b\\Cargo.toml\x1b]8;;\x1b";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }
    #[test]
    fn test_link_hide() {
        let s = "\x1b[8mhttp://example.com/how_about_me\x1b[m";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_link_id() {
        let s = "\x1b]8;id=1;http://example.com/id\x1b\\twice\x1b]8;;\x1b\\";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_empty_link() {
        let s = "\x1b]8;;\x1b\\";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_link_soft_reset() {
        let s = "\x1b]8;;http://example.com/softreset\\\x1b[3;31mfoo[!pbar";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_link_no_close() {
        let s = "\x1b]8;;http://example.com/foo\x1b\\foo\x1b]8;;http://example.com/foo\x1b\\foo\x1b]8;;\x1b\\ \x1b]8;;http://example.com/foo\x1b\\foo\x1b]8;;http://example.com/bar\x1b\\bar\x1b]8;;\x1b\\";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_sgr6() {
        let s = "\x1b[48;5;186;38;5;16m";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_style() {
        let s =
            "aaa\x1b[1mbold\x1b[0m \x1b[2mdim\x1b[0m \x1b[3mitalic\x1b[3m \x1b[4munderline\x1b[4m";
        let r = parse_ansi(s).unwrap();
        println!("{:?}", r);
    }
}
