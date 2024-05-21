use std::io::Read;

use ansi2::{html::to_html, svg::to_svg, theme::Theme};
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone, Copy)]
enum Format {
    Svg,
    Html,
}

#[derive(Parser, Debug, Clone, Copy)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    format: Option<Format>,

    #[arg(short, long)]
    width: Option<usize>,

    #[arg(short, long)]
    theme: Option<Theme>,
}

fn main() {
    let args = Args::parse();

    let format = args.format.unwrap_or(Format::Svg);
    let theme = args.theme.unwrap_or(Theme::Vscode);
    let width = args.width;

    let mut s = Vec::new();
    std::io::stdin().read_to_end(&mut s).unwrap();

    let output = match format {
        Format::Svg => to_svg(&String::from_utf8_lossy(&s), theme, width),
        Format::Html => to_html(&String::from_utf8_lossy(&s), theme, width),
    };

    println!("{}", output);
}
