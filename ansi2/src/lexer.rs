//! A small, allocation-conscious ANSI escape sequence lexer.
//!
//! The parser recognizes exactly the sequences the renderer understands;
//! everything else is preserved as text. Token spans are measured in
//! characters (not bytes) because they are exposed through the sourcemap.

use crate::{
    color::{AnsiColor, Color8},
    span::Span,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Char(char),

    Bell,
    Backspace,
    Tab,
    LineFeed,
    FormFeed,
    CarriageReturn,
    Title(String),
    Cwd(String),
    Prompt,
    CursorUp(i32),
    CursorDown(i32),
    CursorForward(i32),
    CursorBack(i32),
    CursorNextLine(i32),
    CursorPreviousLine(i32),
    CursorHorizontalAbsolute(i32),
    CursorPosition(i32, i32),
    EraseInDisplay(i32),
    EraseInLine(i32),
    ScrollUp(i32),
    ScrollDown(i32),
    HorizontalVerticalPosition(i32, i32),
    AuxPortOn,
    AuxPortOff,
    DeviceStatusReport,

    CursorShow,
    CursorHide,

    Foreground(AnsiColor),
    Background(AnsiColor),
    UnderlineColor(AnsiColor),
    FgBg(AnsiColor, AnsiColor),

    Reset,
    Reverse,
    DefaultForeground,
    DefaultBackground,
    DefaultUnderline,

    Bold,
    NormalIntensity,
    Italic,
    Underline,
    Dim,
    SlowBlink,
    RapidBlink,
    Strike,
    Unstrike,
    Unitalic,
    Ununderline,
    Unblink,
    Unreverse,
    Conceal,
    Unconceal,
    PrimaryFont,
    DoublyUnderlined,
    Decset,
    Decrst,
    Keypad,
    /// `\x1b[>4;1m`
    Ext,

    AlternativeFont(u8),
    Fraktur,
    List(Vec<Event>),

    Link {
        url: String,
        title: String,
        /// Character span of `title` within the original input.
        title_span: Span,
    },

    Unknown(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub event: Event,
    /// Character span of the raw sequence.
    pub span: Span,
}

pub fn parse(input: &str) -> Vec<Token> {
    tokens(input).collect()
}

pub(crate) fn tokens(input: &str) -> impl Iterator<Item = Token> + '_ {
    Lexer::new(input)
}

struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    chars: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            chars: 0,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let rest = &self.input[self.pos..];
        if rest.is_empty() {
            return None;
        }

        let (mut event, len) = scan(rest);
        let end = self.chars + rest[..len].chars().count();
        let span = Span::new(self.chars, end);

        if let Event::Link { title_span, .. } = &mut event {
            *title_span = title_span.shift(span.start);
        }

        self.pos += len;
        self.chars = end;
        Some(Token { event, span })
    }
}

fn scan(rest: &str) -> (Event, usize) {
    let byte = rest.as_bytes()[0];
    match byte {
        0x07 => (Event::Bell, 1),
        0x08 => (Event::Backspace, 1),
        0x09 => (Event::Tab, 1),
        0x0a => (Event::LineFeed, 1),
        0x0c => (Event::FormFeed, 1),
        0x0d => (Event::CarriageReturn, 1),
        0x1b => escape(rest),
        0x1a => (Event::Char('␦'), 1),
        0x00..=0x06 | 0x0e..=0x12 | 0x14 | 0x16 | 0x19 | 0x1c | 0x1e => (Event::Unknown(byte), 1),
        _ => {
            let c = rest.chars().next().expect("non-empty input");
            (Event::Char(c), c.len_utf8())
        }
    }
}

fn escape(s: &str) -> (Event, usize) {
    if let Some(result) = osc(s) {
        return result;
    }

    const EXACT: &[(&str, Event)] = &[
        ("\x1b[?2004l", Event::Decrst),
        ("\x1b[?2004h", Event::Decset),
        ("\x1b[>4;1m", Event::Ext),
        ("\x1b=", Event::Keypad),
        ("\x1b5i", Event::AuxPortOn),
        ("\x1b(B", Event::Bold),
    ];
    for (pattern, event) in EXACT {
        if s.starts_with(pattern) {
            return (event.clone(), pattern.len());
        }
    }

    if let Some(result) = csi(s) {
        return result;
    }

    (Event::Unknown(0x1b), 1)
}

