use std::fmt::Write;

use crate::{
    canvas::Canvas,
    css::{CssType, Font, Markup, NodeStyle, Stylesheet, markup},
    options::Options,
};

pub fn to_html(input: &str, options: &Options) -> String {
    let canvas = Canvas::new(input, options.width);
    let font = Font::new(options.font.as_deref());
    let mut sheet = Stylesheet::default();
    let mut body = String::new();

    body.push_str(&format!("<div class='{}'>", NodeStyle::Main.class()));

    for row in canvas.minify() {
        body.push_str(&format!("<div class='{}'>", NodeStyle::Row.class()));
        for cell in &row {
            let Markup { classes, .. } = markup(cell, CssType::Html, options.sourcemap, &mut sheet);
            let _ = write!(body, "<p class='{classes}'>");
            body.push_str(&html_escape::encode_text(&cell.text));
            body.push_str("</p>");
        }
        if row.is_empty() {
            body.push_str("<br>");
        }
        body.push_str("</div>");
    }

    body.push_str("</div>");

    let css = sheet.to_css(options, &font, options.font_size, CssType::Html);
    format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><style>{}{css}</style></head><body>{body}</body></html>"#,
        font.face
    )
}
