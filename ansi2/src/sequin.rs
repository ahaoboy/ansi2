use crate::color::{AnsiColor, Color8};
use crate::lex::{Sgr, parse_ansi};

/// Format a sequin output line.
/// The Go sequin format is: `<KIND (4-char right-aligned)> <sequence>: <explanation>\n`
fn fmt_line(kind: &str, seq: &str, explanation: &str) -> String {
    format!("{:>4} {}: {}\n", kind, seq, explanation)
}

/// Format a text line.
fn fmt_text(text: &str) -> String {
    format!("Text {}\n", text)
}

/// Get the basic color name for SGR foreground/background codes.
fn basic_color_name(n: u8) -> &'static str {
    match n {
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

/// Describe a Color8 value as sequin would.
fn color8_name(c: &Color8) -> String {
    match c {
        Color8::Black => "Black".into(),
        Color8::Red => "Red".into(),
        Color8::Green => "Green".into(),
        Color8::Yellow => "Yellow".into(),
        Color8::Blue => "Blue".into(),
        Color8::Magenta => "Magenta".into(),
        Color8::Cyan => "Cyan".into(),
        Color8::White => "White".into(),
        Color8::BrightBlack => "Bright Black".into(),
        Color8::BrightRed => "Bright Red".into(),
        Color8::BrightGreen => "Bright Green".into(),
        Color8::BrightYellow => "Bright Yellow".into(),
        Color8::BrightBlue => "Bright Blue".into(),
        Color8::BrightMagenta => "Bright Magenta".into(),
        Color8::BrightCyan => "Bright Cyan".into(),
        Color8::BrightWhite => "Bright White".into(),
    }
}

/// Get hex color string for a 256-color palette index.
fn color256_hex(idx: usize) -> String {
    let (r, g, b) = crate::theme::COLOR256[idx];
    format!("{:02X}{:02X}{:02X}", r, g, b)
}

/// Describe an SGR attribute for the sequin explanation.
fn describe_sgr(sgr: &Sgr) -> Option<String> {
    match sgr {
        Sgr::ColorReset => Some("Reset style".into()),
        Sgr::Bold => Some("Bold".into()),
        Sgr::Dim => Some("Faint".into()),
        Sgr::Italic => Some("Italic".into()),
        Sgr::Underline => Some("Underline".into()),
        Sgr::SlowBlink | Sgr::RapidBlink => Some("Blink".into()),
        Sgr::Reverse => Some("Inverse".into()),
        Sgr::CursorHide => Some("Invisible".into()),
        Sgr::Strike => Some("Crossed-out".into()),
        Sgr::NormalIntensity => Some("Normal intensity".into()),
        Sgr::UnItalic => Some("No italic".into()),
        Sgr::UnUnderlined => Some("No underline".into()),
        Sgr::UnBlink => Some("No blink".into()),
        Sgr::UnReversed => Some("No reverse".into()),
        Sgr::UnHide => Some("No conceal".into()),
        Sgr::UnStrike => Some("No crossed-out".into()),
        Sgr::ColorForeground(c) => {
            let (_, desc) = describe_color_sgr(c, true);
            Some(desc)
        }
        Sgr::ColorBackground(c) => {
            let (_, desc) = describe_color_sgr(c, false);
            Some(desc)
        }
        Sgr::ColorDefaultForeground => Some("Default foreground color".into()),
        Sgr::ColorDefaultBackground => Some("Default background color".into()),
        Sgr::ColorDefaultUnderline => Some("Default underline color".into()),
        _ => None,
    }
}

/// C0 control code names.
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
        0x0A => "Line feed",
        0x0B => "Vertical tab",
        0x0C => "Form feed",
        0x0D => "Carriage return",
        0x0E => "Shift out",
        0x0F => "Shift in",
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
        0x1A => "Substitute",
        0x1B => "Escape",
        0x1C => "File separator",
        0x1D => "Group separator",
        0x1E => "Record separator",
        0x1F => "Unit separator",
        0x7F => "Delete",
        _ => "Unknown",
    }
}

/// Escape a byte to its Go-style `%q` representation for sequin display.
fn escape_ctrl_byte(byte: u8) -> String {
    match byte {
        0x07 => "\\a".into(),
        0x08 => "\\b".into(),
        0x09 => "\\t".into(),
        0x0A => "\\n".into(),
        0x0B => "\\v".into(),
        0x0C => "\\f".into(),
        0x0D => "\\r".into(),
        0x1B => "ESC".into(),
        _ => format!("\\x{:02x}", byte),
    }
}

