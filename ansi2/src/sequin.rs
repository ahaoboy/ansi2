use std::fmt::Write;

use crate::{
    color::{AnsiColor, Color8},
    lexer::{Event, tokens},
    theme,
};

/// Convert ANSI input to sequin-style human-readable output.
///
/// Each escape sequence gets a line showing its kind, raw parameters and an
/// explanation, matching the Go sequin project's format.
pub fn to_sequin(input: &str, _width: Option<usize>) -> String {
    let mut output = String::new();
    let mut text = String::new();

    for token in tokens(input) {
        match token.event {
            Event::Char(c) => text.push(c),
            event => {
                flush_text(&mut text, &mut output);
                write_event(&mut output, &event);
            }
        }
    }

    flush_text(&mut text, &mut output);
    output
}

fn flush_text(text: &mut String, output: &mut String) {
    if !text.is_empty() {
        let _ = writeln!(output, "Text {text}");
        text.clear();
    }
}

fn line(output: &mut String, kind: &str, sequence: &str, explanation: &str) {
    let _ = writeln!(output, "{kind:>4} {sequence}: {explanation}");
}

fn ctrl(output: &mut String, byte: u8) {
    line(
        output,
        "Ctrl",
        &escape_ctrl_byte(byte),
        ctrl_code_name(byte),
    );
}

