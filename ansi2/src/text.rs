use crate::Canvas;

pub fn to_text<S: AsRef<str>>(str: S, width: Option<usize>) -> String {
    let s = str.as_ref();
    let canvas = Canvas::new(s, width);
    let mut text = String::new();
    for row in canvas.pixels.iter() {
        for c in row.iter() {
            text.push_str(&c.text)
        }
        text.push('\n')
    }
    text
}