/// Convert ANSI input to sequin-style human-readable output.
///
/// This produces output matching the Go sequin project's format:
/// each ANSI sequence gets a line showing its kind, raw parameters, and explanation.
pub fn to_sequin<S: AsRef<str>>(input: S, _width: Option<usize>) -> String {
    let s = input.as_ref();
    let tokens = match parse_ansi(s) {
        Ok((_, tokens)) => tokens,
        Err(_) => return String::new(),
    };

    let mut output = String::new();
    let mut text_buf = String::new();

    let flush_text = |buf: &mut String, out: &mut String| {
        if !buf.is_empty() {
            out.push_str(&fmt_text(buf));
            buf.clear();
        }
    };

    for token in &tokens {
        match &token.sgr {
            // Text characters get accumulated
            Sgr::Char(c) => {
                text_buf.push(*c);
            }

            // Control codes
            Sgr::Bell => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "Ctrl",
                    &escape_ctrl_byte(0x07),
                    ctrl_code_name(0x07),
                ));
            }
            Sgr::Backspace => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "Ctrl",
                    &escape_ctrl_byte(0x08),
                    ctrl_code_name(0x08),
                ));
            }
            Sgr::Tab => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "Ctrl",
                    &escape_ctrl_byte(0x09),
                    ctrl_code_name(0x09),
                ));
            }
            Sgr::LineFeed => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "Ctrl",
                    &escape_ctrl_byte(0x0A),
                    ctrl_code_name(0x0A),
                ));
            }
            Sgr::FormFeed => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "Ctrl",
                    &escape_ctrl_byte(0x0C),
                    ctrl_code_name(0x0C),
                ));
            }
            Sgr::CarriageReturn => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "Ctrl",
                    &escape_ctrl_byte(0x0D),
                    ctrl_code_name(0x0D),
                ));
            }

            // SGR (Select Graphic Rendition) - CSI sequences
            Sgr::ColorReset => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "m", "Reset style"));
            }
            Sgr::Bold => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "1m", "Bold"));
            }
            Sgr::Dim => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "2m", "Faint"));
            }
            Sgr::Italic => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "3m", "Italic"));
            }
            Sgr::Underline => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "4m", "Underline"));
            }
            Sgr::SlowBlink => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "5m", "Blink"));
            }
            Sgr::RapidBlink => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "6m", "Blink"));
            }
            Sgr::Reverse => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "7m", "Inverse"));
            }
            Sgr::Strike => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "9m", "Crossed-out"));
            }
            Sgr::NormalIntensity => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "22m", "Normal intensity"));
            }
            Sgr::UnItalic => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "23m", "No italic"));
            }
            Sgr::UnUnderlined => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "24m", "No underline"));
            }
            Sgr::UnBlink => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "25m", "No blink"));
            }
            Sgr::UnReversed => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "27m", "No reverse"));
            }
            Sgr::UnHide => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "28m", "No conceal"));
            }
            Sgr::UnStrike => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "29m", "No crossed-out"));
            }

            // Foreground colors
            Sgr::ColorForeground(c) => {
                flush_text(&mut text_buf, &mut output);
                let (seq, desc) = describe_color_sgr(c, true);
                output.push_str(&fmt_line("CSI", &seq, &desc));
            }
            Sgr::ColorBackground(c) => {
                flush_text(&mut text_buf, &mut output);
                let (seq, desc) = describe_color_sgr(c, false);
                output.push_str(&fmt_line("CSI", &seq, &desc));
            }
            Sgr::ColorDefaultForeground => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "39m", "Default foreground color"));
            }
            Sgr::ColorDefaultBackground => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "49m", "Default background color"));
            }
            Sgr::ColorDefaultUnderline => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "59m", "Default underline color"));
            }
            Sgr::ColorUnderLine(c) => {
                flush_text(&mut text_buf, &mut output);
                let (seq, desc) = describe_underline_color_sgr(c);
                output.push_str(&fmt_line("CSI", &seq, &desc));
            }
            Sgr::ColorFgBg(fg, bg) => {
                flush_text(&mut text_buf, &mut output);
                // Output as a combined sequence
                let (fg_seq, fg_desc) = describe_color_sgr(fg, true);
                let (bg_seq, bg_desc) = describe_color_sgr(bg, false);
                let seq = format!(
                    "{}{}m",
                    fg_seq.trim_end_matches('m'),
                    if bg_seq.is_empty() {
                        String::new()
                    } else {
                        format!(";{}", bg_seq.trim_end_matches('m'))
                    }
                );
                let desc = format!("{}, {}", fg_desc, bg_desc);
                output.push_str(&fmt_line("CSI", &seq, &desc));
            }

            // Cursor movement - CSI sequences
            Sgr::CursorUp(n) => {
                flush_text(&mut text_buf, &mut output);
                let seq = if *n == 1 {
                    "A".into()
                } else {
                    format!("{}A", n)
                };
                output.push_str(&fmt_line("CSI", &seq, &format!("Cursor up {}", n)));
            }
            Sgr::CursorDown(n) => {
                flush_text(&mut text_buf, &mut output);
                let seq = if *n == 1 {
                    "B".into()
                } else {
                    format!("{}B", n)
                };
                output.push_str(&fmt_line("CSI", &seq, &format!("Cursor down {}", n)));
            }
            Sgr::CursorForward(n) => {
                flush_text(&mut text_buf, &mut output);
                let seq = if *n == 1 {
                    "C".into()
                } else {
                    format!("{}C", n)
                };
                output.push_str(&fmt_line("CSI", &seq, &format!("Cursor right {}", n)));
            }
            Sgr::CursorBack(n) => {
                flush_text(&mut text_buf, &mut output);
                let seq = if *n == 1 {
                    "D".into()
                } else {
                    format!("{}D", n)
                };
                output.push_str(&fmt_line("CSI", &seq, &format!("Cursor left {}", n)));
            }
            Sgr::CursorNextLine(n) => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "CSI",
                    &format!("{}E", n),
                    &format!("Cursor next line {}", n),
                ));
            }
            Sgr::CursorPreviousLine(n) => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "CSI",
                    &format!("{}F", n),
                    &format!("Cursor previous line {}", n),
                ));
            }
            Sgr::CursorHorizontalAbsolute(n) => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "CSI",
                    &format!("{}G", n),
                    &format!("Cursor horizontal absolute {}", n),
                ));
            }
            Sgr::CursorPosition(row, col) => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "CSI",
                    &format!("{};{}H", row, col),
                    &format!("Set cursor position row={} col={}", row, col),
                ));
            }
            Sgr::CursorSave => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "s", "Save cursor position"));
            }
            Sgr::CursorRestore => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "u", "Restore cursor position"));
            }

            // Erase sequences
            Sgr::EraseInDisplay(n) => {
                flush_text(&mut text_buf, &mut output);
                let desc = match n {
                    0 => "Erase screen bellow",
                    1 => "Erase screen above",
                    2 => "Erase entire screen",
                    3 => "Erase entire display",
                    _ => "Erase screen",
                };
                let seq = if *n == 0 {
                    "J".into()
                } else {
                    format!("{}J", n)
                };
                output.push_str(&fmt_line("CSI", &seq, desc));
            }
            Sgr::EraseInLine(n) => {
                flush_text(&mut text_buf, &mut output);
                let desc = match n {
                    0 => "Erase line right",
                    1 => "Erase line left",
                    2 => "Erase entire line",
                    _ => "Erase line",
                };
                let seq = if *n == 0 {
                    "K".into()
                } else {
                    format!("{}K", n)
                };
                output.push_str(&fmt_line("CSI", &seq, desc));
            }
            Sgr::ScrollUp(n) => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "CSI",
                    &format!("{}S", n),
                    &format!("Scroll up {} lines", n.max(&1)),
                ));
            }
            Sgr::ScrollDown(n) => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "CSI",
                    &format!("{}T", n),
                    &format!("Scroll down {} lines", n.max(&1)),
                ));
            }

            // Device status
            Sgr::DeviceStatusReport => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "6n", "Request cursor position"));
            }
            Sgr::AUXPortOn => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "5i", "AUX port on"));
            }
            Sgr::AUXPortOff => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "4i", "AUX port off"));
            }
            Sgr::HorizontalVerticalPosition(x, y) => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "CSI",
                    &format!("{};{}f", x, y),
                    &format!("Horizontal vertical position row={} col={}", x, y),
                ));
            }

            // OSC sequences
            Sgr::Title(title) => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "OSC",
                    &format!("0;{}", title),
                    &format!("Set icon name and window title to \"{}\"", title),
                ));
            }
            Sgr::Cwd(url) => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "OSC",
                    &format!("7;{}", url),
                    &format!("Set working directory to {}", url),
                ));
            }
            Sgr::Prompt => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("OSC", "133", "Shell integration"));
            }
            Sgr::Link(url, title) => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "OSC",
                    &format!("8;;{}", url),
                    &format!("Set hyperlink, to \"{}\" title \"{}\"", url, title),
                ));
            }

            // Mode/DECSET/DECRST
            Sgr::Decset => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "CSI",
                    "?2004h",
                    "Enable private mode \"bracketed paste\"",
                ));
            }
            Sgr::Decrst => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "CSI",
                    "?2004l",
                    "Disable private mode \"bracketed paste\"",
                ));
            }
            Sgr::CursorShow => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "CSI",
                    "?25h",
                    "Enable private mode \"cursor visibility\"",
                ));
            }
            Sgr::Keypad => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("ESC", "=", "Application Keypad"));
            }
            Sgr::Ext => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", ">4;1m", "Extended mode"));
            }

            // List of SGR attributes
            Sgr::List(items) => {
                flush_text(&mut text_buf, &mut output);
                let descriptions: Vec<String> =
                    items.iter().filter_map(describe_sgr).collect();
                if !descriptions.is_empty() {
                    output.push_str(&fmt_line("CSI", "m", &descriptions.join(", ")));
                }
            }

            // Unknown control codes
            Sgr::Unknown(byte) => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "Ctrl",
                    &escape_ctrl_byte(*byte),
                    ctrl_code_name(*byte),
                ));
            }

            // Other attributes
            Sgr::PrimaryFont => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "10m", "Primary font"));
            }
            Sgr::AlternativeFont(n) => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line(
                    "CSI",
                    &format!("{}m", 10 + n),
                    &format!("Alternative font {}", n),
                ));
            }
            Sgr::Fraktur => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "20m", "Fraktur"));
            }
            Sgr::DoublyUnderlined => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "21m", "Doubly underlined"));
            }
            Sgr::CursorHide => {
                flush_text(&mut text_buf, &mut output);
                output.push_str(&fmt_line("CSI", "8m", "Invisible"));
            }
        }
    }

    // Flush remaining text
    flush_text(&mut text_buf, &mut output);

    output
}

