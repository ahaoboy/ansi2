use ansi2::ans::to_ans;
use ansi2::image::image_to_ans;
use ansi2::{css::Mode, theme::Theme};
use ansi2::{html::to_html, svg::to_svg, text::to_text};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::{fs::read, io::Read};
use which_shell::{Shell, which_shell};

#[derive(ValueEnum, Debug, Clone, Copy)]
enum Format {
    Svg,
    Html,
    Text,
    Ans,
}

#[derive(Parser, Debug, Clone)]
struct CommonOptions {
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

    /// Output file path (e.g., -o output.svg)
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// Open the output file in default browser
    #[arg(long, default_value_t = false)]
    open: bool,

    #[clap()]
    input: Option<PathBuf>,
}

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = git_version::git_version!();
const VERSION: &str = const_str::concat!(CARGO_PKG_VERSION, " ", GIT_HASH);

#[derive(Parser, Debug, Clone)]
#[command(version=VERSION, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    common: CommonOptions,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Execute commands with optional prompt and syntax highlighting
    Cmd {
        /// The command to execute (can be specified multiple times)
        #[arg(short = 'c', long = "command", required = true)]
        commands: Vec<String>,

        /// Add shell prompt before the command
        #[arg(long, default_value_t = true)]
        prompt: bool,

        /// Shell to use (fish, bash, zsh, etc.). Auto-detected if not specified
        #[arg(long)]
        shell: Option<Shell>,

        #[command(flatten)]
        common: CommonOptions,
    },
}

fn process_input(buf: Vec<u8>) -> String {
    if let Some(ty) = infer::get(&buf)
        && ty.matcher_type() == infer::MatcherType::Image
        && let Some(s) = image_to_ans(&buf)
    {
        return s;
    }

    String::from_utf8_lossy(&buf).to_string()
}

fn get_prompt(shell: &str) -> Option<String> {
    let output = match shell {
        "fish" => Command::new(shell).arg("-c").arg("fish_prompt").output(),
        "bash" => Command::new(shell)
            .arg("-c")
            .arg("PS1='\\u@\\h:\\w\\$ ' bash -i -c 'echo -n \"$PS1\"'")
            .output(),
        "zsh" => Command::new(shell)
            .arg("-c")
            .arg("print -P '%n@%m:%~%# '")
            .output(),
        _ => return None,
    };

    output
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
}

fn highlight_command(shell: &str, command: &str) -> Option<String> {
    let output = match shell {
        "fish" => Command::new(shell)
            .arg("-c")
            .arg(format!("echo '{}' | fish_indent --ansi", command))
            .output(),
        "bash" | "zsh" => {
            // For bash/zsh, we can try using bat or just return the command as-is
            return Some(command.to_string());
        }
        _ => return Some(command.to_string()),
    };

    output
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim_end().to_string())
}