fn write_event(output: &mut String, event: &Event) {
    use Event::*;

    match event {
        Char(_) => {}
        List(items) => {
            let descriptions: Vec<String> = items.iter().filter_map(describe).collect();
            if !descriptions.is_empty() {
                line(output, "CSI", "m", &descriptions.join(", "));
            }
        }

        Bell => ctrl(output, 0x07),
        Backspace => ctrl(output, 0x08),
        Tab => ctrl(output, 0x09),
        LineFeed => ctrl(output, 0x0a),
        FormFeed => ctrl(output, 0x0c),
        CarriageReturn => ctrl(output, 0x0d),
        Unknown(byte) => ctrl(output, *byte),

        Reset => line(output, "CSI", "m", "Reset style"),
        Bold => line(output, "CSI", "1m", "Bold"),
        Dim => line(output, "CSI", "2m", "Faint"),
        Italic => line(output, "CSI", "3m", "Italic"),
        Underline => line(output, "CSI", "4m", "Underline"),
        SlowBlink => line(output, "CSI", "5m", "Blink"),
        RapidBlink => line(output, "CSI", "6m", "Blink"),
        Reverse => line(output, "CSI", "7m", "Inverse"),
        Conceal => line(output, "CSI", "8m", "Invisible"),
        Strike => line(output, "CSI", "9m", "Crossed-out"),
        NormalIntensity => line(output, "CSI", "22m", "Normal intensity"),
        Unitalic => line(output, "CSI", "23m", "No italic"),
        Ununderline => line(output, "CSI", "24m", "No underline"),
        Unblink => line(output, "CSI", "25m", "No blink"),
        Unreverse => line(output, "CSI", "27m", "No reverse"),
        Unconceal => line(output, "CSI", "28m", "No conceal"),
        Unstrike => line(output, "CSI", "29m", "No crossed-out"),

        Foreground(color) => {
            let (sequence, description) = describe_color(color, true);
            line(output, "CSI", &sequence, &description);
        }
        Background(color) => {
            let (sequence, description) = describe_color(color, false);
            line(output, "CSI", &sequence, &description);
        }
        DefaultForeground => line(output, "CSI", "39m", "Default foreground color"),
        DefaultBackground => line(output, "CSI", "49m", "Default background color"),
        DefaultUnderline => line(output, "CSI", "59m", "Default underline color"),
        UnderlineColor(color) => {
            let (sequence, description) = describe_underline_color(color);
            line(output, "CSI", &sequence, &description);
        }
        FgBg(fg, bg) => {
            let (fg_sequence, fg_description) = describe_color(fg, true);
            let (bg_sequence, bg_description) = describe_color(bg, false);
            let sequence = format!(
                "{};{}m",
                fg_sequence.trim_end_matches('m'),
                bg_sequence.trim_end_matches('m')
            );
            line(
                output,
                "CSI",
                &sequence,
                &format!("{fg_description}, {bg_description}"),
            );
        }

        CursorUp(n) => {
            let sequence = maybe_counted(*n, "A");
            line(output, "CSI", &sequence, &format!("Cursor up {n}"));
        }
        CursorDown(n) => {
            let sequence = maybe_counted(*n, "B");
            line(output, "CSI", &sequence, &format!("Cursor down {n}"));
        }
        CursorForward(n) => {
            let sequence = maybe_counted(*n, "C");
            line(output, "CSI", &sequence, &format!("Cursor right {n}"));
        }
        CursorBack(n) => {
            let sequence = maybe_counted(*n, "D");
            line(output, "CSI", &sequence, &format!("Cursor left {n}"));
        }
        CursorNextLine(n) => line(
            output,
            "CSI",
            &format!("{n}E"),
            &format!("Cursor next line {n}"),
        ),
        CursorPreviousLine(n) => line(
            output,
            "CSI",
            &format!("{n}F"),
            &format!("Cursor previous line {n}"),
        ),
        CursorHorizontalAbsolute(n) => line(
            output,
            "CSI",
            &format!("{n}G"),
            &format!("Cursor horizontal absolute {n}"),
        ),
        CursorPosition(row, col) => line(
            output,
            "CSI",
            &format!("{row};{col}H"),
            &format!("Set cursor position row={row} col={col}"),
        ),
        HorizontalVerticalPosition(row, col) => line(
            output,
            "CSI",
            &format!("{row};{col}f"),
            &format!("Horizontal vertical position row={row} col={col}"),
        ),

        EraseInDisplay(n) => {
            let description = match n {
                0 => "Erase screen bellow",
                1 => "Erase screen above",
                2 => "Erase entire screen",
                3 => "Erase entire display",
                _ => "Erase screen",
            };
            let sequence = if *n == 0 { "J".into() } else { format!("{n}J") };
            line(output, "CSI", &sequence, description);
        }
        EraseInLine(n) => {
            let description = match n {
                0 => "Erase line right",
                1 => "Erase line left",
                2 => "Erase entire line",
                _ => "Erase line",
            };
            let sequence = if *n == 0 { "K".into() } else { format!("{n}K") };
            line(output, "CSI", &sequence, description);
        }
        ScrollUp(n) => line(
            output,
            "CSI",
            &format!("{n}S"),
            &format!("Scroll up {} lines", (*n).max(1)),
        ),
        ScrollDown(n) => line(
            output,
            "CSI",
            &format!("{n}T"),
            &format!("Scroll down {} lines", (*n).max(1)),
        ),

        DeviceStatusReport => line(output, "CSI", "6n", "Request cursor position"),
        AuxPortOn => line(output, "CSI", "5i", "AUX port on"),
        AuxPortOff => line(output, "CSI", "4i", "AUX port off"),

        Title(title) => line(
            output,
            "OSC",
            &format!("0;{title}"),
            &format!("Set icon name and window title to \"{title}\""),
        ),
        Cwd(url) => line(
            output,
            "OSC",
            &format!("7;{url}"),
            &format!("Set working directory to {url}"),
        ),
        Prompt => line(output, "OSC", "133", "Shell integration"),
        Link { url, title, .. } => line(
            output,
            "OSC",
            &format!("8;;{url}"),
            &format!("Set hyperlink, to \"{url}\" title \"{title}\""),
        ),

        Decset => line(
            output,
            "CSI",
            "?2004h",
            "Enable private mode \"bracketed paste\"",
        ),
        Decrst => line(
            output,
            "CSI",
            "?2004l",
            "Disable private mode \"bracketed paste\"",
        ),
        CursorShow => line(
            output,
            "CSI",
            "?25h",
            "Enable private mode \"cursor visibility\"",
        ),
        CursorHide => line(
            output,
            "CSI",
            "?25l",
            "Disable private mode \"cursor visibility\"",
        ),
        Keypad => line(output, "ESC", "=", "Application Keypad"),
        Ext => line(output, "CSI", ">4;1m", "Extended mode"),

        PrimaryFont => line(output, "CSI", "10m", "Primary font"),
        AlternativeFont(n) => line(
            output,
            "CSI",
            &format!("{}m", 10 + n),
            &format!("Alternative font {n}"),
        ),
        Fraktur => line(output, "CSI", "20m", "Fraktur"),
        DoublyUnderlined => line(output, "CSI", "21m", "Doubly underlined"),
    }
}

