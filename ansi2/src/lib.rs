use std::str::Chars;

struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(s: &'a str) -> Self {
        let chars = s.chars();
        Lexer { chars }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Char(char),
    Color(u32),
    Bold,
    Reset,
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
    Hide,
    Unhide,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.chars.next()?;
        if c.is_ascii_control() && c != '\n' {
            match self.chars.next()? {
                '[' => {
                    let mut num = 0;
                    let t = loop {
                        let n = self.chars.next()?;
                        if n.is_numeric() {
                            num = num * 10 + (n.to_digit(10)?);
                        } else if n == '?' {
                            continue;
                        } else {
                            break n;
                        }
                    };
                    match c {
                        '\x1b' => match t {
                            'm' | 'M' => {
                                if num == 0 {
                                    return Some(Token::Reset);
                                }
                                if num == 1 {
                                    return Some(Token::Bold);
                                }
                                Some(Token::Color(num))
                            }
                            'a' | 'A' => Some(Token::Up(num)),
                            'b' | 'B' => Some(Token::Down(num)),
                            'c' | 'C' => Some(Token::Right(num)),
                            'd' | 'D' => Some(Token::Left(num)),
                            'l' | 'L' => Some(Token::Hide),
                            'h' | 'H' => Some(Token::Unhide),
                            ';' => {
                                assert_eq!(self.chars.next()?, '5');
                                assert_eq!(self.chars.next()?, ';');
                                let mut color = 0;
                                let m = loop {
                                    let n = self.chars.next()?;
                                    if n.is_numeric() {
                                        color = color * 10 + (n.to_digit(10)?);
                                    } else {
                                        break n;
                                    }
                                };
                                assert_eq!(m, 'm');
                                match num {
                                    38 => {
                                        if color <= 7 {
                                            return Some(Token::Color(color + 30));
                                        }
                                        if (8..=15).contains(&color) {
                                            return Some(Token::Color(color + 82));
                                        }
                                        Some(Token::Color(color))
                                    }
                                    48 => {
                                        if color <= 7 {
                                            return Some(Token::Color(color + 40));
                                        }
                                        if (8..=15).contains(&color) {
                                            return Some(Token::Color(color + 92));
                                        }
                                        Some(Token::Color(color))
                                    }
                                    _ => {
                                        todo!()
                                    }
                                }
                            }
                            _ => self.next(),
                        },
                        _ => {
                            todo!()
                        }
                    }
                }
                '\n' => self.next(),
                _ => {
                    todo!()
                }
            }
        } else {
            Some(Token::Char(c))
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnsiColor(pub u32);

impl AnsiColor {
    pub fn new(c: u32) -> Self {
        AnsiColor(c)
    }

    pub fn to_rgb(&self) -> &'static str {
        match self.0 {
            30 | 40 => "rgb(0,0,0)",
            31 | 41 => "rgb(205, 49, 49)",
            32 | 42 => "rgb(13, 188, 121)",
            33 | 43 => "rgb(229, 229, 16)",
            34 | 44 => "rgb(36, 114, 200)",
            35 | 45 => "rgb(188, 63, 188)",
            36 | 46 => "rgb(17, 168, 205)",
            37 | 47 => "rgb(229, 229, 229)",

            90 | 100 => "rgb(102, 102, 102)",
            91 | 101 => "rgb(241, 76, 76)",
            92 | 102 => "rgb(35, 209, 139)",
            93 | 103 => "rgb(245, 245, 67)",
            94 | 104 => "rgb(59, 142, 234)",
            95 | 105 => "rgb(214, 112, 214)",
            96 | 106 => "rgb(41, 184, 219)",
            97 | 107 => "rgb(229, 229, 229)",
            _ => "white",
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
        let lex = Lexer::new(s);

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
                Token::Char(c) => {
                    if c == '\n' {
                        cur_y += 1;
                        cur_x = 0;
                    } else {
                        let node = Node {
                            char: c,
                            bg_color: AnsiColor::new(cur_bg_c),
                            color: AnsiColor::new(cur_c),
                            bold,
                        };
                        set_node(&mut pixels, node, cur_x, cur_y);
                        cur_x += 1;
                    }

                    w = w.max(cur_x + 1);
                    h = h.max(cur_y + 1);
                }
                Token::Color(c) => {
                    if (40..=47).contains(&c) | (100..=107).contains(&c) {
                        cur_bg_c = c
                    } else {
                        cur_c = c
                    }
                }
                Token::Bold => bold = true,
                Token::Reset => {
                    bold = false;
                    cur_c = 0;
                    cur_bg_c = 0;
                }
                Token::Up(c) => {
                    if cur_y > c as usize {
                        cur_y -= c as usize
                    } else {
                        cur_y = 0;
                    }
                }
                Token::Down(c) => {
                    cur_y += c as usize;
                }
                Token::Left(c) => {
                    if cur_x > c as usize {
                        cur_x -= c as usize
                    } else {
                        cur_x = 0;
                    }
                }
                Token::Right(c) => {
                    cur_x += c as usize;
                }
                _ => {}
            }
        }

        Canvas { pixels, w, h }
    }
}
