use ansi2::ans::to_ans;
use ansi2::image::image_to_ans;
use ansi2::{css::Mode, theme::Theme};
use ansi2::{html::to_html, svg::to_svg, text::to_text};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use clap::{Parser, ValueEnum, command};
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

    #[clap()]
    file: Option<String>,
}

fn process_input(buf: Vec<u8>) -> String {
    if let Some(ty) = infer::get(&buf) {
        if ty.matcher_type() == infer::MatcherType::Image {
            if let Some(s) = image_to_ans(&buf) {
                return s;
            }
        }
    }

    return String::from_utf8_lossy(&buf).to_string();
}
fn main() {
    let args: Args = Args::parse();
    let Args {
        width,
        format,
        theme,
        mode,
        font,
        light_bg,
        dark_bg,
        font_size,
        length_adjust,
        sourcemap,
        file,
    } = args;
    let format = format.unwrap_or(Format::Svg);
    let theme = theme.unwrap_or(Theme::Vscode);

    let buf = if let Some(file) = file {
        std::fs::read(file).expect("can't read string from file")
    } else {
        let mut v = Vec::new();
        std::io::stdin()
            .read_to_end(&mut v)
            .expect("can't read string from stdin");
        v
    };

    let s = process_input(buf);
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

    let output = match format {
        Format::Svg => {
            let svg = to_svg(
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
            #[cfg(feature = "minify")]
            let svg = minify_svg(&svg).expect("compress error");
            svg
        }
        Format::Html => to_html(
            &s, theme, width, base64, mode, light_bg, dark_bg, font_size, sourcemap,
        ),
        Format::Text => to_text(&s, width),
        Format::Ans => to_ans(&s, width),
    };
    print!("{}", output);
}

#[cfg(feature = "minify")]
fn minify_svg(svg: &str) -> Result<String, String> {
    use oxvg_ast::{
        implementations::{roxmltree::parse, shared::Element},
        serialize::{Indent, Node as _, Options},
        visitor::Info,
    };
    use oxvg_optimiser::Jobs;
    let arena = typed_arena::Arena::new();
    let dom = parse(svg, &arena).map_err(|e| e.to_string())?;

    Jobs::default()
        .run(&dom, &Info::<Element>::new(&arena))
        .map_err(|err| err.to_string())?;

    dom.serialize_with_options(Options {
        indent: Indent::None,
        ..Default::default()
    })
    .map_err(|err| err.to_string())
}