fn osc(s: &str) -> Option<(Event, usize)> {
    if let Some(body) = s.strip_prefix("\x1b]0;")
        && let Some(end) = body.find('\x07')
    {
        let title = &body[..end];
        return Some((Event::Title(title.into()), 4 + end + 1));
    }

    if let Some(body) = s.strip_prefix("\x1b]7;")
        && let Some(end) = body.find('\x07')
    {
        let cwd = &body[..end];
        return Some((Event::Cwd(cwd.into()), 4 + end + 1));
    }

    if let Some(result) = osc_link(s) {
        return Some(result);
    }

    if let Some(body) = s.strip_prefix("\x1b]133;") {
        let end = body
            .char_indices()
            .find(|(_, c)| c.is_control() || c.is_whitespace())
            .map(|(i, _)| i)
            .unwrap_or(body.len());
        if end > 0 {
            return Some((Event::Prompt, 6 + end));
        }
    }

    None
}

fn osc_link(s: &str) -> Option<(Event, usize)> {
    let rest = s.strip_prefix("\x1b]8;")?;
    let (body, semicolon) = match rest.strip_prefix(';') {
        Some(body) => (body, 1),
        None => (rest, 0),
    };
    let prefix = 4 + semicolon;

    // OSC 8 ; params ; URI ST  text  OSC 8 ; ; ST
    if let Some(pos) = body.find("\x1b\\") {
        let url = &body[..pos];
        let after = &body[pos + 2..];
        if let Some((title, consumed)) = take_link_close(after) {
            let title_offset = prefix + url.len() + 2;
            let title_start = s[..title_offset].chars().count();
            return Some((
                Event::Link {
                    url: url.into(),
                    title: title.into(),
                    title_span: Span::new(title_start, title_start + title.chars().count()),
                },
                title_offset + consumed,
            ));
        }
    }

    // OSC 8 ; ; URI ST   (no title)
    if let Some((url, consumed)) = take_link_close(body) {
        let start = s[..prefix].chars().count();
        return Some((
            Event::Link {
                url: url.into(),
                title: url.into(),
                title_span: Span::new(start, start + url.chars().count()),
            },
            prefix + consumed,
        ));
    }

    // OSC 8 ; ; URI BEL text OSC 8 ; ; BEL
    if semicolon == 1 {
        let bel = body.find('\x07')?;
        let url = &body[..bel];
        let after = &body[bel + 1..];
        const CLOSE: &str = "\x1b]8;;\x07";
        let (title, tail) = after.split_once(CLOSE)?;
        let title_offset = prefix + url.len() + 1;
        let title_start = s[..title_offset].chars().count();
        return Some((
            Event::Link {
                url: url.into(),
                title: title.into(),
                title_span: Span::new(title_start, title_start + title.chars().count()),
            },
            title_offset + (after.len() - tail.len()),
        ));
    }

    None
}

fn take_link_close(s: &str) -> Option<(&str, usize)> {
    for close in ["\x1b]8;;\x1b\\", "\x1b[!p"] {
        if let Some((before, after)) = s.split_once(close) {
            return Some((before, s.len() - after.len()));
        }
    }
    None
}

fn csi(s: &str) -> Option<(Event, usize)> {
    if !s.starts_with("\x1b[") {
        return None;
    }
    let bytes = s.as_bytes();
    let mut i = 2;
    let private = bytes.get(i) == Some(&b'?');
    if private {
        i += 1;
    }

    let mut fields = Vec::new();
    let mut separators = Vec::new();

    loop {
        let start = i;
        while bytes.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1;
        }
        fields.push(&s[start..i]);

        match *bytes.get(i)? {
            separator @ (b';' | b':') => {
                separators.push(separator);
                i += 1;
            }
            final_byte @ 0x40..=0x7e => {
                i += 1;
                return interpret(&fields, &separators, private, final_byte)
                    .map(|event| (event, i));
            }
            _ => return None,
        }
    }
}