/// Describe a color for SGR with its sequence fragment and description.
fn describe_color_sgr(c: &AnsiColor, is_fg: bool) -> (String, String) {
    let (prefix, label) = if is_fg {
        ("38", "foreground")
    } else {
        ("48", "background")
    };

    match c {
        AnsiColor::Color8(c8) => {
            let n = c8.to_u8();
            let color_name = color8_name(c8);
            (
                format!("{}m", n),
                format!("ANSI {} color: {}", label, color_name),
            )
        }
        AnsiColor::Color256(idx) => {
            let i = *idx as usize;
            if i < 8 {
                let code = if is_fg { 30 + idx } else { 40 + idx };
                (
                    format!("{}m", code),
                    format!("ANSI {} color: {}", label, basic_color_name(*idx)),
                )
            } else if i < 16 {
                let code = if is_fg { 82 + idx } else { 92 + idx };
                (
                    format!("{}m", code),
                    format!("ANSI {} color: Bright {}", label, basic_color_name(idx - 8)),
                )
            } else {
                let hex = color256_hex(i);
                (
                    format!("{};5;{}m", prefix, idx),
                    format!("ANSI256 {} color: {} (#{})", label, idx, hex),
                )
            }
        }
        AnsiColor::Rgb(r, g, b) => (
            format!("{};2;{};{};{}m", prefix, r, g, b),
            format!("24-bit RGB {} color: #{:02X}{:02X}{:02X}", label, r, g, b),
        ),
        AnsiColor::Default => {
            let code = if is_fg { 39 } else { 49 };
            (format!("{}m", code), format!("Default {} color", label))
        }
    }
}

/// Describe an underline color for SGR.
fn describe_underline_color_sgr(c: &AnsiColor) -> (String, String) {
    match c {
        AnsiColor::Color8(c8) => {
            let color_name = color8_name(c8);
            (
                format!("58;5;{}m", c8.to_u8()),
                format!("ANSI underline color: {}", color_name),
            )
        }
        AnsiColor::Color256(idx) => {
            let i = *idx as usize;
            if i < 8 {
                (
                    format!("58;5;{}m", idx),
                    format!("ANSI underline color: {}", basic_color_name(*idx)),
                )
            } else if i < 16 {
                (
                    format!("58;5;{}m", idx),
                    format!("ANSI underline color: Bright {}", basic_color_name(idx - 8)),
                )
            } else {
                let hex = color256_hex(i);
                (
                    format!("58;5;{}m", idx),
                    format!("ANSI256 underline color: {} (#{})", idx, hex),
                )
            }
        }
        AnsiColor::Rgb(r, g, b) => (
            format!("58;2;{};{};{}m", r, g, b),
            format!("24-bit RGB underline color: #{:02X}{:02X}{:02X}", r, g, b),
        ),
        AnsiColor::Default => ("59m".into(), "Default underline color".into()),
    }
}
