use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::cli::error::{Error, Result};

/// Run a program and capture its stdout.
fn exec(program: &str, args: &[&str], stdin: Option<&[u8]>) -> Result<String> {
    let mut command = Command::new(program);
    command.args(args);

    let output = match stdin {
        None => command.output(),
        Some(input) => {
            command
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            let mut child = command.spawn().map_err(|source| Error::Spawn {
                program: program.into(),
                source,
            })?;
            child
                .stdin
                .take()
                .expect("stdin was requested")
                .write_all(input)
                .map_err(|source| Error::CommandInput {
                    program: program.into(),
                    source,
                })?;
            child.wait_with_output()
        }
    };

    output
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
        .map_err(|source| Error::Spawn {
            program: program.into(),
            source,
        })
}

fn trim_ansi_newline(value: &str) -> String {
    match value.strip_suffix("\n\x1b[m") {
        Some(trimmed) => format!("{trimmed}\x1b[m"),
        None => value.into(),
    }
}

fn prompt(shell: &str) -> Result<Option<String>> {
    let output = match shell {
        "fish" => exec(shell, &["-c", "fish_prompt"], None)?,
        "bash" => exec(
            shell,
            &["-c", "PS1='\\u@\\h:\\w\\$ ' bash -i -c 'echo -n \"$PS1\"'"],
            None,
        )?,
        "zsh" => exec(shell, &["-c", "print -P '%n@%m:%~%# '"], None)?,
        _ => return Ok(None),
    };
    Ok(Some(trim_ansi_newline(&output)))
}

fn highlight(shell: &str, command: &str) -> Result<Option<String>> {
    if shell == "fish" {
        let output = exec(
            shell,
            &["-c", "fish_indent --ansi"],
            Some(command.as_bytes()),
        )?;
        return Ok(Some(trim_ansi_newline(&output)));
    }
    Ok(Some(command.into()))
}

/// Execute every command, optional prompt and syntax highlighting combined
/// into a single ANSI stream.
pub fn run(shell: &str, commands: &[String], with_prompt: bool) -> Result<String> {
    let prompt = if with_prompt { prompt(shell)? } else { None };
    let mut output = String::new();

    for command in commands {
        if let Some(prompt) = &prompt {
            output.push_str(prompt);
        }

        if let Some(highlighted) = highlight(shell, command)? {
            output.push_str(&highlighted);
            if !highlighted.ends_with('\n') {
                output.push('\n');
            }
        }

        output.push_str(&exec(shell, &["-c", command], None)?);
    }

    Ok(output)
}
