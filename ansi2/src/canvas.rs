use crate::{
    cell::{Cell, SourceMap, Style},
    color::{AnsiColor, Color8},
    lexer::{self, Event, Token},
    span::Span,
};

/// A grid of styled character cells.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Canvas {
    pub pixels: Vec<Vec<Cell>>,
    pub w: usize,
    pub h: usize,
}

impl Canvas {
    pub fn new(input: &str, max_width: Option<usize>) -> Self {
        let mut terminal = Terminal::new(max_width.filter(|width| *width > 0));
        for token in lexer::tokens(input) {
            terminal.apply(token);
        }
        terminal.finish()
    }

    /// Merge adjacent cells that share the same visual style.
    pub fn minify(&self) -> Vec<Vec<Cell>> {
        let mut rows = Vec::with_capacity(self.pixels.len());

        for row in &self.pixels {
            let mut merged: Vec<Cell> = Vec::new();
            for cell in row {
                match merged.last_mut() {
                    Some(last) if last.same_style(cell) => {
                        last.text.push_str(&cell.text);
                        last.src.text = last.src.text.merge(cell.src.text);
                    }
                    _ => merged.push(cell.clone()),
                }
            }
            rows.push(merged);
        }

        rows
    }
}

struct Terminal {
    pixels: Vec<Vec<Cell>>,
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    style: Style,
    src: SourceMap,
    reverse: bool,
    max_width: Option<usize>,
}

impl Terminal {
    fn new(max_width: Option<usize>) -> Self {
        Self {
            pixels: Vec::new(),
            w: 0,
            h: 0,
            x: 0,
            y: 0,
            style: Style::default(),
            src: SourceMap::default(),
            reverse: false,
            max_width,
        }
    }

    fn apply(&mut self, token: Token) {
        let Token { event, span } = token;

        match event {
            Event::Char(c) => {
                let src = SourceMap {
                    text: span,
                    ..self.src
                };
                self.put(c, self.style, src);
            }
            Event::List(events) => {
                for event in events {
                    self.apply(Token { event, span });
                }
            }
            Event::Link {
                title, title_span, ..
            } => {
                self.link(&title, title_span, span);
            }

            Event::LineFeed => {
                self.y += 1;
                self.x = 0;
                self.touch_height();
            }
            Event::Backspace => {
                self.x = self.x.saturating_sub(1);
                self.touch_height();
            }
            Event::Tab => {
                let remainder = self.x & 7;
                self.x += if remainder == 0 { 8 } else { 8 - remainder };
                self.wrap();
                self.touch_width();
                self.touch_height();
            }
            Event::CarriageReturn => {
                self.x = 0;
                self.touch_height();
            }

            Event::CursorUp(n) => {
                self.y = self.y.saturating_sub(n as usize);
                self.touch_height();
            }
            Event::CursorDown(n) => {
                self.y += n as usize;
                self.touch_height();
            }
            Event::CursorForward(n) => {
                self.x += n as usize;
                self.wrap();
                self.touch_width();
                self.touch_height();
            }
            Event::CursorBack(n) => {
                self.x = self.x.saturating_sub(n as usize);
                self.touch_height();
            }
            Event::CursorNextLine(n) => {
                self.y += n as usize;
                self.x = 0;
                self.touch_height();
            }
            Event::CursorPreviousLine(n) => {
                self.y = self.y.saturating_sub(n as usize);
                self.x = 0;
                self.touch_height();
            }
            Event::CursorHorizontalAbsolute(n) => {
                self.x = (n - 1).max(0) as usize;
                self.touch_width();
                self.touch_height();
            }
            Event::CursorPosition(row, col) => {
                self.y = (row - 1).max(0) as usize;
                self.x = (col - 1).max(0) as usize;
                self.touch_width();
                self.touch_height();
            }
            Event::HorizontalVerticalPosition(row, col) => {
                self.y = (row - 1).max(0) as usize;
                self.x = (col - 1).max(0) as usize;
                self.touch_width();
                self.touch_height();
            }

            Event::EraseInDisplay(mode) => self.erase_display(mode),
            Event::EraseInLine(mode) => self.erase_line(mode),

            Event::Foreground(color) => {
                if self.reverse {
                    self.set_bg(color, span);
                } else {
                    self.set_fg(color, span);
                }
            }
            Event::Background(color) => {
                if self.reverse {
                    self.set_fg(color, span);
                } else {
                    self.set_bg(color, span);
                }
            }
            Event::FgBg(fg, bg) => {
                if self.reverse {
                    self.set_fg(bg, span);
                    self.set_bg(fg, span);
                } else {
                    self.set_fg(fg, span);
                    self.set_bg(bg, span);
                }
            }
            Event::Reset => {
                self.style = Style::default();
                self.reverse = false;
                self.src.set_all(span);
            }
            Event::Reverse => {
                self.reverse = true;
                let fg = self.style.bg;
                let bg = self.style.fg;
                self.set_fg(fg, span);
                self.set_bg(bg, span);
            }
            Event::Unreverse => {
                self.reverse = false;
                self.set_fg(AnsiColor::Default, span);
                self.set_bg(AnsiColor::Default, span);
            }
            Event::DefaultForeground => {
                if self.reverse {
                    self.set_bg(AnsiColor::Default, span);
                } else {
                    self.set_fg(AnsiColor::Default, span);
                }
            }
            Event::DefaultBackground => {
                if self.reverse {
                    self.set_fg(AnsiColor::Default, span);
                } else {
                    self.set_bg(AnsiColor::Default, span);
                }
            }
            Event::DefaultUnderline => {}

            Event::Bold => {
                self.style.bold = true;
                self.src.bold = span;
            }
            Event::Dim => {
                self.style.dim = true;
                self.src.dim = span;
            }
            Event::Italic => {
                self.style.italic = true;
                self.src.italic = span;
            }
            Event::Underline => {
                self.style.underline = true;
                self.src.underline = span;
            }
            Event::SlowBlink | Event::RapidBlink => {
                self.style.blink = true;
                self.src.blink = span;
            }
            Event::Strike => {
                self.style.strike = true;
                self.src.strike = span;
            }
            Event::Conceal => {
                self.style.hide = true;
                self.src.hide = span;
            }
            Event::Unitalic => {
                self.style.italic = false;
                self.src.italic = span;
            }
            Event::Ununderline => {
                self.style.underline = false;
                self.src.underline = span;
            }
            Event::Unblink => {
                self.style.blink = false;
                self.src.blink = span;
            }
            Event::Unstrike => {
                self.style.strike = false;
                self.src.strike = span;
            }
            Event::Unconceal => {
                self.style.hide = false;
                self.src.hide = span;
            }
            Event::NormalIntensity => {
                self.style.bold = false;
                self.style.dim = false;
                self.src.bold = span;
                self.src.dim = span;
            }
            Event::DoublyUnderlined => {
                self.style.bold = false;
                self.style.underline = true;
                self.src.bold = span;
                self.src.underline = span;
            }

            Event::Bell
            | Event::FormFeed
            | Event::Title(_)
            | Event::Cwd(_)
            | Event::Prompt
            | Event::ScrollUp(_)
            | Event::ScrollDown(_)
            | Event::AuxPortOn
            | Event::AuxPortOff
            | Event::DeviceStatusReport
            | Event::CursorShow
            | Event::CursorHide
            | Event::UnderlineColor(_)
            | Event::PrimaryFont
            | Event::AlternativeFont(_)
            | Event::Fraktur
            | Event::Decset
            | Event::Decrst
            | Event::Keypad
            | Event::Ext
            | Event::Unknown(_) => {}
        }
    }