fn interpret(fields: &[&str], separators: &[u8], private: bool, final_byte: u8) -> Option<Event> {
    let len = fields.len();
    let no_separator = separators.is_empty();
    let semicolons_only = separators.iter().all(|separator| *separator == b';');
    let single = no_separator && len == 1;
    let coordinate = len == 2 && separators.len() == 1 && matches!(separators[0], b';' | b':');

    match final_byte {
        b'm' | b'M' if !private && semicolons_only => sgr(fields),

        b'A' | b'a' if !private && single => Some(Event::CursorUp(num(fields[0], 1))),
        b'B' | b'b' if !private && single => Some(Event::CursorDown(num(fields[0], 1))),
        b'C' | b'c' if !private && single => Some(Event::CursorForward(num(fields[0], 1))),
        b'D' | b'd' if !private && single => Some(Event::CursorBack(num(fields[0], 1))),
        b'E' | b'e' if !private && single => Some(Event::CursorNextLine(num(fields[0], 1))),
        b'F' | b'f' if !private && single => Some(Event::CursorPreviousLine(num(fields[0], 1))),
        b'G' | b'g' if !private && single => {
            Some(Event::CursorHorizontalAbsolute(num(fields[0], 1)))
        }

        b'H' | b'h' if single => Some(Event::CursorShow),
        b'H' | b'h' if !private && coordinate => {
            Some(Event::CursorPosition(num(fields[0], 0), num(fields[1], 0)))
        }
        b'L' | b'l' if single => Some(Event::CursorHide),

        b'F' | b'f' if !private && coordinate => Some(Event::HorizontalVerticalPosition(
            num(fields[0], 0),
            num(fields[1], 0),
        )),

        b'J' | b'j' if !private && single => Some(Event::EraseInDisplay(num(fields[0], 0))),
        b'K' | b'k' if !private && single => Some(Event::EraseInLine(num(fields[0], 0))),
        b'S' | b's' if !private && single => Some(Event::ScrollUp(num(fields[0], 0))),
        b'T' | b't' if !private && single => Some(Event::ScrollDown(num(fields[0], 0))),

        b'i' if !private && single && fields[0] == "4" => Some(Event::AuxPortOff),
        b'n' if !private && single && fields[0] == "6" => Some(Event::DeviceStatusReport),

        _ => None,
    }
}

fn sgr(fields: &[&str]) -> Option<Event> {
    let value = |i: usize| fields[i].parse::<u8>().unwrap_or(0);

    match fields.len() {
        0 => Some(get_sgr(0)),
        1 => Some(get_sgr(value(0))),
        2 => Some(Event::List(vec![
            get_sgr(value(0)),
            get_sgr_color(value(1)),
        ])),
        3 => {
            let (first, second, third) = (value(0), value(1), value(2));
            match (first, second) {
                (38, 5) => Some(Event::Foreground(color256_or_8(third))),
                (48, 5) => Some(Event::Background(color256_or_8(third))),
                (58, 5) => Some(Event::UnderlineColor(color256_or_8(third))),
                _ => Some(Event::List(vec![
                    get_sgr(first),
                    get_sgr_color(second),
                    get_sgr_color(third),
                ])),
            }
        }
        4 => {
            let (reset, kind, sub, last) = (value(0), value(1), value(2), value(3));
            let mut events = vec![get_sgr(reset)];
            match kind {
                48 => events.push(Event::Background(match sub {
                    5 => AnsiColor::Color256(last),
                    _ => AnsiColor::Color8(Color8::from_u8(last)),
                })),
                38 => events.push(Event::Foreground(match sub {
                    5 => AnsiColor::Color256(last),
                    _ => AnsiColor::Color8(Color8::from_u8(last)),
                })),
                5 | 6 => {
                    events.push(get_sgr(kind));
                    events.push(Event::Foreground(AnsiColor::Color8(Color8::from_u8(sub))));
                    events.push(Event::Background(AnsiColor::Color8(Color8::from_u8(last))));
                }
                _ => {
                    events.push(get_sgr(kind));
                    events.push(Event::Foreground(AnsiColor::Color256(sub)));
                    events.push(Event::Background(AnsiColor::Color256(last)));
                }
            }
            Some(Event::List(events))
        }
        5 => {
            let (kind, ty, r, g, b) = (value(0), value(1), value(2), value(3), value(4));
            match (kind, ty) {
                (38, 2) => Some(Event::Foreground(AnsiColor::Rgb(r, g, b))),
                (48, 2) => Some(Event::Background(AnsiColor::Rgb(r, g, b))),
                _ => None,
            }
        }
        6 => {
            let (kind, ty, r, g, b, last) =
                (value(0), value(1), value(2), value(3), value(4), value(5));
            match (kind, ty) {
                (38, 2) => Some(Event::FgBg(
                    AnsiColor::Rgb(r, g, b),
                    AnsiColor::Color8(Color8::from_u8(last)),
                )),
                (48, 2) => Some(Event::FgBg(
                    AnsiColor::Color8(Color8::from_u8(last)),
                    AnsiColor::Rgb(r, g, b),
                )),
                (48, 5) if g == 38 && b == 5 => Some(Event::FgBg(
                    AnsiColor::Color256(last),
                    AnsiColor::Color256(r),
                )),
                (38, 5) if g == 48 && b == 5 => Some(Event::FgBg(
                    AnsiColor::Color256(r),
                    AnsiColor::Color256(last),
                )),
                _ => None,
            }
        }
        7 => {
            let (reset, k1, t1, v1, k2, t2, v2) = (
                value(0),
                value(1),
                value(2),
                value(3),
                value(4),
                value(5),
                value(6),
            );
            let mut events = vec![get_sgr(reset)];
            push_color(&mut events, k1, t1, v1);
            push_color(&mut events, k2, t2, v2);
            Some(Event::List(events))
        }
        10 => {
            let first = (value(0), value(1), value(2), value(3), value(4));
            let second = (value(5), value(6), value(7), value(8), value(9));
            let mut fg = AnsiColor::Default;
            let mut bg = AnsiColor::Default;
            apply_rgb(&mut fg, &mut bg, first);
            apply_rgb(&mut fg, &mut bg, second);
            Some(Event::FgBg(fg, bg))
        }
        _ => None,
    }
}

