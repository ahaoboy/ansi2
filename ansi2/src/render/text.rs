use crate::canvas::Canvas;

pub fn to_text(input: &str, width: Option<usize>) -> String {
    let canvas = Canvas::new(input, width);
    let mut rows = Vec::with_capacity(canvas.pixels.len());

    for row in &canvas.pixels {
        let mut text = String::new();
        for cell in row {
            text.push_str(&cell.text);
        }
        rows.push(text);
    }

    rows.join("\n")
}
