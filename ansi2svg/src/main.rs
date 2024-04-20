use std::io::Read;
use ansi2svg::to_svg;

fn main() {
    let mut s = Vec::new();
    std::io::stdin().read_to_end(&mut s).unwrap();
    println!("{}", to_svg(&String::from_utf8_lossy(&s)));
}
