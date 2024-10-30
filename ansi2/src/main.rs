use ansi2::ans::to_ans;
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
    Ans,
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

    #[arg(long)]
    font_size: Option<usize>,

    #[arg(long)]
    length_adjust: Option<String>,

    #[arg(short, long, default_value_t = false)]
    sourcemap: bool,
}

fn main() {
    let args: Args = Args::parse();
    let Args {
        width,
        format,
        theme,
        mode,
        font,
        compress,
        light_bg,
        dark_bg,
        font_size,
        length_adjust,
        sourcemap,
    } = args;
    let format = format.unwrap_or(Format::Svg);
    let theme = theme.unwrap_or(Theme::Vscode);

    let mut buf = Vec::new();
    std::io::stdin()
        .read_to_end(&mut buf)
        .expect("can't read string from stdin");
    let base64 = font.map(|font_url| {
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
    let output = match format {
        Format::Svg => {
            let mut svg = to_svg(
                s,
                theme,
                width,
                base64,
                mode,
                light_bg,
                dark_bg,
                font_size,
                length_adjust,
                sourcemap,
            );
            if compress {
                svg = osvg::osvg(
                    &svg,
                    Some(
                        r#"
{
  plugins: [
    {
      name: "preset-default",
      params: {
        overrides: {
          inlineStyles: false,
        },
      },
    },
  ],
}"#,
                    ),
                )
                .expect("compress error");
            }
            svg
        }
        Format::Html => to_html(
            &s, theme, width, base64, mode, light_bg, dark_bg, font_size, sourcemap,
        ),
        Format::Text => to_text(&s, width),
        Format::Ans => to_ans(&s, width, compress),
    };
    print!("{}", output);
}