fn maybe_counted(count: i32, suffix: &str) -> String {
    if count == 1 {
        suffix.into()
    } else {
        format!("{count}{suffix}")
    }
}

fn describe(event: &Event) -> Option<String> {
    use Event::*;

    Some(match event {
        Reset => "Reset style".into(),
        Bold => "Bold".into(),
        Dim => "Faint".into(),
        Italic => "Italic".into(),
        Underline => "Underline".into(),
        SlowBlink | RapidBlink => "Blink".into(),
        Reverse => "Inverse".into(),
        Conceal => "Invisible".into(),
        Strike => "Crossed-out".into(),
        NormalIntensity => "Normal intensity".into(),
        Unitalic => "No italic".into(),
        Ununderline => "No underline".into(),
        Unblink => "No blink".into(),
        Unreverse => "No reverse".into(),
        Unconceal => "No conceal".into(),
        Unstrike => "No crossed-out".into(),
        Foreground(color) => describe_color(color, true).1,
        Background(color) => describe_color(color, false).1,
        DefaultForeground => "Default foreground color".into(),
        DefaultBackground => "Default background color".into(),
        DefaultUnderline => "Default underline color".into(),
        _ => return None,
    })
}

fn describe_color(color: &AnsiColor, foreground: bool) -> (String, String) {
    let (prefix, label) = if foreground {
        ("38", "foreground")
    } else {
        ("48", "background")
    };

    match color {
        AnsiColor::Color8(color) => (
            format!("{}m", color.to_u8()),
            format!("ANSI {label} color: {}", color8_name(*color)),
        ),
        AnsiColor::Color256(index) => {
            let index = *index;
            if index < 8 {
                let code = if foreground { 30 + index } else { 40 + index };
                (
                    format!("{code}m"),
                    format!("ANSI {label} color: {}", basic_color_name(index)),
                )
            } else if index < 16 {
                let code = if foreground { 82 + index } else { 92 + index };
                (
                    format!("{code}m"),
                    format!("ANSI {label} color: Bright {}", basic_color_name(index - 8)),
                )
            } else {
                (
                    format!("{prefix};5;{index}m"),
                    format!("ANSI256 {label} color: {index} (#{})", color256_hex(index)),
                )
            }
        }
        AnsiColor::Rgb(r, g, b) => (
            format!("{prefix};2;{r};{g};{b}m"),
            format!("24-bit RGB {label} color: #{r:02X}{g:02X}{b:02X}"),
        ),
        AnsiColor::Default => {
            let code = if foreground { 39 } else { 49 };
            (format!("{code}m"), format!("Default {label} color"))
        }
    }
}

fn describe_underline_color(color: &AnsiColor) -> (String, String) {
    match color {
        AnsiColor::Color8(color) => (
            format!("58;5;{}m", color.to_u8()),
            format!("ANSI underline color: {}", color8_name(*color)),
        ),
        AnsiColor::Color256(index) => {
            let index = *index;
            if index < 8 {
                (
                    format!("58;5;{index}m"),
                    format!("ANSI underline color: {}", basic_color_name(index)),
                )
            } else if index < 16 {
                (
                    format!("58;5;{index}m"),
                    format!(
                        "ANSI underline color: Bright {}",
                        basic_color_name(index - 8)
                    ),
                )
            } else {
                (
                    format!("58;5;{index}m"),
                    format!(
                        "ANSI256 underline color: {index} (#{})",
                        color256_hex(index)
                    ),
                )
            }
        }
        AnsiColor::Rgb(r, g, b) => (
            format!("58;2;{r};{g};{b}m"),
            format!("24-bit RGB underline color: #{r:02X}{g:02X}{b:02X}"),
        ),
        AnsiColor::Default => ("59m".into(), "Default underline color".into()),
    }
}

