use std::fmt::Write;

use crate::{
    cell::Cell,
    color::{AnsiColor, Color8},
    options::{Mode, Options},
    theme::Theme,
};

pub const DEFAULT_FONTS: &str = "Consolas,Courier New,Monaco,monospace";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssType {
    Svg,
    Html,
}

#[derive(Debug, Clone, Copy)]
pub enum NodeStyle {
    Bold,
    Blink,
    Dim,
    Italic,
    Underline,
    Hide,
    Row,
    Text,
    Main,
    Strike,
}

impl NodeStyle {
    pub const fn class(self) -> &'static str {
        match self {
            Self::Bold => "b",
            Self::Blink => "B",
            Self::Dim => "d",
            Self::Italic => "i",
            Self::Underline => "u",
            Self::Hide => "h",
            Self::Row => "r",
            Self::Text => "t",
            Self::Main => "m",
            Self::Strike => "s",
        }
    }
}

/// The CSS rules that are actually needed by the rendered document.
#[derive(Debug, Default)]
pub struct Stylesheet {
    colors: Vec<AnsiColor>,
    bg_colors: Vec<AnsiColor>,
    pub bold: bool,
    pub blink: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub hide: bool,
    pub strike: bool,
}

impl Stylesheet {
    pub fn add_color(&mut self, color: AnsiColor) {
        if !self.colors.contains(&color) {
            self.colors.push(color);
        }
    }

    pub fn add_bg_color(&mut self, color: AnsiColor) {
        if !self.bg_colors.contains(&color) {
            self.bg_colors.push(color);
        }
    }

    pub fn to_css(
        &self,
        options: &Options,
        font: &Font,
        font_size: usize,
        target: CssType,
    ) -> String {
        let mut css = String::new();
        self.push_base(&mut css, target, &font.family, font_size);
        self.push_flags(&mut css);

        let (color_property, background_property) = match target {
            CssType::Html => ("color", "background"),
            CssType::Svg => ("fill", "fill"),
        };

        let light_colors = self.colors_css(options.theme, color_property, false);
        let light_backgrounds = self.bg_colors_css(options.theme, background_property, false);
        let dark_colors = self.colors_css(options.theme, color_property, true);
        let dark_backgrounds = self.bg_colors_css(options.theme, background_property, true);

        let dark_bg = options.dark_bg.as_deref().unwrap_or("#181818");
        let light_bg = options.light_bg.as_deref().unwrap_or("#FFFFFF");

        if let Some(mode) = options.mode {
            let (colors, backgrounds, background) = match mode {
                Mode::Dark => (dark_colors, dark_backgrounds, dark_bg),
                Mode::Light => (light_colors, light_backgrounds, light_bg),
            };
            push_default_text_color(&mut css, options.theme, target, mode);
            let _ = write!(css, ":root{{background:{background}}}");
            css.push_str(&colors);
            css.push_str(&backgrounds);
            return css;
        }

        let _ = write!(
            css,
            ":root {{color-scheme: light dark; background: {light_bg}}}"
        );
        css.push_str(&light_colors);
        css.push_str(&light_backgrounds);
        push_default_text_color(&mut css, options.theme, target, Mode::Light);
        let _ = write!(
            css,
            "@media (prefers-color-scheme: dark) {{:root {{background: {dark_bg}}}{dark_colors}{dark_backgrounds}"
        );
        push_default_text_color(&mut css, options.theme, target, Mode::Dark);
        css.push('}');

        css
    }

    fn push_base(&self, css: &mut String, target: CssType, font_family: &str, font_size: usize) {
        match target {
            CssType::Svg => {
                let _ = write!(
                    css,
                    ".t{{dominant-baseline:central;font-variant-ligatures:none;white-space: pre;font-family:{font_family};font-size:{font_size}px;}}"
                );
            }
            CssType::Html => {
                let _ = write!(css, ".m{{display:flex;flex-direction:column;}}");
                let _ = write!(css, ".r{{display:flex;}}");
                let _ = write!(
                    css,
                    ".t{{margin:0;padding:0;font-family:{font_family};white-space:pre;display:inline-block;font-size:{font_size}px}}"
                );
            }
        }
    }

    fn push_flags(&self, css: &mut String) {
        if self.bold {
            let _ = write!(css, ".b{{font-weight:bold}}");
        }
        if self.hide {
            let _ = write!(css, ".h{{opacity:0}}");
        }
        if self.dim {
            let _ = write!(css, ".d{{font-weight:lighter;opacity:0.5}}");
        }
        if self.italic {
            let _ = write!(css, ".i{{font-style:italic}}");
        }
        if self.underline {
            let _ = write!(css, ".u{{text-decoration:underline}}");
        }
        if self.strike {
            let _ = write!(css, ".s{{text-decoration:line-through}}");
        }
        if self.blink {
            let _ = write!(
                css,
                ".B{{animation:bk 1s steps(1, end) infinite;}} @keyframes bk{{50% {{opacity: 0}}}}"
            );
        }
    }

