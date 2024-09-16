use crate::theme::ColorTable;


#[derive(Debug, Clone, Copy)]
pub  enum CssType {
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
    let light_bg_color = "rgba(255, 255, 255, 0)".to_string();

    let (color_field, bg_field) = match ty {
        CssType::Html => ("color", "background-color"),
        CssType::Svg => ("fill", "fill"),
    };

    let light_colors = [
        ("black", theme.black()),
        ("red", theme.red()),
        ("green", theme.green()),
        ("yellow", theme.yellow()),
        ("blue", theme.blue()),
        ("magenta", theme.magenta()),
        ("cyan", theme.cyan()),
        ("white", theme.white()),
        ("bright_black", theme.bright_black()),
        ("bright_red", theme.bright_red()),
        ("bright_green", theme.bright_green()),
        ("bright_yellow", theme.bright_yellow()),
        ("bright_blue", theme.bright_blue()),
        ("bright_magenta", theme.bright_magenta()),
        ("bright_cyan", theme.bright_cyan()),
        ("bright_white", theme.bright_white()),
    ];
    let dark_colors = [
        ("black", theme.white()),
        ("red", theme.red()),
        ("green", theme.green()),
        ("yellow", theme.yellow()),
        ("blue", theme.blue()),
        ("magenta", theme.magenta()),
        ("cyan", theme.cyan()),
        ("white", theme.black()),
        ("bright_black", theme.bright_white()),
        ("bright_red", theme.bright_red()),
        ("bright_green", theme.bright_green()),
        ("bright_yellow", theme.bright_yellow()),
        ("bright_blue", theme.bright_blue()),
        ("bright_magenta", theme.bright_magenta()),
        ("bright_cyan", theme.bright_cyan()),
        ("bright_white", theme.bright_white()),
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

    if let Some(mode) = mode {
        let default_text_style = match (mode, ty) {
            (Mode::Dark, CssType::Html) => format!("div{{color: rgb{:?} }}", theme.white()),
            (Mode::Dark, CssType::Svg) => format!("text{{fill: rgb{:?} }}", theme.white()),
            (Mode::Light, CssType::Html) => format!("div{{color: rgb{:?} }}", theme.black()),
            (Mode::Light, CssType::Svg) => format!("text{{fill: rgb{:?} }}", theme.black()),
        };

        let colors = match mode {
            Mode::Dark => dark_colors,
            Mode::Light => light_colors,
        };
        let color_css: String = colors.iter().fold(String::new(), |mut acc, (name, c)| {
            acc.push_str(&format!(".{name}{{ {color_field}: rgb{:?};}} ", c));
            acc
        });

        let bg_color_css: String = colors.iter().fold(String::new(), |mut acc, (name, c)| {
            acc.push_str(&format!(".bg-{name}{{ {bg_field}: rgb{:?};}} ", c));
            acc
        });

        let root_style = match mode {
            Mode::Dark => format!(":root{{background-color: {dark_bg_color} }}"),
            Mode::Light => format!(":root{{background-color: {light_bg_color}}}"),
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

    let light_var_color_css: String =
        light_colors
            .iter()
            .fold(String::new(), |mut acc, (name, c)| {
                acc.push_str(&format!("--{name}: rgb{:?};", c));
                acc
            });

    let light_var_bg_color_css: String =
        light_colors
            .iter()
            .fold(String::new(), |mut acc, (name, c)| {
                acc.push_str(&format!("--bg-{name}: rgb{:?};", c));
                acc
            });

    let dark_var_color_css: String =
        dark_colors
            .iter()
            .fold(String::new(), |mut acc, (name, c)| {
                acc.push_str(&format!("--{name}: rgb{:?};", c));
                acc
            });

    let dark_var_bg_color_css: String =
        dark_colors
            .iter()
            .fold(String::new(), |mut acc, (name, c)| {
                acc.push_str(&format!("--bg-{name}: rgb{:?};", c));
                acc
            });

    let root_css = r#"
:root {
color-scheme: light dark;
}
  "#.to_string()
    .trim()
    .to_string();

    let class_color_css: String = light_colors
        .iter()
        .fold(String::new(), |mut acc, (name, _)| {
            acc.push_str(&format!(".{name}{{{color_field}: var(--{name});}}"));
            acc
        });

    let class_bg_color_css: String =
        light_colors
            .iter()
            .fold(String::new(), |mut acc, (name, _)| {
                acc.push_str(&format!(".bg-{name}{{{bg_field}: var(--bg-{name});}}"));
                acc
            });

    let dark_css = format!(
        r#"
@media (prefers-color-scheme: dark) {{
:root {{
{dark_var_color_css}
{dark_var_bg_color_css}
background-color: {dark_bg_color}
}}
}}
  "#
    )
    .trim()
    .to_string();

    let light_css = format!(
        r#"
@media (prefers-color-scheme: light) {{
:root {{
  {light_var_color_css}
  {light_var_bg_color_css}
background-color: {light_bg_color}
}}
}}
"#
    )
    .trim()
    .to_string();

    let mut style = format!(
        r#"
{root_css}

{class_color_css}

{class_bg_color_css}

{dark_css}

{light_css}

{common_style}
"#,
    );

    match ty {
        CssType::Svg => style.push_str(
            r#"
text{
fill: var(--black)
}
"#
            .trim(),
        ),
        CssType::Html => style.push_str(
            r#"
div{
color: var(--black)
}
"#
            .trim(),
        ),
    }
    style
}
