use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Vscode,
    Ubuntu,
    Vga,
}

impl From<Theme> for ansi2::theme::Theme {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Vscode => ansi2::theme::Theme::Vscode,
            Theme::Ubuntu => ansi2::theme::Theme::Ubuntu,
            Theme::Vga => ansi2::theme::Theme::Vga,
        }
    }
}

#[wasm_bindgen]
pub fn to_svg(s: String, theme: Theme, width: Option<usize>, font: Option<String>) -> String {
    ansi2::svg::to_svg(&s, Into::<ansi2::theme::Theme>::into(theme), width, font)
}

#[wasm_bindgen]
pub fn to_html(s: String, theme: Theme, width: Option<usize>, font: Option<String>) -> String {
    ansi2::html::to_html(&s, Into::<ansi2::theme::Theme>::into(theme), width, font)
}

#[wasm_bindgen]
pub fn to_text(s: String, width: Option<usize>) -> String {
    ansi2::text::to_text(&s, width)
}
