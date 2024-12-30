use image::{ImageReader, Pixel};

use crate::{canvas::pixels_to_ans, color::AnsiColor, node::Node};

pub fn image_to_ans(buf: &[u8]) -> Option<String> {
    let img = ImageReader::new(std::io::Cursor::new(buf))
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()?;

    let rgb_image = img.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let mut pixels = vec![vec![Node::default(); width as usize]; height as usize];
    for (y, x, pixel) in rgb_image.enumerate_pixels() {
        let node = &mut pixels[x as usize][y as usize];
        let rgb = pixel.to_rgb();
        node.color = AnsiColor::Rgb(rgb[0], rgb[1], rgb[2]);
        node.text = "█".to_string();
    }
    Some(pixels_to_ans(pixels))
}
