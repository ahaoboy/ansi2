use crate::{
    css::{CssType, Mode, NodeStyle, Style},
    theme::ColorTable,
    Canvas,
};
#[allow(clippy::too_many_arguments)]
pub fn to_svg<S: AsRef<str>, T: ColorTable>(
    str: S,
    theme: T,
    width: Option<usize>,
    font: Option<String>,
    mode: Option<Mode>,
    light_bg: Option<String>,
    dark_bg: Option<String>,
    font_size: Option<usize>,
    length_adjust: Option<String>,
) -> String {
    let font_size = font_size.unwrap_or(16);
    let s = str.as_ref();
    let canvas = Canvas::new(s, width);
    let mut s = String::new();
    let mut cur_x = 0;
    // FIXME: for better alignment
    let fn_w = font_size * 5 / 8;
    let fn_h = font_size;
    let baseline_h = font_size / 2;
    let underline_h = font_size / 8;
    let text_h = fn_h + underline_h;

    let mut cur_y = 0;
    let mut style = Style::default();
    let mut font_style = "".into();
    let mut font_family = "Consolas,Courier New,Monaco".into();

    if let Some(url) = font {
        if url.starts_with("http") || url.starts_with("data:font;base64") {
            font_family = "ansi2-custom-font".into();
            font_style =
                format!(r#"@font-face {{font-family: ansi2-custom-font;src: url({url});}}"#)
        } else {
            font_family = url;
        }
    }

    for row in canvas.minify().iter() {
        for c in row.iter() {
            let mut text_class = vec![NodeStyle::Text.class_name()];
            let str_w = fn_w * c.text.chars().count();
            // FIXME: baseline offset
            let text_x = cur_x;
            let text_y = cur_y + baseline_h + underline_h;

            if !c.bg_color.is_default() {
                let name = c.bg_color.bg_class_name();
                let class_str = format!("class='{}'", name);
                s.push_str(&format!(
                    r#"<rect x="{cur_x}px" y="{}px" width="{str_w}px" height="{text_h}px" {class_str}/>"#,
                    cur_y+underline_h
                ));
                style.add_bg(c.bg_color);
            }

            if !c.color.is_default() {
                let name = c.color.class_name();
                text_class.push(name);
                style.add_color(c.color);
            };

            let mut attr = vec![];

            if c.bold {
                text_class.push(NodeStyle::Bold.class_name());
                style.bold = true;
            }
            if c.blink {
                text_class.push(NodeStyle::Blink.class_name());
                style.blink = true;
            }

            if c.italic {
                attr.push("font-style=\"italic\"");
            }
            if c.dim {
                attr.push("opacity=\"0.5\"");
            }
            if c.underline {
                attr.push("text-decoration=\"underline\"");
            }

            let class_str = if text_class.is_empty() {
                String::new()
            } else {
                format!("class='{}'", text_class.join(" "))
            };

            // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/lengthAdjust
            let length_adjust_style = match length_adjust {
                Some(ref s) => {
                    if s == "spacingAndGlyphs" || s == "spacing" {
                        format!("lengthAdjust=\"{s}\" textLength=\"{str_w}\"")
                    } else {
                        "".to_string()
                    }
                }
                None => format!("textLength=\"{str_w}\""),
            };

            // FIXME: lengthAdjust="spacingAndGlyphs" or lengthAdjust="spacing"
            s.push_str(&format!(
r#"<text x="{text_x}px" y="{text_y}px" width="{str_w}px" height="{text_h}px" {} {}><tspan {length_adjust_style}>{}</tspan></text>"#,
class_str ,
attr.join(" "),
                html_escape::encode_text(&c.text)
            ));
            cur_x += str_w;
        }
        cur_y += fn_h + underline_h;
        cur_x = 0;
    }

    let svg_w = fn_w * canvas.w;
    let svg_h = (fn_h + underline_h) * canvas.h;

    let style_css = style.to_css(
        theme,
        CssType::Svg,
        mode,
        light_bg,
        dark_bg,
        font_family,
        fn_h,
    );
    format!(
        r#"<svg width="{svg_w}px" height="{svg_h}px" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><style>{font_style}{style_css}</style>{s}</svg>"#
    )
}
