use ansi2::{css::Mode, theme::Theme};
use wasm_bindgen::prelude::wasm_bindgen;

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
) -> String {
    let mode = mode.map(|m| match m {
        Mode::Dark => ansi2::css::Mode::Dark,
        Mode::Light => ansi2::css::Mode::Light,
    });
    ansi2::svg::to_svg(
        &s,
        Into::<ansi2::theme::Theme>::into(theme),
        width,
        font,
        mode,
        light_bg,
        dark_bg,
        font_size,
        length_adjust,
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
) -> String {
    let mode = mode.map(|m| match m {
        Mode::Dark => ansi2::css::Mode::Dark,
        Mode::Light => ansi2::css::Mode::Light,
    });
    ansi2::html::to_html(
        &s,
        Into::<ansi2::theme::Theme>::into(theme),
        width,
        font,
        mode,
        light_bg,
        dark_bg,
        font_size,
    )
}

#[wasm_bindgen]
pub fn to_text(s: String, width: Option<usize>) -> String {
    ansi2::text::to_text(&s, width)
}
