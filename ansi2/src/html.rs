use std::collections::HashSet;

use crate::{
    css::{to_style, CssType, Mode},
    theme::ColorTable,
    Canvas,
};

pub fn to_html<S: AsRef<str>>(
    str: S,
    theme: impl ColorTable,
    width: Option<usize>,
    font: Option<String>,
    mode: Option<Mode>,
    light_bg: Option<String>,
    dark_bg: Option<String>,
) -> String {
    let s = str.as_ref();
    let canvas = Canvas::new(s, width);
    let mut s = String::new();
    let style = to_style(theme, CssType::Html, mode, light_bg,dark_bg);
    let mut font_style = "".into();
    let mut font_family = "Consolas,Courier New,Monaco".into();

    if let Some(url) = font {
        if url.starts_with("http") || url.starts_with("data:font;base64") {
            font_family = "ansi2-custom-font".into();
            font_style = format!(
                r#"
  @font-face {{
    font-family: ansi2-custom-font;
    src: url({url});
  }}
  "#
            )
        } else {
            font_family = url;
        }
    }

    s.push_str("<div class='ansi-main'>\n");

    let mut color256 = HashSet::new();
    for row in canvas.pixels.iter() {
        s.push_str("<div class='row'>");
        for c in row.iter() {
            let mut text_class = vec!["char".into()];
            let mut bg_class = vec!["char".into()];
            if c.bold {
                text_class.push("bold".into());
            }
            if c.italic {
                text_class.push("italic".into());
            }
            if c.dim {
                text_class.push("dim".into());
            }
            if c.underline {
                text_class.push("underline".into());
            }
            if c.hide {
              text_class.push("hide".into());
          }
            if !c.color.is_default() {
                let name = c.color.name();
                text_class.push(name);

                if let crate::lex::AnsiColor::Rgb(r, g, b) = c.color {
                    color256.insert(format!(".rgb_{r}_{g}_{b}{{ color: rgb({r},{g},{b}) ;}}\n"));
                }
            }

            if !c.bg_color.is_default() {
                let name = "bg-".to_string() + &c.bg_color.name();
                bg_class.push(name);

                if let crate::lex::AnsiColor::Rgb(r, g, b) = c.color {
                    color256.insert(format!(
                        ".bg-rgb_{r}_{g}_{b}{{ background: rgb({r},{g},{b}) ;}}\n"
                    ));
                }
            }

            if c.blink {
                text_class.push("blink".into());
            }

            let text_class = text_class.join(" ").trim().to_string();
            let bg_class = bg_class.join(" ");
            let html_char = c.char.to_string();
            let html_char = html_escape::encode_text(&html_char);
            let class_str = if text_class.is_empty() {
                String::new()
            } else {
                format!("class='{text_class}'")
            };
            s.push_str(&format!(
                "<div class='{bg_class}'><div {class_str}>{html_char}</div></div>",
            ))
        }
        s.push_str("</div>");
    }

    let color256_str: String = color256.into_iter().collect();
    s.push_str("</div>\n");

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <style>
{font_style}
{style}
{color256_str}
.ansi-main{{display:flex;flex-direction:column;}}
.row{{display: flex;}}
.char{{
  margin: 0;
  padding: 0;
  font-family: {font_family};
  white-space: pre;
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
