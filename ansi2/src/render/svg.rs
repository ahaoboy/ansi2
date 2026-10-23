use std::fmt::Write;

use crate::{
    canvas::Canvas,
    css::{CssType, Font, Markup, Stylesheet, markup},
    options::{LengthAdjust, Options},
};

pub fn to_svg(input: &str, options: &Options) -> String {
    let canvas = Canvas::new(input, options.width);
    let font = Font::new(options.font.as_deref());
    let mut sheet = Stylesheet::default();
    let mut body = String::new();

    let font_size = options.font_size;
    let char_width = font_size * 5 / 8;
    let line_height = font_size;
    let baseline = font_size / 2;
    let underline = font_size / 8;
    let text_height = line_height + underline;

    let mut x = 0;
    let mut y = 0;

    for row in canvas.minify() {
        for cell in &row {
            let width = char_width * cell.text.chars().count();
            let text_x = x;
            let text_y = y + baseline + underline;

            if !cell.style.bg.is_default() && !cell.style.hide {
                let class = cell.style.bg.bg_class();
                let _ = write!(
                    body,
                    r#"<rect x="{x}" y="{}" width="{width}" height="{text_height}" class='{class}'/>"#,
                    y + underline
                );
                sheet.add_bg_color(cell.style.bg);
            }

            let Markup { classes, attrs } =
                markup(cell, CssType::Svg, options.sourcemap, &mut sheet);

            let length = match options.length_adjust {
                LengthAdjust::Auto => format!(r#"textLength="{width}""#),
                LengthAdjust::Spacing => {
                    format!(r#"lengthAdjust="spacing" textLength="{width}""#)
                }
                LengthAdjust::SpacingAndGlyphs => {
                    format!(r#"lengthAdjust="spacingAndGlyphs" textLength="{width}""#)
                }
                LengthAdjust::Disabled => String::new(),
            };

            let _ = write!(
                body,
                r#"<text x="{text_x}" y="{text_y}" width="{width}" height="{text_height}" class='{classes}' {attrs}><tspan {length}>{}</tspan></text>"#,
                html_escape::encode_text(&cell.text)
            );

            x += width;
        }
        y += line_height + underline;
        x = 0;
    }

    let width = char_width * canvas.w;
    let height = (line_height + underline) * canvas.h;
    let css = sheet.to_css(options, &font, font_size, CssType::Svg);

    format!(
        r#"<svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><style>{}{css}</style>{body}</svg>"#,
        font.face
    )
}
