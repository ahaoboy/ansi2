use ansi2::theme::Theme;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_svg(s: String, theme: String) -> String {
    let theme: Theme = theme.as_str().into();
    ansi2svg::to_svg(&s, theme)
}

#[wasm_bindgen]
pub fn to_html(s: String, theme: String) -> String {
    let theme: Theme = theme.as_str().into();
    ansi2html::to_html(&s, theme)
}
