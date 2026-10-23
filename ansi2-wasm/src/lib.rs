use ansi2::{LengthAdjust, Mode, Options, Theme};
use wasm_bindgen::prelude::wasm_bindgen;

#[allow(clippy::too_many_arguments)]
fn options(
    theme: Theme,
    width: Option<usize>,
    font: Option<String>,
    mode: Option<Mode>,
    light_bg: Option<String>,
    dark_bg: Option<String>,
    font_size: Option<usize>,
    length_adjust: Option<String>,
    sourcemap: Option<bool>,
) -> Options {
    Options {
        theme,
        mode,
        width,
        font,
        font_size: font_size.unwrap_or(16),
        light_bg,
        dark_bg,
        length_adjust: LengthAdjust::parse(length_adjust.as_deref()),
        sourcemap: sourcemap.unwrap_or(false),
    }
}

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn to_svg(
    s: String,
    theme: Theme,
    width: Option<usize>,
    font: Option<String>,
    mode: Option<Mode>,
    light_bg: Option<String>,
    dark_bg: Option<String>,
    font_size: Option<usize>,
    length_adjust: Option<String>,
    sourcemap: Option<bool>,
) -> String {
    ansi2::to_svg(
        &s,
        &options(
            theme,
            width,
            font,
            mode,
            light_bg,
            dark_bg,
            font_size,
            length_adjust,
            sourcemap,
        ),
    )
}

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn to_html(
    s: String,
    theme: Theme,
    width: Option<usize>,
    font: Option<String>,
    mode: Option<Mode>,
    light_bg: Option<String>,
    dark_bg: Option<String>,
    font_size: Option<usize>,
    sourcemap: Option<bool>,
) -> String {
    ansi2::to_html(
        &s,
        &options(
            theme, width, font, mode, light_bg, dark_bg, font_size, None, sourcemap,
        ),
    )
}

#[wasm_bindgen]
pub fn to_text(s: String, width: Option<usize>) -> String {
    ansi2::to_text(&s, width)
}

#[wasm_bindgen]
pub fn to_ans(s: String, width: Option<usize>) -> String {
    ansi2::to_ans(&s, width)
}
