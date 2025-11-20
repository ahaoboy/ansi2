use crate::{
    canvas::Canvas,
    css::{CssType, DEFAULT_FONTS, Mode, NodeStyle, Style},
    theme::ColorTable,
};

#[allow(clippy::too_many_arguments)]
pub fn to_html<S: AsRef<str>>(
    str: S,
    theme: impl ColorTable,
    width: Option<usize>,
    font: Option<String>,
    mode: Option<Mode>,
    light_bg: Option<String>,
    dark_bg: Option<String>,
    font_size: Option<usize>,
    sourcemap: bool,
) -> String {
    let font_size = font_size.unwrap_or(16);
    let s = str.as_ref();
    let canvas = Canvas::new(s, width);
    let mut s = String::new();

    let mut style = Style::default();

    let mut font_style = "".into();
    let mut font_family = DEFAULT_FONTS.into();

    if let Some(url) = font {
        if url.starts_with("http") || url.starts_with("data:font;base64") {
            font_family = "ansi2-custom-font".into();
            font_style =
                format!(r#"@font-face {{font-family: ansi2-custom-font;src: url({url});}}"#)
        } else {
            font_family = url;
        }
    }

    s.push_str(&format!("<div class='{}'>", NodeStyle::Main.class_name()));

    let row_style = format!("<div class='{}'>", NodeStyle::Row.class_name());
    for row in canvas.minify().iter() {
        s.push_str(&row_style);
        for c in row.iter() {
            let mut text_class = vec![NodeStyle::Text.class_name().to_string()];
            if c.bold {
                text_class.push(NodeStyle::Bold.class_name().to_string());
                style.bold = true;
            }
            if c.italic {
                text_class.push(NodeStyle::Italic.class_name().to_string());
                style.italic = true;
            }
            if c.dim {
                text_class.push(NodeStyle::Dim.class_name().to_string());
                style.dim = true;
            }
            if c.underline {
                text_class.push(NodeStyle::Underline.class_name().to_string());
                style.underline = true;
            }
            if c.hide {
                text_class.push(NodeStyle::Hide.class_name().to_string());
                style.hide = true;
            }
            if c.blink {
                text_class.push(NodeStyle::Blink.class_name().to_string());
                style.blink = true;
            }
            if c.strike {
                text_class.push(NodeStyle::Strike.class_name().to_string());
                style.strike = true;
            }
            if !c.color.is_default() {
                let name = c.color.class_name();
                text_class.push(name);
                style.add_color(c.color);
            }

            if !c.bg_color.is_default() {
                let name = c.bg_color.bg_class_name();
                text_class.push(name);
                style.add_bg_color(c.bg_color);
            }

            if sourcemap {
                text_class.push(format!("text:{}:{}", c.text_r.0, c.text_r.1));
                text_class.push(format!("color:{}:{}", c.color_r.0, c.color_r.1));
                text_class.push(format!("bg:{}:{}", c.bg_color_r.0, c.bg_color_r.1));
                text_class.push(format!("bold:{}:{}", c.bold_r.0, c.bold_r.1));
                text_class.push(format!("blink:{}:{}", c.blink_r.0, c.blink_r.1));
                text_class.push(format!("dim:{}:{}", c.dim_r.0, c.dim_r.1));
                text_class.push(format!("italic:{}:{}", c.italic_r.0, c.italic_r.1));
                text_class.push(format!("underline:{}:{}", c.underline_r.0, c.underline_r.1));
                text_class.push(format!("hide:{}:{}", c.hide_r.0, c.hide_r.1));
                text_class.push(format!("strike:{}:{}", c.strike_r.0, c.strike_r.1));
            }

            let text_class = text_class.join(" ").trim().to_string();
            let html_char = c.text.to_string();
            let html_char = html_escape::encode_text(&html_char);
            let class_str = if text_class.is_empty() {
                String::new()
            } else {
                format!("class='{text_class}'")
            };
            s.push_str(&format!("<p {class_str}>{html_char}</p>",))
        }

        if row.is_empty() {
            s.push_str("<br>");
        }
        // .row
        s.push_str("</div>");
    }

    // .ansi-main
    s.push_str("</div>");

    let style_css = style.to_css(
        theme,
        CssType::Html,
        mode,
        light_bg,
        dark_bg,
        font_family,
        font_size,
    );
    format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><style>{font_style}{style_css}</style></head><body>{s}</body></html>"#
    )
}
