use crate::theme::ColorTable;

fn get_hex((r, g, b): (u8, u8, u8)) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

#[derive(Debug, Clone, Copy)]
pub enum CssType {
    Svg,
    Html,
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Dark,
    Light,
}

pub(crate) fn to_style(theme: impl ColorTable, ty: CssType, mode: Option<Mode>) -> String {
    let dark_bg_color = "rgb(32,32,32)".to_string();
    let light_bg_color = "rgba(255,255,255,0)".to_string();

    let (color_field, bg_field) = match ty {
        CssType::Html => ("color", "background-color"),
        CssType::Svg => ("fill", "fill"),
    };

    let light_colors = [
        ("black", get_hex(theme.black())),
        ("red", get_hex(theme.red())),
        ("green", get_hex(theme.green())),
        ("yellow", get_hex(theme.yellow())),
        ("blue", get_hex(theme.blue())),
        ("magenta", get_hex(theme.magenta())),
        ("cyan", get_hex(theme.cyan())),
        ("white", get_hex(theme.white())),
        ("bright_black", get_hex(theme.bright_black())),
        ("bright_red", get_hex(theme.bright_red())),
        ("bright_green", get_hex(theme.bright_green())),
        ("bright_yellow", get_hex(theme.bright_yellow())),
        ("bright_blue", get_hex(theme.bright_blue())),
        ("bright_magenta", get_hex(theme.bright_magenta())),
        ("bright_cyan", get_hex(theme.bright_cyan())),
        ("bright_white", get_hex(theme.bright_white())),
    ];
    let dark_colors = [
        ("black", get_hex(theme.white())),
        ("red", get_hex(theme.red())),
        ("green", get_hex(theme.green())),
        ("yellow", get_hex(theme.yellow())),
        ("blue", get_hex(theme.blue())),
        ("magenta", get_hex(theme.magenta())),
        ("cyan", get_hex(theme.cyan())),
        ("white", get_hex(theme.black())),
        ("bright_black", get_hex(theme.bright_white())),
        ("bright_red", get_hex(theme.bright_red())),
        ("bright_green", get_hex(theme.bright_green())),
        ("bright_yellow", get_hex(theme.bright_yellow())),
        ("bright_blue", get_hex(theme.bright_blue())),
        ("bright_magenta", get_hex(theme.bright_magenta())),
        ("bright_cyan", get_hex(theme.bright_cyan())),
        ("bright_white", get_hex(theme.bright_white())),
    ];

    let common_style = r#"
.bold{
font-weight: bold;
}

.blink {
animation: blink_keyframes 1s steps(1, end) infinite;
}

@keyframes blink_keyframes{
50% {
opacity: 0;
}
}
"#;

    let light_color_css: String = light_colors
        .iter()
        .fold(String::new(), |mut acc, (name, c)| {
            acc.push_str(&format!(".{name}{{ {color_field}: {};}} ", c));
            acc
        });

    let bg_light_color_css: String =
        light_colors
            .iter()
            .fold(String::new(), |mut acc, (name, c)| {
                acc.push_str(&format!(".bg-{name}{{ {bg_field}: {};}} ", c));
                acc
            });
    let dark_color_css: String = dark_colors
        .iter()
        .fold(String::new(), |mut acc, (name, c)| {
            acc.push_str(&format!(".{name}{{ {color_field}: {};}} ", c));
            acc
        });

    let bg_dark_color_css: String = dark_colors
        .iter()
        .fold(String::new(), |mut acc, (name, c)| {
            acc.push_str(&format!(".bg-{name}{{ {bg_field}: {};}} ", c));
            acc
        });

    if let Some(mode) = mode {
        let default_text_style = match (mode, ty) {
            (Mode::Dark, CssType::Html) => {
                format!("div{{color: {} }}", get_hex(theme.white()))
            }
            (Mode::Dark, CssType::Svg) => format!("text{{fill:{}}}", get_hex(theme.white())),
            (Mode::Light, CssType::Html) => {
                format!("div{{color:{}}}", get_hex(theme.black()))
            }
            (Mode::Light, CssType::Svg) => {
                format!("text{{fill:{}}}", get_hex(theme.black()))
            }
        };

        let (color_css, bg_color_css) = match mode {
            Mode::Dark => (dark_color_css, bg_dark_color_css),
            Mode::Light => (light_color_css, bg_light_color_css),
        };

        let root_style = match mode {
            Mode::Dark => format!(":root{{background-color:{dark_bg_color}}}"),
            Mode::Light => format!(":root{{background-color:{light_bg_color}}}"),
        };

        let style = format!(
            r#"
{root_style}
{default_text_style}
{common_style}
{color_css}
{bg_color_css}
      "#
        )
        .trim()
        .to_string();

        return style;
    }

    let default_light_text_style = match ty {
        CssType::Svg => format!("text{{fill:{}}}", get_hex(theme.black())),
        CssType::Html => format!("div{{color:{}}}", get_hex(theme.black())),
    };

    let default_dark_text_style = match ty {
        CssType::Svg => format!("text{{fill:{}}}", get_hex(theme.white())),
        CssType::Html => format!("div{{color:{}}}", get_hex(theme.white())),
    };

    let root_css = format!(
        r#"
:root {{color-scheme: light dark; background-color: {light_bg_color}}}
{light_color_css}
{bg_light_color_css}
{default_light_text_style}
"#
    );

    let dark_css = format!(
        r#"
@media (prefers-color-scheme: dark) {{
:root {{background-color: {dark_bg_color}}}
{dark_color_css}
{bg_dark_color_css}
{default_dark_text_style}
}}
"#
    )
    .trim()
    .to_string();

    format!(
        r#"
{root_css}

{dark_css}

{common_style}
"#,
    )
    .trim()
    .to_string()
}