use crate::{
    color::get_hex,
    color::{AnsiColor, Color8},
    theme::ColorTable,
};

#[derive(Debug, Clone, Copy)]
pub enum CssType {
    Svg,
    Html,
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Dark,
    Light,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Default)]
pub struct Style {
    // FIXME: Hashset order is different in wasm
    pub colors: Vec<AnsiColor>,
    pub bg_colors: Vec<AnsiColor>,
    pub bold: bool,
    pub blink: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub hide: bool,
    pub strike: bool,
}

impl NodeStyle {
    pub fn class_name(&self) -> &'static str {
        match self {
            NodeStyle::Bold => "b",
            NodeStyle::Blink => "B",
            NodeStyle::Dim => "d",
            NodeStyle::Italic => "i",
            NodeStyle::Underline => "u",
            NodeStyle::Hide => "h",
            NodeStyle::Row => "r",
            NodeStyle::Text => "t",
            NodeStyle::Main => "m",
            NodeStyle::Strike => "s",
        }
    }
}

impl Style {
    pub fn add_color(&mut self, c: AnsiColor) {
        if !self.colors.contains(&c) {
            self.colors.push(c);
        }
    }
    pub fn add_bg_color(&mut self, c: AnsiColor) {
        if !self.bg_colors.contains(&c) {
            self.bg_colors.push(c);
        }
    }
    pub fn to_css<T: ColorTable>(
        &self,
        theme: T,
        ty: CssType,
        mode: Option<Mode>,
        light_bg: Option<String>,
        dark_bg: Option<String>,
        font_family: String,
        font_size: usize,
    ) -> String {
        let mut css = String::new();
        match ty {
            CssType::Svg => {
                let text_style = format!(
                    r#".{}{{dominant-baseline:central;font-variant-ligatures:none;white-space: pre;font-family:{font_family};font-size:{font_size}px;}}"#,
                    NodeStyle::Text.class_name()
                );

                css.push_str(&text_style);
            }
            CssType::Html => {
                let main_style = format!(
                    ".{}{{display:flex;flex-direction:column;}}",
                    NodeStyle::Main.class_name()
                );
                css.push_str(&main_style);
                let row_style = format!(".{}{{display:flex;}}", NodeStyle::Row.class_name());
                css.push_str(&row_style);

                let text_style = format!(
                  ".{}{{margin:0;padding:0;font-family:{font_family};white-space:pre;display:inline-block;font-size:{font_size}px}}",
                  NodeStyle::Text.class_name()
              );
                css.push_str(&text_style);
            }
        }

        if self.bold {
            css.push_str(&format!(
                ".{}{{font-weight:bold}}",
                NodeStyle::Bold.class_name()
            ));
        }
        if self.hide {
            css.push_str(&format!(".{}{{opacity:0}}", NodeStyle::Hide.class_name()));
        }
        if self.dim {
            css.push_str(&format!(
                ".{}{{font-weight:lighter;opacity:0.5}}",
                NodeStyle::Dim.class_name()
            ));
        }
        if self.italic {
            css.push_str(&format!(
                ".{}{{font-style:italic}}",
                NodeStyle::Italic.class_name()
            ));
        }
        if self.underline {
            css.push_str(&format!(
                ".{}{{text-decoration:underline}}",
                NodeStyle::Underline.class_name()
            ));
        }
        if self.strike {
            css.push_str(&format!(
                ".{}{{text-decoration:line-through}}",
                NodeStyle::Strike.class_name()
            ));
        }
        if self.blink {
            css.push_str(&format!(
                ".{}{{animation:bk 1s steps(1, end) infinite;}} @keyframes bk{{50% {{opacity: 0}}}}",
                NodeStyle::Blink.class_name()
            ));
        }

        let dark_bg_color = dark_bg.unwrap_or("#181818".to_string());
        let light_bg_color = light_bg.unwrap_or("#FFFFFF".to_string());

        let (color_field, bg_field) = match ty {
            CssType::Html => ("color", "background"),
            CssType::Svg => ("fill", "fill"),
        };

        let light_color_css: String = self.colors.iter().fold(String::new(), |mut acc, c| {
            acc.push_str(&format!(
                ".{}{{{color_field}:{}}}",
                c.class_name(),
                c.get_hex(theme)
            ));
            acc
        });

        let bg_light_color_css: String = self.bg_colors.iter().fold(String::new(), |mut acc, c| {
            acc.push_str(&format!(
                ".{}{{{bg_field}:{}}}",
                c.bg_class_name(),
                c.get_hex(theme)
            ));
            acc
        });

        let dark_color_css: String = self.colors.iter().fold(String::new(), |mut acc, c| {
            let hex = match c {
                AnsiColor::Color8(color8) => match color8 {
                    Color8::Black => Color8::White.get_hex(theme),
                    Color8::White => Color8::Black.get_hex(theme),
                    Color8::BrightBlack => Color8::BrightWhite.get_hex(theme),
                    Color8::BrightWhite => Color8::BrightBlack.get_hex(theme),
                    _ => color8.get_hex(theme),
                },
                _ => c.get_hex(theme),
            };
            acc.push_str(&format!(".{}{{{color_field}:{}}}", c.class_name(), hex));
            acc
        });

        let bg_dark_color_css: String = self.bg_colors.iter().fold(String::new(), |mut acc, c| {
            let hex = match c {
                AnsiColor::Color8(color8) => match color8 {
                    Color8::Black => Color8::White.get_hex(theme),
                    Color8::White => Color8::Black.get_hex(theme),
                    Color8::BrightBlack => Color8::BrightWhite.get_hex(theme),
                    Color8::BrightWhite => Color8::BrightBlack.get_hex(theme),
                    _ => color8.get_hex(theme),
                },
                _ => c.get_hex(theme),
            };
            acc.push_str(&format!(".{}{{{bg_field}:{}}}", c.bg_class_name(), hex));
            acc
        });

        if let Some(mode) = mode {
            let default_text_style = match (mode, ty) {
                (Mode::Dark, CssType::Html) => {
                    format!("div{{color: {} }}", get_hex(theme.white()))
                }
                (Mode::Dark, CssType::Svg) => {
                    format!("svg > text{{fill:{}}}", get_hex(theme.white()))
                }
                (Mode::Light, CssType::Html) => {
                    format!("div{{color:{}}}", get_hex(theme.black()))
                }
                (Mode::Light, CssType::Svg) => {
                    format!("svg > text{{fill:{}}}", get_hex(theme.black()))
                }
            };

            css.push_str(&default_text_style);

            let (color_css, bg_color_css) = match mode {
                Mode::Dark => (dark_color_css, bg_dark_color_css),
                Mode::Light => (light_color_css, bg_light_color_css),
            };

            let root_style = match mode {
                Mode::Dark => format!(":root{{background:{dark_bg_color}}}"),
                Mode::Light => format!(":root{{background:{light_bg_color}}}"),
            };

            css.push_str(&root_style);
            css.push_str(&color_css);
            css.push_str(&bg_color_css);
            return css;
        }

        let default_light_text_style = match ty {
            CssType::Svg => format!("svg > text{{fill:{}}}", get_hex(theme.black())),
            CssType::Html => format!("div{{color:{}}}", get_hex(theme.black())),
        };

        let default_dark_text_style = match ty {
            CssType::Svg => format!("svg > text{{fill:{}}}", get_hex(theme.white())),
            CssType::Html => format!("div{{color:{}}}", get_hex(theme.white())),
        };

        let root_css = format!(
            r#":root {{color-scheme: light dark; background: {light_bg_color}}}{light_color_css}{bg_light_color_css}{default_light_text_style}"#
        );

        let dark_css = format!(
            r#"@media (prefers-color-scheme: dark) {{:root {{background: {dark_bg_color}}}{dark_color_css}{bg_dark_color_css}{default_dark_text_style}}}"#)
        .trim()
        .to_string();

        css.push_str(&root_css);
        css.push_str(&dark_css);

        css
    }
}
