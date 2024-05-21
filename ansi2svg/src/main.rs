use ansi2::{svg::to_svg, theme::Theme};
use std::io::Read;

fn main() {
    let mut s = Vec::new();
    std::io::stdin().read_to_end(&mut s).unwrap();
    println!(
        "{}",
        to_svg(&String::from_utf8_lossy(&s), Theme::Vscode, None)
    );
}
