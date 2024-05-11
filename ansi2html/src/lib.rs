use ansi2::{theme::ColorTable, Canvas};

pub fn to_html(s: &str, theme: impl ColorTable) -> String {
    let canvas = Canvas::new(s);
    let mut s = String::new();
    s.push_str("<div class='ansi-main'>\n");
    for row in canvas.pixels.iter() {
        s.push_str("<div class='row'>");
        for c in row.iter() {
            let bold: &str = if c.bold { "bold" } else { "normal" };
            let fn_w = format!("font-weight: {bold};");
            let mut class = String::from("char");
            if c.bold {
                class.push_str(" char-bold")
            }

            s.push_str(&format!(
                "<div class='{class}' style='color: {};background: {}; {fn_w}; '>{}</div>",
                c.color.to_rgb(theme),
                c.bg_color.to_rgb(theme),
                html_escape::encode_text(&c.char.to_string())
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
  font-family: Courier, monospace;
  white-space: pre;
}}

.char-bold{{
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
