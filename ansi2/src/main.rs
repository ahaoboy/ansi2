use ansi2::{css::Mode, theme::Theme};
use ansi2::{html::to_html, svg::to_svg, text::to_text};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use clap::{command, Parser, ValueEnum};
use std::path::Path;
use std::{fs::read, io::Read};

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

    #[clap(short, long)]
    mode: Option<Mode>,

    #[arg(long)]
    font: Option<String>,

    #[arg(short, long, default_value_t = false)]
    compress: bool,

    #[arg(long)]
    light_bg: Option<String>,
    #[arg(long)]
    dark_bg: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    let format = args.format.unwrap_or(Format::Svg);
    let theme = args.theme.unwrap_or(Theme::Vscode);
    let mode = args.mode.map(|m| match m {
        Mode::Dark => ansi2::css::Mode::Dark,
        Mode::Light => ansi2::css::Mode::Light,
    });
    let width = args.width;

    let mut buf = Vec::new();
    std::io::stdin()
        .read_to_end(&mut buf)
        .expect("can't read string from stdin");
    let base64 = args.font.map(|font_url| {
        if font_url.starts_with("http") {
            return font_url;
        }

        if !Path::new(&font_url).exists() {
            return font_url;
        }

        let bin = read(font_url).expect("read font file error");
        let base64 = BASE64_STANDARD.encode(bin);
        return format!("data:font;base64,{base64}");
    });

    let s = String::from_utf8_lossy(&buf);
    let mut output = match format {
        Format::Svg => to_svg(s, theme, width, base64, mode, args.light_bg, args.dark_bg),
        Format::Html => to_html(&s, theme, width, base64, mode, args.light_bg, args.dark_bg),
        Format::Text => to_text(&s, width),
    };

    if args.compress {
        output = osvg::osvg(&output).expect("compress error");
    }

    println!("{}", output);
}
