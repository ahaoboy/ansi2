use std::collections::HashSet;

use crate::{theme::ColorTable, Canvas};

fn to_style(theme: impl ColorTable) -> String {
    format!(
        r#"

.black{{
fill: rgb{:?};
}}
.red{{
fill: rgb{:?};
}}
.green{{
fill: rgb{:?};
}}
.yellow{{
fill: rgb{:?};
}}
.blue{{
fill: rgb{:?};
}}
.magenta{{
fill: rgb{:?};
}}
.cyan{{
fill: rgb{:?};
}}
.white{{
fill: rgb{:?};
}}

.bright_black{{
fill: rgb{:?};
}}
.bright_red{{
fill: rgb{:?};
}}
.bright_green{{
fill: rgb{:?};
}}
.bright_yellow{{
fill: rgb{:?};
}}
.bright_blue{{
fill: rgb{:?};
}}
.bright_magenta{{
fill: rgb{:?};
}}
.bright_cyan{{
fill: rgb{:?};
}}
.bright_white{{
fill: rgb{:?};
}}

.bold{{
font-weight: bold;
}}

.blink {{
  animation: blink_keyframes 1s steps(1, end) infinite;
}}

@keyframes blink_keyframes{{
  50% {{
    opacity: 0;
  }}
}}
"#,
        theme.black(),
        theme.red(),
        theme.green(),
        theme.yellow(),
        theme.blue(),
        theme.magenta(),
        theme.cyan(),
        theme.white(),
        theme.bright_black(),
        theme.bright_red(),
        theme.bright_green(),
        theme.bright_yellow(),
        theme.bright_blue(),
        theme.bright_magenta(),
        theme.bright_cyan(),
        theme.bright_white(),
    )
}

pub fn to_svg(s: &str, theme: impl ColorTable, width: Option<usize>) -> String {
    let canvas = Canvas::new(s, width);
    let mut s = String::new();
    let mut cur_x = 0;
    let fn_w = 20;
    let fn_h = 32;
    let baseline_h = 16;
    let mut cur_y = 0;
    let style = to_style(theme);
    let mut fg_color_style = HashSet::new();
    let mut bg_color_style = HashSet::new();

    for row in canvas.pixels.iter() {
        for c in row.iter() {
            let mut text_class = vec![];

            if !c.bg_color.is_default() {
                let name = "bg_".to_string() + &c.bg_color.name();
                let style = format!(r#".{name} {{ fill: {} }}"#, c.bg_color.to_rgb(theme));
                bg_color_style.insert(style);

                s.push_str(&format!(
                    r#"<rect x="{cur_x}px" y="{cur_y}px" width="{fn_w}px" height="{fn_h}px" class="{name}"/>"#,
                ));
            }

            if !c.color.is_default() {
                let name = c.color.name();
                let style = format!(r#".{name} {{ fill: {} }}"#, c.color.to_rgb(theme));
                fg_color_style.insert(style);
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
            s.push_str(&format!(
r#"<text x="{text_x}px" y="{text_y}px" width="{fn_w}px" height="{fn_h}px" class="{}"><tspan>{}</tspan></text>"#,
                text_class.join(" "),
                html_escape::encode_text(&c.char.to_string())
            ));
            cur_x += fn_w;
        }
        cur_y += fn_h;
        cur_x = 0;
    }

    let svg_w = fn_w * canvas.w;
    let svg_h = fn_h * canvas.h;
    let fg_style = fg_color_style.into_iter().collect::<Vec<_>>().join("\n");
    let bg_style = bg_color_style.into_iter().collect::<Vec<_>>().join("\n");

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
    font-family: Courier, monospace;
    font-size: {fn_h}px;
}}

{style}
{fg_style}
{bg_style}

</style>
{s}
</svg>
"#
    )
}
