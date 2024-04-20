use ansi2::Canvas;

pub fn to_svg(s: &str) -> String {
    let canvas = Canvas::new(s);
    let mut s = String::new();
    let mut cur_x = 0;
    let fn_w = 20;
    let fn_h = 32;
    let mut cur_y = fn_h;

    for row in canvas.pixels.iter() {
        for c in row.iter() {
            if c.bg_color.0 != 0 {
                s.push_str(&format!(
                    r#"<rect x="{cur_x}px" y="{cur_y}px" width="{fn_w}px" height="{fn_h}px" fill="{}"/>"#,
                    c.bg_color.to_rgb(),
                ));
            }
            let fill_str = if c.color.0 == 0 {
                "".into()
            } else {
                format!("fill='{}'", c.color.to_rgb())
            };

            s.push_str(&format!(
                r#"<text x="{cur_x}px" y="{cur_y}px" font-weight="{}" width="{fn_w}px" height="{fn_h}px" {}>
                    <tspan>{}</tspan>
                </text>"#,
                if c.bold { "bold" } else { "normal" },
                fill_str,
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
           xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve">

<style>
tspan {{
    font-variant-ligatures: none;
    dominant-baseline: central;
    font-variant-ligatures: none;
    white-space: pre;
    font-family: Courier, monospace;
    font-size: {fn_h}px;
}}

</style>

        {s}
</svg>
"#
    )
}
