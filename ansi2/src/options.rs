use crate::theme::Theme;

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Dark,
    Light,
}

/// Controls the `textLength` / `lengthAdjust` attributes of SVG text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LengthAdjust {
    /// Only emit `textLength` (the default).
    #[default]
    Auto,
    Spacing,
    SpacingAndGlyphs,
    /// Do not emit either attribute.
    Disabled,
}

impl LengthAdjust {
    /// Mirrors the historical CLI behaviour: unknown or empty values disable
    /// the attributes.
    pub fn parse(value: Option<&str>) -> Self {
        match value {
            None => Self::Auto,
            Some("spacing") => Self::Spacing,
            Some("spacingAndGlyphs") => Self::SpacingAndGlyphs,
            Some(_) => Self::Disabled,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    pub theme: Theme,
    pub mode: Option<Mode>,
    pub width: Option<usize>,
    pub font: Option<String>,
    pub font_size: usize,
    pub light_bg: Option<String>,
    pub dark_bg: Option<String>,
    pub length_adjust: LengthAdjust,
    pub sourcemap: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            mode: None,
            width: None,
            font: None,
            font_size: 16,
            light_bg: None,
            dark_bg: None,
            length_adjust: LengthAdjust::default(),
            sourcemap: false,
        }
    }
}
