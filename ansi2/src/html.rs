use std::collections::HashSet;

use crate::{theme::ColorTable, Canvas};

fn to_style(theme: impl ColorTable) -> String {
    format!(
        r#"
.black{{
color: rgb{:?};
}}
.red{{
color: rgb{:?};
}}
.green{{
color: rgb{:?};
}}
.yellow{{
color: rgb{:?};
}}
.blue{{
color: rgb{:?};
}}
.magenta{{
color: rgb{:?};
}}
.cyan{{
color: rgb{:?};
}}
.white{{
color: rgb{:?};
}}

.bright_black{{
color: rgb{:?};
}}
.bright_red{{
color: rgb{:?};
}}
.bright_green{{
color: rgb{:?};
}}
.bright_yellow{{
color: rgb{:?};
}}
.bright_blue{{
color: rgb{:?};
}}
.bright_magenta{{
color: rgb{:?};
}}
.bright_cyan{{
color: rgb{:?};
}}
.bright_white{{
color: rgb{:?};
}}

.bg_black{{
background-color: rgb{:?};
}}
.bg_red{{
background-color: rgb{:?};
}}
.bg_green{{
background-color: rgb{:?};
}}
.bg_yellow{{
background-color: rgb{:?};
}}
.bg_blue{{
background-color: rgb{:?};
}}
.bg_magenta{{
background-color: rgb{:?};
}}
.bg_cyan{{
background-color: rgb{:?};
}}
.bg_white{{
background-color: rgb{:?};
}}

.bg_bright_black{{
background-color: rgb{:?};
}}
.bg_bright_red{{
background-color: rgb{:?};
}}
.bg_bright_green{{
background-color: rgb{:?};
}}
.bg_bright_yellow{{
background-color: rgb{:?};
}}
.bg_bright_blue{{
background-color: rgb{:?};
}}
.bg_bright_magenta{{
background-color: rgb{:?};
}}
.bg_bright_cyan{{
background-color: rgb{:?};
}}
.bg_bright_white{{
background-color: rgb{:?};
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
pub fn to_html(
    s: &str,
    theme: impl ColorTable,
    width: Option<usize>,
    font: Option<String>,
) -> String {
    let canvas = Canvas::new(s, width);
    let mut s = String::new();
    let style = to_style(theme);

    let mut fg_color_style = HashSet::new();
    let mut bg_color_style = HashSet::new();

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

    s.push_str("<div class='ansi-main'>\n");
    for row in canvas.pixels.iter() {
        s.push_str("<div class='row'>");
        for c in row.iter() {
            let mut text_class = vec!["char".into()];
            let mut bg_class = vec!["char".into()];
            if c.bold {
                text_class.push("bold".into());
            }

            if !c.color.is_default() {
                let name = c.color.name();
                let style = format!(r#".{name} {{ color: {} }}"#, c.color.to_rgb(theme));
                fg_color_style.insert(style);
                text_class.push(name);
            }

            if !c.bg_color.is_default() {
                let name = "bg_".to_string() + &c.bg_color.name();
                let style = format!(
                    r#".{name} {{ background-color: {} }}"#,
                    c.bg_color.to_rgb(theme)
                );
                bg_color_style.insert(style);
                bg_class.push(name);
            }

            if c.blink {
                text_class.push("blink".into());
            }

            let text_class = text_class.join(" ");
            let bg_class = bg_class.join(" ");
            let html_char = c.char.to_string();
            let html_char = html_escape::encode_text(&html_char);
            s.push_str(&format!(
                "<div class='{bg_class}'><div class='{text_class}'>{html_char}</div></div>",
            ))
        }
        s.push_str("</div>");
    }
    s.push_str("</div>\n");

    let fg_style = fg_color_style.into_iter().collect::<Vec<_>>().join("\n");
    let bg_style = bg_color_style.into_iter().collect::<Vec<_>>().join("\n");
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <style>

{font_style}
{style}
{fg_style}
{bg_style}

.ansi-main{{
  display: flex;
  flex-direction: column;
}}

.row{{
  display: flex;
}}

.char{{
  margin: 0;
  padding: 0;
  font-family: ansi2-custom-font, Courier, monospace;
  white-space: pre;
}}

.bold{{
 font-weight: bold;
}}

  </style>
</head>
<body>
{s}
</body>
</html>
"#
    )
}
