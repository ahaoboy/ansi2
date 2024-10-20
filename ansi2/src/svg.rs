use std::collections::HashSet;

use crate::{
    css::{get_hex, to_style, CssType, Mode},
    theme::ColorTable,
    Canvas,
};

pub fn to_svg<S: AsRef<str>, T: ColorTable>(
    str: S,
    theme: T,
    width: Option<usize>,
    font: Option<String>,
    mode: Option<Mode>,
    light_bg: Option<String>,
    dark_bg: Option<String>,
) -> String {
    let s = str.as_ref();
    let canvas = Canvas::new(s, width);
    let mut s = String::new();
    let mut cur_x = 0;
    let fn_w = 20;
    let fn_h = 32;
    let baseline_h = 16;
    let mut cur_y = 0;
    let style = to_style(theme, CssType::Svg, mode, light_bg, dark_bg);
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

    let mut color256 = HashSet::new();

    for row in canvas.pixels.iter() {
        for c in row.iter() {
            let mut text_class = vec![];

            if !c.bg_color.is_default() {
                let name = "bg-".to_string() + &c.bg_color.class_name();

                let class_str = format!(" class='{}'", name);
                s.push_str(&format!(
                    r#"<rect x="{cur_x}px" y="{cur_y}px" width="{fn_w}px" height="{fn_h}px" {class_str}/>"#
                ));

                if let crate::lex::AnsiColor::Rgb(r, g, b) = c.color {
                    color256.insert(format!(
                        ".bg-rgb_{r}_{g}_{b}{{fill:{};}}\n",
                        get_hex((r, g, b))
                    ));
                }
            }

            if !c.color.is_default() {
                let name = c.color.class_name();
                text_class.push(name);

                if let crate::lex::AnsiColor::Rgb(r, g, b) = c.color {
                    color256.insert(format!(
                        ".rgb_{r}_{g}_{b}{{fill:{};}}\n",
                        get_hex((r, g, b))
                    ));
                }
            };

            let mut italic_str = "";
            let mut dim_str = "";
            let mut underline_str = "";
            if c.bold {
                text_class.push("bold".into());
            }
            if c.blink {
                text_class.push("blink".into());
            }

            if c.italic {
                text_class.push("italic".into());
                italic_str = "font-style=\"italic\"";
            }
            if c.dim {
                text_class.push("dim".into());
                dim_str = "opacity=\"0.5\"";
            }
            if c.underline {
                text_class.push("underline".into());
                underline_str = "text-decoration=\"underline\"";
            }

            // baseline offset
            let text_x = cur_x;
            let text_y = cur_y + baseline_h;
            let class_str = if text_class.is_empty() {
                String::new()
            } else {
                format!("class='{}'", text_class.join(" "))
            };

            s.push_str(&format!(
r#"<text x="{text_x}px" y="{text_y}px" width="{fn_w}px" height="{fn_h}px" {} {italic_str} {dim_str} {underline_str}><tspan>{}</tspan></text>"#,
class_str ,
                html_escape::encode_text(&c.char.to_string())
            ));
            cur_x += fn_w;
        }
        cur_y += fn_h;
        cur_x = 0;
    }

    let svg_w = fn_w * canvas.w;
    let svg_h = fn_h * canvas.h;

    let color256_str: String = color256.into_iter().collect();

    format!(
        r#"<svg width="{svg_w}px" height="{svg_h}px" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
<style>
tspan {{
font-variant-ligatures: none;
dominant-baseline: central;
font-variant-ligatures: none;
white-space: pre;
font-family: {font_family};
font-size: {fn_h}px;
}}
{font_style}
{style}
{color256_str}
</style>
{s}
</svg>
"#
    )
}