    fn colors_css(&self, theme: Theme, property: &str, dark: bool) -> String {
        let mut css = String::new();
        for color in &self.colors {
            let hex = if dark {
                dark_hex(*color, theme)
            } else {
                color.hex(theme)
            };
            let _ = write!(css, ".{}{{{property}:{hex}}}", color.class());
        }
        css
    }

    fn bg_colors_css(&self, theme: Theme, property: &str, dark: bool) -> String {
        let mut css = String::new();
        for color in &self.bg_colors {
            let hex = if dark {
                dark_hex(*color, theme)
            } else {
                color.hex(theme)
            };
            let _ = write!(css, ".{}{{{property}:{hex}}}", color.bg_class());
        }
        css
    }
}

fn dark_hex(color: AnsiColor, theme: Theme) -> String {
    match color {
        AnsiColor::Color8(color) => match color {
            Color8::Black => Color8::White.hex(theme),
            Color8::White => Color8::Black.hex(theme),
            Color8::BrightBlack => Color8::BrightWhite.hex(theme),
            Color8::BrightWhite => Color8::BrightBlack.hex(theme),
            _ => color.hex(theme),
        },
        _ => color.hex(theme),
    }
}

fn push_default_text_color(css: &mut String, theme: Theme, target: CssType, mode: Mode) {
    let hex = match mode {
        Mode::Dark => Color8::White.hex(theme),
        Mode::Light => Color8::Black.hex(theme),
    };
    match target {
        CssType::Svg => {
            let _ = write!(css, "svg text{{fill:{hex}}}");
        }
        CssType::Html => {
            let _ = write!(css, "div{{color:{hex}}}");
        }
    }
}

pub struct Font {
    pub family: String,
    /// A `@font-face` rule, if the font is embedded.
    pub face: String,
}

impl Font {
    pub fn new(font: Option<&str>) -> Self {
        match font {
            Some(url) if url.starts_with("http") || url.starts_with("data:") => Self {
                family: "ansi2-custom-font".into(),
                face: format!("@font-face {{font-family: ansi2-custom-font;src: url({url});}}"),
            },
            Some(family) => Self {
                family: family.into(),
                face: String::new(),
            },
            None => Self {
                family: DEFAULT_FONTS.into(),
                face: String::new(),
            },
        }
    }
}

/// Build the CSS classes and SVG-only attributes for a single cell.
pub struct Markup {
    pub classes: String,
    pub attrs: String,
}

pub fn markup(cell: &Cell, target: CssType, sourcemap: bool, sheet: &mut Stylesheet) -> Markup {
    let mut classes = vec![NodeStyle::Text.class().to_string()];
    let mut attrs = Vec::new();

    if target == CssType::Html {
        if cell.style.bold {
            classes.push(NodeStyle::Bold.class().into());
            sheet.bold = true;
        }
        if cell.style.italic {
            classes.push(NodeStyle::Italic.class().into());
            sheet.italic = true;
        }
        if cell.style.dim {
            classes.push(NodeStyle::Dim.class().into());
            sheet.dim = true;
        }
        if cell.style.underline {
            classes.push(NodeStyle::Underline.class().into());
            sheet.underline = true;
        }
        if cell.style.hide {
            classes.push(NodeStyle::Hide.class().into());
            sheet.hide = true;
        }
        if cell.style.blink {
            classes.push(NodeStyle::Blink.class().into());
            sheet.blink = true;
        }
        if cell.style.strike {
            classes.push(NodeStyle::Strike.class().into());
            sheet.strike = true;
        }
        if !cell.style.fg.is_default() {
            classes.push(cell.style.fg.class());
            sheet.add_color(cell.style.fg);
        }
        if !cell.style.bg.is_default() {
            classes.push(cell.style.bg.bg_class());
            sheet.add_bg_color(cell.style.bg);
        }
    } else {
        if !cell.style.fg.is_default() {
            classes.push(cell.style.fg.class());
            sheet.add_color(cell.style.fg);
        }
        if cell.style.bold {
            classes.push(NodeStyle::Bold.class().into());
            sheet.bold = true;
        }
        if cell.style.blink {
            classes.push(NodeStyle::Blink.class().into());
            sheet.blink = true;
        }
        if cell.style.italic {
            attrs.push("font-style=\"italic\"");
        }
        if cell.style.dim {
            attrs.push("opacity=\"0.5\"");
        }
        if cell.style.underline {
            classes.push(NodeStyle::Underline.class().into());
            sheet.underline = true;
        }
        if cell.style.strike {
            classes.push(NodeStyle::Strike.class().into());
            sheet.strike = true;
        }
        if cell.style.hide {
            classes.push(NodeStyle::Hide.class().into());
            sheet.hide = true;
        }
    }

    if sourcemap {
        classes.extend(cell.src.classes());
    }

    Markup {
        classes: classes.join(" "),
        attrs: attrs.join(" "),
    }
}
