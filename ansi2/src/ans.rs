use crate::{color::AnsiColor, Canvas, Node};

pub fn to_ans<S: AsRef<str>>(str: S, width: Option<usize>, compress: bool) -> String {
    let s = str.as_ref();
    let canvas = Canvas::new(s, width);

    let iter = if compress {
        canvas.minify().into_iter()
    } else {
        canvas.pixels.into_iter()
    };

    let mut text: Vec<String> = Vec::new();

    let mut last_node = Node::default();

    for row in iter {
        let mut row_str = Vec::new();
        for c in row.iter() {
            if !last_node.same_style(c) {
                // FIXME: Find the minimum distance between two styles
                row_str.push("\x1b[0m".to_string());
                if c.bold {
                    row_str.push("\x1b[1m".to_string());
                }
                if c.italic {
                    row_str.push("\x1b[3m".to_string());
                }
                if c.dim {
                    row_str.push("\x1b[2m".to_string());
                }
                if c.underline {
                    row_str.push("\x1b[4m".to_string());
                }
                if c.hide {
                    row_str.push("\x1b[8m".to_string());
                }
                if c.blink {
                    row_str.push("\x1b[5m".to_string());
                }

                row_str.push(match c.color {
                    AnsiColor::Default => "".to_string(),
                    AnsiColor::Color8(color8) => format!("\x1b[{}m", color8.to_u8()),
                    AnsiColor::Color256(n) => format!("\x1b[38;5;{}m", n),
                    AnsiColor::Rgb(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
                });

                row_str.push(match c.bg_color {
                    AnsiColor::Default => "".to_string(),
                    AnsiColor::Color8(color8) => format!("\x1b[{}m", color8.to_u8() + 10),
                    AnsiColor::Color256(n) => format!("\x1b[48;5;{}m", n),
                    AnsiColor::Rgb(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
                });
            }
            row_str.push(c.text.clone());
            last_node = c.clone();
        }
        text.push(row_str.into_iter().collect());
    }
    text.join("\n")
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::{ans::to_ans, Canvas};

    #[test]
    fn test() {
        let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let assets_dir = Path::new(&cargo_dir).parent().unwrap().join("assets");
        let v = std::fs::read_dir(assets_dir).unwrap();
        for i in v {
            let p = i.unwrap().path().to_string_lossy().to_string();
            if !p.ends_with(".ans") {
                continue;
            }
            if p.ends_with(".min.ans") {
                continue;
            }
            let s = std::fs::read_to_string(&p).unwrap();
            let min = to_ans(&s, None, true);

            let c1 = Canvas::new(&s, None);
            let c2 = Canvas::new(&min, None);
            assert_eq!(c1, c2);
        }
    }
}
