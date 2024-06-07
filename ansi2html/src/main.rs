use std::io::Read;

use ansi2::theme::Theme;

fn main() {
    let mut s = Vec::new();
    std::io::stdin().read_to_end(&mut s).unwrap();
    println!(
        "{}",
        ansi2::html::to_html(&String::from_utf8_lossy(&s), Theme::Vscode, None,None)
    );
}
