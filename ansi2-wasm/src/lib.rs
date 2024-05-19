use ansi2::theme::Theme;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_svg(s: String, theme: String, width: Option<usize>) -> String {
    let theme: Theme = theme.as_str().into();
    ansi2::svg::to_svg(&s, theme, width)
}

#[wasm_bindgen]
pub fn to_html(s: String, theme: String, width: Option<usize>) -> String {
    let theme: Theme = theme.as_str().into();
    ansi2::html::to_html(&s, theme, width)
}
