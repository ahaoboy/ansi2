mod cmd;
mod error;
#[cfg(feature = "minify")]
mod minify;

use std::{
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use ansi2::{LengthAdjust, Mode, Options, Theme, image::image_to_ans};
use base64::{Engine as _, prelude::BASE64_STANDARD};
use clap::{Parser, Subcommand, ValueEnum};
use which_shell::{Shell, which_shell};

use self::error::{Error, Result};

const GIT_HASH: &str = git_version::git_version!();
const VERSION: &str = const_str::concat!(env!("CARGO_PKG_VERSION"), " ", GIT_HASH);

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Format {
    Svg,
    Html,
    Text,
    Ans,
    #[cfg(feature = "sequin")]
    Sequin,
}

#[derive(Parser, Debug)]
#[command(version = VERSION, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<SubCommand>,

    #[command(flatten)]
    common: Common,

    /// Input file path (defaults to stdin)
    input: Option<PathBuf>,
}

#[derive(clap::Args, Debug, Clone)]
struct Common {
    #[arg(short, long, global = true)]
    format: Option<Format>,

    #[arg(short, long, global = true)]
    width: Option<usize>,

    #[arg(short, long, global = true)]
    theme: Option<Theme>,

    #[arg(short, long, global = true)]
    mode: Option<Mode>,

    #[arg(long, global = true)]
    font: Option<String>,

    #[arg(long, global = true)]
    light_bg: Option<String>,

    #[arg(long, global = true)]
    dark_bg: Option<String>,

    #[arg(long, global = true)]
    font_size: Option<usize>,

    #[arg(long, global = true)]
    length_adjust: Option<String>,

    #[arg(short, long, global = true)]
    sourcemap: bool,

    /// Output file path
    #[arg(short = 'o', long, global = true)]
    output: Option<PathBuf>,

    /// Open the output file in the default browser
    #[arg(long, global = true)]
    open: bool,
}

impl Common {
    fn options(&self, font: Option<String>) -> Options {
        Options {
            theme: self.theme.unwrap_or_default(),
            mode: self.mode,
            width: self.width,
            font,
            font_size: self.font_size.unwrap_or(16),
            light_bg: self.light_bg.clone(),
            dark_bg: self.dark_bg.clone(),
            length_adjust: LengthAdjust::parse(self.length_adjust.as_deref()),
            sourcemap: self.sourcemap,
        }
    }
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Execute commands with optional prompt and syntax highlighting
    Cmd {
        /// The command to execute (can be specified multiple times)
        #[arg(short = 'c', long = "command", required = true)]
        commands: Vec<String>,

        /// Show the shell prompt before each command (disabled by default)
        #[arg(long)]
        prompt: bool,

        /// Shell to use (fish, bash, zsh, ...). Auto-detected if not specified
        #[arg(long)]
        shell: Option<Shell>,
    },
}

pub fn run() -> Result<()> {
    let Cli {
        command,
        common,
        input,
    } = Cli::parse();

    let format = common.format.unwrap_or(Format::Svg);
    let font = resolve_font(common.font.clone())?;
    let options = common.options(font);

    match command {
        Some(SubCommand::Cmd {
            commands,
            prompt,
            shell,
        }) => {
            let shell = shell.unwrap_or_else(detect_shell);
            if shell == Shell::Unknown {
                return Err(Error::UnknownShell);
            }
            let input = cmd::run(&shell.to_string(), &commands, prompt)?;
            let output = render(format, &input, &options)?;
            write_output(&output, common.output.as_deref(), common.open)
        }
        None => {
            let buf = read_input(input.as_deref())?;
            let input =
                image_to_ans(&buf).unwrap_or_else(|| String::from_utf8_lossy(&buf).into_owned());

            let output = render(format, &input, &options)?;
            write_output(&output, common.output.as_deref(), common.open)
        }
    }
}

fn detect_shell() -> Shell {
    which_shell().map(|info| info.shell).unwrap_or(Shell::Bash)
}

fn read_input(path: Option<&Path>) -> Result<Vec<u8>> {
    match path {
        Some(path) => fs::read(path).map_err(|source| Error::Read {
            path: path.into(),
            source,
        }),
        None => {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .map_err(|source| Error::Read {
                    path: "<stdin>".into(),
                    source,
                })?;
            Ok(buf)
        }
    }
}

fn resolve_font(font: Option<String>) -> Result<Option<String>> {
    let Some(font) = font else {
        return Ok(None);
    };

    if font.starts_with("http") || font.starts_with("data:") || !Path::new(&font).exists() {
        return Ok(Some(font));
    }

    let path = PathBuf::from(&font);
    let bytes = fs::read(&path).map_err(|source| Error::Read {
        path: path.clone(),
        source,
    })?;
    Ok(Some(format!(
        "data:font;base64,{}",
        BASE64_STANDARD.encode(bytes)
    )))
}

fn render(format: Format, input: &str, options: &Options) -> Result<String> {
    match format {
        Format::Svg => {
            let svg = ansi2::to_svg(input, options);
            #[cfg(feature = "minify")]
            {
                minify::minify_svg(&svg)
            }
            #[cfg(not(feature = "minify"))]
            {
                Ok(svg)
            }
        }
        Format::Html => Ok(ansi2::to_html(input, options)),
        Format::Text => Ok(ansi2::to_text(input, options.width)),
        Format::Ans => Ok(ansi2::to_ans(input, options.width)),
        #[cfg(feature = "sequin")]
        Format::Sequin => Ok(ansi2::to_sequin(input, options.width)),
    }
}

fn write_output(content: &str, path: Option<&Path>, open: bool) -> Result<()> {
    let Some(path) = path else {
        if open {
            return Err(Error::OpenWithoutOutput);
        }
        std::io::stdout()
            .write_all(content.as_bytes())
            .map_err(|source| Error::Write {
                path: "<stdout>".into(),
                source,
            })?;
        return Ok(());
    };

    fs::write(path, content).map_err(|source| Error::Write {
        path: path.into(),
        source,
    })?;

    if open {
        opener::open(path).map_err(|source| Error::Open {
            path: path.into(),
            source,
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use clap::Parser;

    use super::{Cli, SubCommand};

    fn command(args: &[&str]) -> (bool, Option<which_shell::Shell>) {
        let cli = Cli::try_parse_from(args).unwrap();
        match cli.command.unwrap() {
            SubCommand::Cmd { prompt, shell, .. } => (prompt, shell),
        }
    }

    #[test]
    fn prompt_is_opt_in() {
        let (prompt, shell) = command(&["ansi2", "cmd", "-c", "echo hi", "--shell", "fish"]);
        assert!(!prompt);
        assert_eq!(shell, Some(which_shell::Shell::Fish));
    }

    #[test]
    fn prompt_flag_enables_prompt() {
        let (prompt, _) = command(&["ansi2", "cmd", "-c", "echo hi", "--prompt"]);
        assert!(prompt);
    }
}
