use crate::canvas::Canvas;

pub fn to_text<S: AsRef<str>>(str: S, width: Option<usize>) -> String {
    let s = str.as_ref();
    let canvas = Canvas::new(s, width);
    let mut text: Vec<String> = Vec::new();
    for row in canvas.pixels.iter() {
        let mut row_str = String::new();
        for c in row.iter() {
            row_str.push_str(&c.text)
        }
        text.push(row_str);
    }
    text.join("\n")
}