fn execute_command(shell: &str, command: &str) -> Result<String, String> {
    let output = Command::new(shell)
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn handle_cmd_subcommand(
    commands: Vec<String>,
    prompt: bool,
    shell: Option<Shell>,
    common: CommonOptions,
) {
    let shell = shell.unwrap_or_else(|| which_shell().map(|i| i.shell).unwrap_or(Shell::Bash));
    let format = common.format.unwrap_or(Format::Svg);
    let theme = common.theme.unwrap_or(Theme::Vscode);

    let mut ansi_output = String::new();

    // Process each command
    for (idx, cmd) in commands.iter().enumerate() {
        // Add separator between commands (except for the first one)
        if idx > 0 {
            ansi_output.push('\n');
        }

        // Add prompt if requested
        if prompt && let Some(prompt_str) = get_prompt(&shell.to_string()) {
            ansi_output.push_str(&prompt_str);
        }

        // Add highlighted command
        if let Some(highlighted) = highlight_command(&shell.to_string(), cmd) {
            ansi_output.push_str(&highlighted);
            ansi_output.push('\n');
        }

        // Execute command and add output
        match execute_command(&shell.to_string(), cmd) {
            Ok(output) => ansi_output.push_str(&output),
            Err(e) => {
                eprintln!("Error executing command '{}': {}", cmd, e);
                std::process::exit(1);
            }
        }
    }

    // Convert to base64 font if needed
    let base64 = common.font.map(|font_url| {
        if font_url.starts_with("http") {
            return font_url;
        }

        if !Path::new(&font_url).exists() {
            return font_url;
        }

        let bin = read(font_url).expect("read font file error");
        let base64 = BASE64_STANDARD.encode(bin);
        format!("data:font;base64,{base64}")
    });

    // Format output
    let output = match format {
        Format::Svg => {
            let svg = to_svg(
                ansi_output,
                theme,
                common.width,
                base64,
                common.mode,
                common.light_bg,
                common.dark_bg,
                common.font_size,
                common.length_adjust,
                common.sourcemap,
            );
            #[cfg(feature = "minify")]
            let svg = minify_svg(&svg).expect("compress error");
            svg
        }
        Format::Html => to_html(
            &ansi_output,
            theme,
            common.width,
            base64,
            common.mode,
            common.light_bg,
            common.dark_bg,
            common.font_size,
            common.sourcemap,
        ),
        Format::Text => to_text(&ansi_output, common.width),
        Format::Ans => to_ans(&ansi_output, common.width),
    };

    write_output(&output, common.output, common.open);
}

fn main() {
    let args: Args = Args::parse();

    // Handle subcommands
    if let Some(Commands::Cmd {
        commands,
        prompt,
        shell,
        common,
    }) = args.command
    {
        handle_cmd_subcommand(commands, prompt, shell, common);
        return;
    }

    // Original functionality
    let Args { common, .. } = args;
    let format = common.format.unwrap_or(Format::Svg);
    let theme = common.theme.unwrap_or(Theme::Vscode);

    let buf = if let Some(file) = common.input {
        std::fs::read(file).expect("can't read string from file")
    } else {
        let mut v = Vec::new();
        std::io::stdin()
            .read_to_end(&mut v)
            .expect("can't read string from stdin");
        v
    };

    let s = process_input(buf);
    let base64 = common.font.map(|font_url| {
        if font_url.starts_with("http") {
            return font_url;
        }

        if !Path::new(&font_url).exists() {
            return font_url;
        }

        let bin = read(font_url).expect("read font file error");
        let base64 = BASE64_STANDARD.encode(bin);
        format!("data:font;base64,{base64}")
    });

    let output = match format {
        Format::Svg => {
            let svg = to_svg(
                s,
                theme,
                common.width,
                base64,
                common.mode,
                common.light_bg,
                common.dark_bg,
                common.font_size,
                common.length_adjust,
                common.sourcemap,
            );
            #[cfg(feature = "minify")]
            let svg = minify_svg(&svg).expect("compress error");
            svg
        }
        Format::Html => to_html(
            &s,
            theme,
            common.width,
            base64,
            common.mode,
            common.light_bg,
            common.dark_bg,
            common.font_size,
            common.sourcemap,
        ),
        Format::Text => to_text(&s, common.width),
        Format::Ans => to_ans(&s, common.width),
    };

    write_output(&output, common.output, common.open);
}

fn write_output(content: &str, output_path: Option<PathBuf>, open: bool) {
    if let Some(path) = output_path {
        // Write to file
        std::fs::write(&path, content).expect("Failed to write output file");

        // Open in browser if requested
        if open && let Err(e) = opener::open(&path) {
            eprintln!("Failed to open file in browser: {}", e);
            std::process::exit(1);
        }
    } else {
        // Print to stdout
        print!("{}", content);
    }
}

#[cfg(feature = "minify")]
fn minify_svg(svg: &str) -> Result<String, String> {
    use oxvg_ast::{
        parse::roxmltree::parse,
        serialize::Node as _,
        visitor::Info,
        xmlwriter::{Options, Space},
    };
    use oxvg_optimiser::Jobs;
    let config = Jobs::default();

    let opt = Options {
        trim_whitespace: Space::Never,
        ..Default::default()
    };
    parse(svg, |dom, allocator| {
        config
            .run(dom, &Info::new(allocator))
            .map_err(|e| e.to_string())?;
        dom.serialize_with_options(opt).map_err(|e| e.to_string())
    })
    .map_err(|e| e.to_string())?
}