fn apply_rgb(fg: &mut AnsiColor, bg: &mut AnsiColor, (kind, ty, r, g, b): (u8, u8, u8, u8, u8)) {
    if ty != 2 {
        return;
    }
    match kind {
        38 => *fg = AnsiColor::Rgb(r, g, b),
        48 => *bg = AnsiColor::Rgb(r, g, b),
        _ => {}
    }
}

fn push_color(events: &mut Vec<Event>, kind: u8, ty: u8, value: u8) {
    match (kind, ty) {
        (38, 5) => events.push(Event::Foreground(AnsiColor::Color256(value))),
        (38, 2) => events.push(Event::Foreground(AnsiColor::Color8(Color8::from_u8(value)))),
        (48, 5) => events.push(Event::Background(AnsiColor::Color256(value))),
        (48, 2) => events.push(Event::Background(AnsiColor::Color8(Color8::from_u8(value)))),
        _ => {}
    }
}

fn num(field: &str, default: i32) -> i32 {
    field.parse().unwrap_or(default)
}

fn color256_or_8(value: u8) -> AnsiColor {
    match value {
        0..=7 => AnsiColor::Color8(Color8::from_u8(value + 30)),
        8..=15 => AnsiColor::Color8(Color8::from_u8(value + 82)),
        _ => AnsiColor::Color256(value),
    }
}

fn get_sgr(value: u8) -> Event {
    match value {
        0 => Event::Reset,
        1 => Event::Bold,
        2 => Event::Dim,
        3 => Event::Italic,
        4 => Event::Underline,
        5 => Event::SlowBlink,
        6 => Event::RapidBlink,
        7 => Event::Reverse,
        8 => Event::Conceal,
        9 => Event::Strike,
        10 => Event::PrimaryFont,
        11..=19 => Event::AlternativeFont(value - 10),
        20 => Event::Fraktur,
        21 => Event::DoublyUnderlined,
        22 => Event::NormalIntensity,
        23 => Event::Unitalic,
        24 => Event::Ununderline,
        25 => Event::Unblink,
        27 => Event::Unreverse,
        28 => Event::Unconceal,
        29 => Event::Unstrike,
        30..=37 | 90..=97 => Event::Foreground(AnsiColor::Color8(Color8::from_u8(value))),
        39 => Event::DefaultForeground,
        40..=47 | 100..=107 => Event::Background(AnsiColor::Color8(Color8::from_u8(value))),
        49 => Event::DefaultBackground,
        59 => Event::DefaultUnderline,
        _ => Event::Unknown(value),
    }
}

fn get_sgr_color(value: u8) -> Event {
    match value {
        30..=37 | 90..=97 => Event::Foreground(AnsiColor::Color8(Color8::from_u8(value))),
        40..=47 | 100..=107 => Event::Background(AnsiColor::Color8(Color8::from_u8(value))),
        39 => Event::DefaultForeground,
        49 => Event::DefaultBackground,
        _ => Event::Unknown(0),
    }
}
