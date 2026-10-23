use crate::{
    cell::{Cell, Style},
    color::AnsiColor,
    render::ans::cells_to_ans,
};

/// Convert an encoded image into an ANSI truecolor string.
///
/// Returns `None` when the input is not a supported image.
pub fn image_to_ans(buf: &[u8]) -> Option<String> {
    let image = image::load_from_memory(buf).ok()?.to_rgb8();
    let (width, height) = image.dimensions();
    let mut pixels = vec![vec![Cell::default(); width as usize]; height as usize];

    for (x, y, pixel) in image.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        pixels[y as usize][x as usize] = Cell::new(
            Style {
                fg: AnsiColor::Rgb(r, g, b),
                ..Style::default()
            },
            "█",
            Default::default(),
        );
    }

    Some(cells_to_ans(&pixels))
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::image_to_ans;
    use crate::{Canvas, color::AnsiColor};

    #[test]
    fn converts_pixels_in_row_major_order() {
        let image = image::RgbImage::from_fn(3, 2, |x, y| {
            image::Rgb([(x * 100) as u8, (y * 100) as u8, 0])
        });
        let mut bytes = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
            .unwrap();

        let ansi = image_to_ans(&bytes).unwrap();
        let canvas = Canvas::new(&ansi, None);

        assert_eq!((canvas.w, canvas.h), (3, 2));
        assert_eq!(canvas.pixels[0][1].style.fg, AnsiColor::Rgb(100, 0, 0));
        assert_eq!(canvas.pixels[1][2].style.fg, AnsiColor::Rgb(200, 100, 0));
    }

    #[test]
    fn ignores_non_images() {
        assert!(image_to_ans(b"not an image").is_none());
    }
}