fn color256_hex(index: u8) -> String {
    let (r, g, b) = theme::color256(index);
    format!("{r:02X}{g:02X}{b:02X}")
}

fn basic_color_name(index: u8) -> &'static str {
    match index {
        0 => "Black",
        1 => "Red",
        2 => "Green",
        3 => "Yellow",
        4 => "Blue",
        5 => "Magenta",
        6 => "Cyan",
        7 => "White",
        _ => "Unknown",
    }
}

fn color8_name(color: Color8) -> String {
    match color {
        Color8::Black => "Black",
        Color8::Red => "Red",
        Color8::Green => "Green",
        Color8::Yellow => "Yellow",
        Color8::Blue => "Blue",
        Color8::Magenta => "Magenta",
        Color8::Cyan => "Cyan",
        Color8::White => "White",
        Color8::BrightBlack => "Bright Black",
        Color8::BrightRed => "Bright Red",
        Color8::BrightGreen => "Bright Green",
        Color8::BrightYellow => "Bright Yellow",
        Color8::BrightBlue => "Bright Blue",
        Color8::BrightMagenta => "Bright Magenta",
        Color8::BrightCyan => "Bright Cyan",
        Color8::BrightWhite => "Bright White",
    }
    .into()
}

fn ctrl_code_name(byte: u8) -> &'static str {
    match byte {
        0x00 => "Null",
        0x01 => "Start of heading",
        0x02 => "Start of text",
        0x03 => "End of text",
        0x04 => "End of transmission",
        0x05 => "Enquiry",
        0x06 => "Acknowledge",
        0x07 => "Bell",
        0x08 => "Backspace",
        0x09 => "Horizontal tab",
        0x0a => "Line feed",
        0x0b => "Vertical tab",
        0x0c => "Form feed",
        0x0d => "Carriage return",
        0x0e => "Shift out",
        0x0f => "Shift in",
        0x10 => "Data link escape",
        0x11 => "Device control 1",
        0x12 => "Device control 2",
        0x13 => "Device control 3",
        0x14 => "Device control 4",
        0x15 => "Negative acknowledge",
        0x16 => "Synchronous idle",
        0x17 => "End of transmission block",
        0x18 => "Cancel",
        0x19 => "End of medium",
        0x1a => "Substitute",
        0x1b => "Escape",
        0x1c => "File separator",
        0x1d => "Group separator",
        0x1e => "Record separator",
        0x1f => "Unit separator",
        0x7f => "Delete",
        _ => "Unknown",
    }
}

fn escape_ctrl_byte(byte: u8) -> String {
    match byte {
        0x07 => "\\a".into(),
        0x08 => "\\b".into(),
        0x09 => "\\t".into(),
        0x0a => "\\n".into(),
        0x0b => "\\v".into(),
        0x0c => "\\f".into(),
        0x0d => "\\r".into(),
        0x1b => "ESC".into(),
        _ => format!("\\x{byte:02x}"),
    }
}

#[cfg(test)]
mod test {
    use super::to_sequin;

    #[test]
    fn combined_sgr() {
        assert_eq!(
            to_sequin("\x1b[1;31mhi", None),
            " CSI m: Bold, ANSI foreground color: Red\nText hi\n"
        );
    }

    #[test]
    fn control_codes() {
        assert_eq!(to_sequin("\x07", None), "Ctrl \\a: Bell\n");
    }
}
