use std::{fs::read, io::Read};

use ansi2::text::to_text;
use ansi2::{html::to_html, svg::to_svg, theme::Theme};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone, Copy)]
enum Format {
    Svg,
    Html,
    Text,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    format: Option<Format>,

    #[arg(short, long)]
    width: Option<usize>,

    #[arg(short, long)]
    theme: Option<Theme>,

    #[arg(long)]
    font: Option<String>,
}

fn main() {
    let args = Args::parse();

    let format = args.format.unwrap_or(Format::Svg);
    let theme = args.theme.unwrap_or(Theme::Vscode);
    let width = args.width;

    let mut buf = Vec::new();
    std::io::stdin()
        .read_to_end(&mut buf)
        .expect("can't read string from stdin");
    let base64 = args.font.map(|p| {
        let bin = read(p).expect("read font file error");
        BASE64_STANDARD.encode(bin)
    });

    let s = String::from_utf8_lossy(&buf);
    let output = match format {
        Format::Svg => to_svg(s, theme, width, base64),
        Format::Html => to_html(&s, theme, width, base64),
        Format::Text => to_text(&s, width),
    };

    println!("{}", output);
}
