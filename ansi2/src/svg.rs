use crate::{
    css::{to_style, CssType, Mode},
    theme::ColorTable,
    Canvas,
};

pub fn to_svg<S: AsRef<str>>(
    str: S,
    theme: impl ColorTable,
    width: Option<usize>,
    font: Option<String>,
    mode: Option<Mode>,
) -> String {
    let s = str.as_ref();
    let canvas = Canvas::new(s, width);
    let mut s = String::new();
    let mut cur_x = 0;
    let fn_w = 20;
    let fn_h = 32;
    let baseline_h = 16;
    let mut cur_y = 0;
    let style = to_style(theme, CssType::Svg, mode);
    let font_style = if let Some(base64) = font {
        format!(
            r#"
@font-face {{
  font-family: ansi2-custom-font;
  src: url(data:font/truetype;charset=utf-8;base64,{base64});
}}
"#
        )
    } else {
        "".into()
    };
    for row in canvas.pixels.iter() {
        for c in row.iter() {
            let mut text_class = vec![];

            if !c.bg_color.is_default() {
                let name = "bg-".to_string() + &c.bg_color.name();

                let class_str = format!(" class='{}'", name);
                s.push_str(&format!(
                    r#"<rect x="{cur_x}px" y="{cur_y}px" width="{fn_w}px" height="{fn_h}px" {class_str}/>"#
                    ,
                ));
            }

            if !c.color.is_default() {
                let name = c.color.name();
                text_class.push(name);
            };

            if c.bold {
                text_class.push("bold".into());
            }
            if c.blink {
                text_class.push("blink".into());
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
r#"<text x="{text_x}px" y="{text_y}px" width="{fn_w}px" height="{fn_h}px" {}><tspan>{}</tspan></text>"#,
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

    format!(
        r#"<svg
width="{svg_w}px"
height="{svg_h}px"
xmlns="http://www.w3.org/2000/svg"
xmlns:xlink="http://www.w3.org/1999/xlink"
>
<style>
tspan {{
font-variant-ligatures: none;
dominant-baseline: central;
font-variant-ligatures: none;
white-space: pre;
font-family: ansi2-custom-font, Courier, monospace;
font-size: {fn_h}px;
}}
{font_style}
{style}
</style>
{s}
</svg>
"#
    )
}
