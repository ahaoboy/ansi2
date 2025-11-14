use std::collections::VecDeque;

use crate::{
    ans::min_distance,
    color::{AnsiColor, Color8},
    lex::{Sgr, Token, parse_ansi},
    node::Node,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Canvas {
    pub pixels: Vec<Vec<Node>>,
    pub w: usize,
    pub h: usize,
}

fn set_node(v: &mut Vec<Vec<Node>>, node: Node, x: usize, y: usize) {
    ensure_shape(v, x, y);

    let row = &mut v[y];
    while x >= row.len() {
        let empty = Node {
            bg_color: AnsiColor::Default,
            color: AnsiColor::Default,
            bold: false,
            text: ' '.into(),
            blink: false,
            dim: false,
            italic: false,
            underline: false,
            hide: false,
            strike: false,

            bg_color_r: (0, 0),
            color_r: (0, 0),
            bold_r: (0, 0),
            blink_r: (0, 0),
            text_r: (0, 0),
            dim_r: (0, 0),
            italic_r: (0, 0),
            underline_r: (0, 0),
            hide_r: (0, 0),
            strike_r: (0, 0),
        };
        row.push(empty);
    }

    row[x] = node;
}

fn ensure_shape(v: &mut Vec<Vec<Node>>, w: usize, h: usize) {
    while v.len() <= h {
        v.push(Vec::new());
    }

    for i in v {
        while i.len() < w {
            i.push(Default::default());
        }
    }
}

fn merge_range(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    let (a1, a2) = a;
    let (b1, b2) = b;

    (a1.min(b1), a2.max(b2))
}

fn offset_range(range: (usize, usize), offset: (usize, usize)) -> (usize, usize) {
    let (a1, _) = range;
    let (b1, b2) = offset;

    (a1 + b1, a1 + b2)
}

fn erase(pixels: &mut [Vec<Node>], start_x: usize, start_y: usize, end_x: usize, end_y: usize) {
    for row in pixels.iter_mut().skip(start_y).take(end_y - start_y) {
        for cell in row.iter_mut().skip(start_x).take(end_x - start_x) {
            *cell = Default::default();
        }
    }
}

impl Canvas {
    pub fn new<S: AsRef<str>>(str: S, max_width: Option<usize>) -> Self {
        let s = str.as_ref();
        let (_, lex) = parse_ansi(s).unwrap();
        let mut cur_x = 0;
        let mut cur_y = 0;
        let mut cur_c = AnsiColor::Default;
        let mut cur_bg_c = AnsiColor::Default;
        let mut bold = false;
        let mut dim = false;
        let mut italic = false;
        let mut reverse = false;
        let mut underline = false;
        let mut blink = false;
        let mut strike = false;

        let mut bg_color_r = (0, 0);
        let mut color_r = (0, 0);
        let mut bold_r = (0, 0);
        let mut blink_r = (0, 0);
        // let mut text_r = (0, 0);
        let mut dim_r = (0, 0);
        let mut italic_r = (0, 0);
        let mut underline_r = (0, 0);
        let mut hide_r = (0, 0);
        let mut strike_r = (0, 0);

        let mut w = 0;
        let mut h = 0;
        let mut pixels = Vec::new();
        let mut hide = false;
        let max_width = max_width.unwrap_or(usize::MAX);

        let mut q = VecDeque::from(lex);

        while let Some(token) = q.pop_front() {
            let Token { sgr: i, range } = token;
            // eprintln!("{:?} {:?}", i, range);

            macro_rules! set_bg_color {
                ($color:expr) => {
                    if reverse && $color == AnsiColor::Default {
                        cur_bg_c = AnsiColor::Color8(Color8::Black);
                    } else {
                        cur_bg_c = $color;
                    }

                    bg_color_r = range;
                };
            }

            macro_rules! set_color {
                ($color:expr) => {
                    if reverse && $color == AnsiColor::Default {
                        cur_c = AnsiColor::Color8(Color8::White);
                    } else {
                        cur_c = $color;
                    }
                    color_r = range;
                };
            }

            match i {
                Sgr::LineFeed => {
                    cur_y += 1;
                    cur_x = 0;
                    ensure_shape(&mut pixels, w, cur_y);
                }

                Sgr::Char(c) => {
                    // text_r = range;
                    let node = Node {
                        text: c.into(),
                        bg_color: cur_bg_c,
                        color: cur_c,
                        bold,
                        blink,
                        dim,
                        italic,
                        underline,
                        hide,
                        strike,

                        bg_color_r,
                        color_r,
                        bold_r,
                        blink_r,
                        text_r: range,
                        dim_r,
                        italic_r,
                        underline_r,
                        hide_r,
                        strike_r,
                    };
                    if cur_x >= max_width {
                        cur_x = 0;
                        cur_y += 1;
                    }
                    set_node(&mut pixels, node, cur_x, cur_y);
                    cur_x += 1;
                }
                Sgr::ColorBackground(c) => {
                    if reverse {
                        set_color!(c);
                    } else {
                        set_bg_color!(c);
                    }
                }
                Sgr::ColorForeground(c) => {
                    if reverse {
                        set_bg_color!(c);
                    } else {
                        set_color!(c);
                    }
                }
                Sgr::ColorFgBg(fg, bg) => {
                    if reverse {
                        set_color!(bg);
                        set_bg_color!(fg);
                    } else {
                        set_color!(fg);
                        set_bg_color!(bg);
                    }
                }
                Sgr::Bold => {
                    bold = true;
                    bold_r = range;
                }
                Sgr::Italic => {
                    italic = true;
                    italic_r = range;
                }
                Sgr::UnItalic => {
                    italic = false;
                    italic_r = range;
                }
                Sgr::Underline => {
                    underline = true;
                    underline_r = range;
                }
                Sgr::UnUnderlined => {
                    underline = false;
                    underline_r = range;
                }
                Sgr::Dim => {
                    dim = true;
                    dim_r = range;
                }
                Sgr::ColorReset => {
                    bold = false;
                    dim = false;
                    italic = false;
                    underline = false;
                    reverse = false;
                    cur_bg_c = AnsiColor::Default;
                    cur_c = AnsiColor::Default;
                    blink = false;
                    hide = false;
                    strike = false;

                    bold_r = range;
                    dim_r = range;
                    italic_r = range;
                    underline_r = range;
                    bg_color_r = range;
                    color_r = range;
                    blink_r = range;
                    hide_r = range;
                    strike_r = range;
                }
                Sgr::CursorUp(c) => cur_y = cur_y.saturating_sub(c as usize),
                Sgr::CursorDown(c) => {
                    cur_y += c as usize;
                    ensure_shape(&mut pixels, w, cur_y);
                }
                Sgr::CursorBack(c) => cur_x = cur_x.saturating_sub(c as usize),
                Sgr::CursorForward(c) => {
                    cur_x += c as usize;
                    if cur_x >= max_width {
                        cur_x %= max_width;
                        cur_y += 1;
                    }
                    ensure_shape(&mut pixels, cur_x, cur_y);
                }
                Sgr::Backspace => cur_x = cur_x.saturating_sub(1),
                Sgr::Tab => {
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
                    ensure_shape(&mut pixels, w, cur_y);
                }

                Sgr::CarriageReturn => cur_x = 0,

                Sgr::CursorNextLine(n) => {
                    cur_y += n as usize;
                    cur_x = 0;
                    ensure_shape(&mut pixels, w, cur_y);
                }
                Sgr::CursorPreviousLine(n) => {
                    cur_y = cur_y.saturating_sub(n as usize);
                    cur_x = 0;
                    ensure_shape(&mut pixels, w, cur_y);
                }
                Sgr::CursorHorizontalAbsolute(n) => cur_x = (n - 1).max(0) as usize,
                Sgr::CursorPosition(x, y) => {
                    cur_x = x as usize;
                    cur_y = y as usize;
                    ensure_shape(&mut pixels, w, cur_y);
                }
                Sgr::SlowBlink | Sgr::RapidBlink => blink = true,
                Sgr::UnBlink => blink = false,
                Sgr::Reverse => {
                    reverse = true;
                    let tmp_c = cur_c;
                    let tmp_bg_c = cur_bg_c;
                    set_color!(tmp_bg_c);
                    set_bg_color!(tmp_c);
                }
                Sgr::NormalIntensity => {
                    dim = false;
                    bold = false;

                    dim_r = range;
                    bold_r = range;
                }
                Sgr::UnReversed => {
                    reverse = false;
                    set_bg_color!(AnsiColor::Default);
                    set_color!(AnsiColor::Default);
                }
                Sgr::Strike => {
                    strike = true;
                    strike_r = range;
                }
                Sgr::UnStrike => {
                    strike = false;
                    strike_r = range;
                }
                Sgr::ColorDefaultForeground => {
                    if reverse {
                        set_bg_color!(AnsiColor::Default);
                    } else {
                        set_color!(AnsiColor::Default);
                    }
                }
                Sgr::ColorDefaultBackground => {
                    if reverse {
                        set_color!(AnsiColor::Default);
                    } else {
                        set_bg_color!(AnsiColor::Default);
                    }
                }

                Sgr::Link(_, title) => {
                    if title.contains("\x1b") {
                        if let Ok((_, tokens)) = parse_ansi(&title) {
                            // FIXME: Avoid the influence of styles in link on subsequent characters
                            q.push_front(Token {
                                range,
                                sgr: Sgr::ColorReset,
                            });
                            for mut i in tokens.into_iter().rev() {
                                i.range = offset_range(i.range, range);
                                q.push_front(i);
                            }
                            q.push_front(Token {
                                range,
                                sgr: Sgr::Underline,
                            });
                        }
                    } else {
                        for (k, i) in title.chars().enumerate() {
                            if i == '\n' {
                                cur_x = 0;
                                cur_y += 1;
                                ensure_shape(&mut pixels, w, cur_y);
                                continue;
                            }

                            let node = Node {
                                text: i.into(),
                                bg_color: cur_bg_c,
                                color: cur_c,
                                bold,
                                blink,
                                dim,
                                italic,
                                underline: true,
                                hide,
                                strike,

                                bg_color_r,
                                color_r,
                                bold_r,
                                blink_r,
                                text_r: (range.0 + k, range.1 + k),
                                dim_r,
                                italic_r,
                                underline_r,
                                hide_r,
                                strike_r,
                            };

                            if cur_x >= max_width {
                                cur_x = 0;
                                cur_y += 1;
                            }
                            set_node(&mut pixels, node, cur_x, cur_y);
                            cur_x += 1;
                        }
                    }
                }
                Sgr::CursorHide => {
                    hide = true;
                    hide_r = range;
                }
                Sgr::UnHide => {
                    hide = false;
                    hide_r = range;
                }
                Sgr::DoublyUnderlined => {
                    bold = false;
                    underline = true;

                    bold_r = range;
                    underline_r = range;
                }
                Sgr::List(v) => {
                    for i in v.into_iter().rev() {
                        q.push_front(Token {
                            range: (0, 0),
                            sgr: i,
                        });
                    }
                }
                Sgr::EraseInDisplay(n) => match n {
                    0 => {
                        erase(&mut pixels, cur_x, cur_y, w, h);
                    }
                    _ => {
                        erase(&mut pixels, 0, 0, w, h);
                    }
                },
                Sgr::EraseInLine(_) => {
                    erase(&mut pixels, cur_x, cur_y, w, cur_y);
                }
                _ => {}
            }

            w = w.max(cur_x + 1);
            h = h.max(cur_y + 1);
            ensure_shape(&mut pixels, w, h);
        }

        Canvas { pixels, w, h }
    }

    pub fn minify(&self) -> Vec<Vec<Node>> {
        let mut v = vec![];
        for row in &self.pixels {
            let row_len = row.len();

            if row_len == 0 {
                v.push(vec![]);
                continue;
            }

            let mut block = row[0].clone();

            let mut list = vec![];
            for c in row.iter().take(row_len).skip(1) {
                if c.same_style(&block) {
                    block.text.push_str(&c.text);
                    block.text_r = merge_range(block.text_r, c.text_r);
                } else {
                    list.push(block.clone());
                    block = c.clone();
                }
            }

            list.push(block);
            v.push(list);
        }

        v
    }
}

pub fn pixels_to_ans(pixels: Vec<Vec<Node>>) -> String {
    let mut text: Vec<String> = Vec::new();
    let mut last_node = Node::default();
    for row in pixels.iter() {
        let mut row_str = Vec::new();
        for c in row.iter() {
            row_str.push(min_distance(&last_node, c));
            row_str.push(c.text.clone());
            last_node = c.clone();
        }
        text.push(row_str.into_iter().collect());
    }
    text.join("\n")
}