    fn set_fg(&mut self, color: AnsiColor, span: Span) {
        self.style.fg = if self.reverse && color.is_default() {
            AnsiColor::Color8(Color8::White)
        } else {
            color
        };
        self.src.fg = span;
    }

    fn set_bg(&mut self, color: AnsiColor, span: Span) {
        self.style.bg = if self.reverse && color.is_default() {
            AnsiColor::Color8(Color8::Black)
        } else {
            color
        };
        self.src.bg = span;
    }

    fn link(&mut self, title: &str, title_span: Span, link_span: Span) {
        if title.contains('\x1b') {
            self.apply(Token {
                event: Event::Underline,
                span: link_span,
            });
            for token in lexer::tokens(title) {
                self.apply(Token {
                    event: token.event,
                    span: token.span.shift(title_span.start),
                });
            }
            self.apply(Token {
                event: Event::Reset,
                span: link_span,
            });
            return;
        }

        let style = Style {
            underline: true,
            ..self.style
        };
        for (index, c) in title.chars().enumerate() {
            if c == '\n' {
                self.y += 1;
                self.x = 0;
                self.touch_height();
                continue;
            }
            let pos = title_span.start + index;
            let src = SourceMap {
                text: Span::new(pos, pos + 1),
                underline: link_span,
                ..self.src
            };
            self.put(c, style, src);
        }
    }

    fn put(&mut self, c: char, style: Style, src: SourceMap) {
        if let Some(max_width) = self.max_width
            && self.x >= max_width
        {
            self.x = 0;
            self.y += 1;
        }

        self.ensure_row(self.y);
        let row = &mut self.pixels[self.y];
        while row.len() <= self.x {
            row.push(Cell::blank());
        }
        row[self.x] = Cell::new(style, c, src);

        self.x += 1;
        self.w = self.w.max(self.x);
        self.h = self.h.max(self.y + 1);
    }

    fn wrap(&mut self) {
        if let Some(max_width) = self.max_width
            && self.x >= max_width
        {
            self.x %= max_width;
            self.y += 1;
        }
    }

    fn touch_width(&mut self) {
        self.w = self.w.max(self.x + 1);
    }

    fn touch_height(&mut self) {
        self.h = self.h.max(self.y + 1);
    }

    fn ensure_row(&mut self, y: usize) {
        while self.pixels.len() <= y {
            self.pixels.push(Vec::new());
        }
    }

    fn erase_display(&mut self, mode: i32) {
        let (start_x, start_y, end_x, end_y) = match mode {
            0 => (self.x, self.y, self.w, self.h),
            1 => (0, 0, self.x + 1, self.y + 1),
            _ => (0, 0, self.w, self.h),
        };
        self.erase(start_x, start_y, end_x, end_y);
    }

    fn erase_line(&mut self, mode: i32) {
        let (start_x, end_x) = match mode {
            0 => (self.x, self.w),
            1 => (0, self.x + 1),
            _ => (0, self.w),
        };
        self.erase(start_x, self.y, end_x, self.y + 1);
    }

    fn erase(&mut self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) {
        for row in self.pixels.iter_mut().take(end_y).skip(start_y) {
            let end = end_x.min(row.len());
            let start = start_x.min(end);
            for cell in &mut row[start..end] {
                *cell = Cell::blank();
            }
        }
    }

    fn finish(mut self) -> Canvas {
        while self.pixels.len() < self.h {
            self.pixels.push(Vec::new());
        }

        Canvas {
            pixels: self.pixels,
            w: self.w,
            h: self.h,
        }
    }
}
