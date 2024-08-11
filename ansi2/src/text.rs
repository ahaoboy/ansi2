use crate::Canvas;

pub fn to_text<S: AsRef<str>>(str: S, width: Option<usize>) -> String {
    let s = str.as_ref();
    let canvas = Canvas::new(s, width);
    let mut list: Vec<char> = vec![];
    for row in canvas.pixels.iter() {
        for c in row.iter() {
            list.push(c.char)
        }
        list.push('\n')
    }
    return list.iter().collect();
}
