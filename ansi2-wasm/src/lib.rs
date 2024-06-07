use ansi2::theme::Theme;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_svg(s: String, theme: String, width: Option<usize>, font: Option<String>) -> String {
    let theme: Theme = theme.as_str().into();
    ansi2::svg::to_svg(&s, theme, width, font)
}

#[wasm_bindgen]
pub fn to_html(s: String, theme: String, width: Option<usize>, font: Option<String>) -> String {
    let theme: Theme = theme.as_str().into();
    ansi2::html::to_html(&s, theme, width, font)
}
