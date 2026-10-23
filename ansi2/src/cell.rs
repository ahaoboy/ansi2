use crate::{color::AnsiColor, span::Span};

/// Visual attributes shared by a run of characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Style {
    pub fg: AnsiColor,
    pub bg: AnsiColor,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub hide: bool,
    pub strike: bool,
}

/// Source positions for each attribute of a [`Cell`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SourceMap {
    pub text: Span,
    pub fg: Span,
    pub bg: Span,
    pub bold: Span,
    pub dim: Span,
    pub italic: Span,
    pub underline: Span,
    pub blink: Span,
    pub hide: Span,
    pub strike: Span,
}

impl SourceMap {
    /// Reset every attribute range except `text`.
    pub fn set_all(&mut self, span: Span) {
        self.fg = span;
        self.bg = span;
        self.bold = span;
        self.dim = span;
        self.italic = span;
        self.underline = span;
        self.blink = span;
        self.hide = span;
        self.strike = span;
    }

    pub fn classes(&self) -> impl Iterator<Item = String> + '_ {
        [
            ("text", self.text),
            ("color", self.fg),
            ("bg", self.bg),
            ("bold", self.bold),
            ("blink", self.blink),
            ("dim", self.dim),
            ("italic", self.italic),
            ("underline", self.underline),
            ("hide", self.hide),
            ("strike", self.strike),
        ]
        .into_iter()
        .map(|(name, span)| span.class(name))
    }
}

/// A single character cell on a [`Canvas`](crate::canvas::Canvas).
#[derive(Debug, Clone, Default)]
pub struct Cell {
    pub style: Style,
    pub text: String,
    pub src: SourceMap,
}

impl Cell {
    pub fn new(style: Style, text: impl Into<String>, src: SourceMap) -> Self {
        Self {
            style,
            text: text.into(),
            src,
        }
    }

    /// A padding cell (a rendered space).
    pub fn blank() -> Self {
        Self {
            text: " ".into(),
            ..Self::default()
        }
    }

    pub fn same_style(&self, other: &Self) -> bool {
        self.style == other.style
    }
}

// Source positions do not affect equality; they are only metadata.
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.style == other.style && self.text == other.text
    }
}

impl Eq for Cell {}
