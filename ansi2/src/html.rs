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
) -> String {
    let s = str.as_ref();
    let canvas = Canvas::new(s, width);
    let mut s = String::new();
    let style = to_style(theme, CssType::Html, mode);

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
                text_class.push(name);
            }

            if !c.bg_color.is_default() {
                let name = "bg-".to_string() + &c.bg_color.name();
                bg_class.push(name);
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
