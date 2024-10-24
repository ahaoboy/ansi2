pub mod color;
#[allow(clippy::too_many_arguments)]
pub mod css;
pub mod html;
pub mod lex;
pub mod svg;
pub mod text;
pub mod theme;
use color::AnsiColor;
use lex::{parse_ansi, Token};
use std::{collections::VecDeque, vec};

#[derive(Debug, Clone)]
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
}

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
    }
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
            bg_color: AnsiColor::Default,
            color: AnsiColor::Default,
            bold: false,
            text: ' '.into(),
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
        let mut cur_c = AnsiColor::Default;
        let mut cur_bg_c = AnsiColor::Default;
        let mut bold = false;
        let mut dim = false;
        let mut italic = false;
        let mut reverse = false;
        let mut underline = false;
        let mut blink = false;
        let mut w = 0;
        let mut h = 0;
        let mut pixels = Vec::new();
        let mut hide = false;
        let max_width = max_width.unwrap_or(usize::MAX);

        let mut q = VecDeque::from(lex);

        macro_rules! set_bg_color {
            ($color:expr) => {
                if reverse && $color == AnsiColor::Default {
                    cur_bg_c = AnsiColor::Color8(color::Color8::Black);
                } else {
                    cur_bg_c = $color;
                }
            };
        }

        macro_rules! set_color {
            ($color:expr) => {
                if reverse && $color == AnsiColor::Default {
                    cur_c = AnsiColor::Color8(color::Color8::White);
                } else {
                    cur_c = $color;
                }
            };
        }

        while let Some(i) = q.pop_front() {
            match i {
                Token::LineFeed => {
                    cur_y += 1;
                    cur_x = 0;
                }
                Token::Char(c) => {
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
                    };
                    if cur_x >= max_width {
                        cur_x = 0;
                        cur_y += 1;
                    }
                    set_node(&mut pixels, node, cur_x, cur_y);
                    cur_x += 1;
                }
                Token::ColorBackground(c) => {
                    if reverse {
                        set_color!(c);
                    } else {
                        set_bg_color!(c);
                    }
                }
                Token::ColorForeground(c) => {
                    if reverse {
                        set_bg_color!(c);
                    } else {
                        set_color!(c);
                    }
                }
                Token::ColorFgBg(fg, bg) => {
                    if reverse {
                        set_color!(bg);
                        set_bg_color!(fg);
                    } else {
                        set_color!(fg);
                        set_bg_color!(bg);
                    }
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
                    bold = false;
                    dim = false;
                    italic = false;
                    underline = false;
                    reverse = false;

                    cur_bg_c = AnsiColor::Default;
                    cur_c = AnsiColor::Default;
                    blink = false;
                    hide = false;
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
                Token::SlowBlink | Token::RapidBlink => blink = true,
                Token::Reverse => {
                    reverse = true;
                    let tmp_c = cur_c;
                    let tmp_bg_c = cur_bg_c;
                    set_color!(tmp_bg_c);
                    set_bg_color!(tmp_c);
                }
                Token::NormalIntensity => {
                    dim = false;
                    bold = false;
                }
                Token::NotReversed => {
                    reverse = false;
                    set_bg_color!(AnsiColor::Default);
                    set_color!(AnsiColor::Default);
                }
                Token::ColorDefaultForeground => {
                    if reverse {
                        set_bg_color!(AnsiColor::Default);
                    } else {
                        set_color!(AnsiColor::Default);
                    }
                }
                Token::ColorDefaultBackground => {
                    if reverse {
                        set_color!(AnsiColor::Default);
                    } else {
                        set_bg_color!(AnsiColor::Default);
                    }
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
                                text: i.into(),
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

                Token::List(v) => {
                    for i in v.into_iter().rev() {
                        q.push_front(i);
                    }
                }
                _ => {}
            }

            w = w.max(cur_x + 1);
            h = h.max(cur_y + 1);
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

#[cfg(test)]
mod test {
    use crate::{lex::parse_ansi, Canvas};
    use insta::assert_debug_snapshot;
    #[test]
    fn test_plain() {
        let s = "ansi";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test() {
        let s = "\x1b[0;5;35;45m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_reset() {
        let s = "\x1b[m\x1b";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_starship_title() {
        let s = "\x1b[?2004h\x1b]0;/c/wt\x1b[30m\x1b(B\x1b[m\x1b[J\x1b[K";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_starship_prompt() {
        let s = "\x1b[38;2;218;98;125m\x1b[48;2;218;98;125;30mwin\x1b[38;2;218;98;125m\x1b[30mC:/wt \x1b[48;2;252;161;125;38;2;218;98;125m\x1b[48;2;134;187;216;38;2;252;161;125m\x1b[48;2;6;150;154;38;2;134;187;216m\x1b[48;2;51;101;138;38;2;6;150;154m\x1b[0m\x1b[K";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_vitest_bench() {
        let s = "\x1b[36m\x1b[7m\x1b[1m BENCH \x1b[22m\x1b[27m\x1b[39m \x1b[36mSummary\x1b[39m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_fastfetch() {
        let s = "\x1b[1G\x1b[19A\x1b[47C";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_color256() {
        let s = "\x1b[38;5;99ma\x1b[48;5;99mb";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_color24() {
        let s = "\x1b[38;2;0;0;114m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_base() {
        let s =
            "\x1b[30mblack\x1b[0m    \x1b[90mbright black\x1b[0m     \x1b[40mblack\x1b[0m    \x1b[100mbright black\x1b[0m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_link() {
        let s =
            "\x1b]8;;file:///Users/xxx/src/new-nu-parser/Cargo.toml\x1b\\Cargo.toml\x1b]8;;\x1b";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_link_hide() {
        let s = "\x1b[8mhttp://example.com/how_about_me\x1b[m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_link_id() {
        let s = "\x1b]8;id=1;http://example.com/id\x1b\\twice\x1b]8;;\x1b\\";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_empty_link() {
        let s = "\x1b]8;;\x1b\\";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_link_soft_reset() {
        let s = "\x1b]8;;http://example.com/softreset\\\x1b[3;31mfoo[!pbar";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_link_no_close() {
        let s = "\x1b]8;;http://example.com/foo\x1b\\foo\x1b]8;;http://example.com/foo\x1b\\foo\x1b]8;;\x1b\\ \x1b]8;;http://example.com/foo\x1b\\foo\x1b]8;;http://example.com/bar\x1b\\bar\x1b]8;;\x1b\\";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_link_ll() {
        let s = "]8;;file://win/c/code/ansi2/targettarget]8;;";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_sgr6() {
        let s = "\x1b[48;5;186;38;5;16m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_style() {
        let s =
            "aaa\x1b[1mbold\x1b[0m \x1b[2mdim\x1b[0m \x1b[3mitalic\x1b[3m \x1b[4munderline\x1b[4m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_dim() {
        let s = "[39m[2;39mNUSHELL";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_sgr4() {
        let s = "[0;48;5;45maaa";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_vitest() {
        let s = "[36m[7m[1m BENCH [22m[27m[39m [36mSummary[39m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_svg2() {
        let s = "[39;40m 39;40m [0m[39;41m 39;41m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
}
