use std::io::Read;

use ansi2html::to_html;

fn main() {
    let mut s = Vec::new();
    std::io::stdin().read_to_end(&mut s).unwrap();
    println!("{}", to_html(&String::from_utf8_lossy(&s)));
}
