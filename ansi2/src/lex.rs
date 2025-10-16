use crate::color::{AnsiColor, Color8};
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_until, take_while1};
use nom::character::complete::{anychar, digit0};
use nom::combinator::opt;
use nom::multi::many0;

#[derive(Debug, Clone)]
pub enum Sgr {
    Char(char),

    Bell,
    Backspace,
    Tab,
    LineFeed,
    FormFeed,
    CarriageReturn,
    Title(String),
    Cwd(String),
    Prompt,
    CursorUp(i32),
    CursorDown(i32),
    CursorForward(i32),
    CursorBack(i32),
    CursorNextLine(i32),
    CursorPreviousLine(i32),
    CursorHorizontalAbsolute(i32),
    CursorPosition(i32, i32),
    EraseInDisplay(i32),
    EraseInLine(i32),
    ScrollUp(i32),
    ScrollDown(i32),
    HorizontalVerticalPosition(i32, i32),
    AUXPortOn,
    AUXPortOff,
    DeviceStatusReport,

    CursorSave,
    CursorRestore,

    CursorHide,
    CursorShow,
    ColorForeground(AnsiColor),
    ColorBackground(AnsiColor),
    ColorUnderLine(AnsiColor),
    ColorFgBg(AnsiColor, AnsiColor),

    ColorReset,
    Reverse,
    ColorDefaultForeground,
    ColorDefaultBackground,
    ColorDefaultUnderline,

    Bold,
    NormalIntensity,
    Italic,
    Underline,
    Dim,
    SlowBlink,
    RapidBlink,
    Strike,
    UnStrike,
    UnItalic,
    PrimaryFont,
    DoublyUnderlined,
    UnUnderlined,
    UnBlink,

    List(Vec<Sgr>),

    // url, title
    Link(String, String),

    AlternativeFont(u8),
    Fraktur,
    UnReversed,
    UnHide,
    Unknown(u8),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub sgr: Sgr,
    pub range: (usize, usize),
}

fn get_sgr_color(n: u8) -> Sgr {
    match n {
        39 => Sgr::ColorDefaultForeground,
        49 => Sgr::ColorDefaultBackground,
        30..=37 | 90..=97 => Sgr::ColorForeground(AnsiColor::from_u8(n)),
        40..=47 | 100..=107 => Sgr::ColorBackground(AnsiColor::from_u8(n)),
        _ => Sgr::Unknown(0),
    }
}

fn parse_cursor_up(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b["), digit0, tag_no_case("a")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::CursorUp(str::parse(b).unwrap_or(1)),
        },
    ))
}

fn parse_cursor_down(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b["), digit0, tag_no_case("b")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::CursorDown(str::parse(b).unwrap_or(1)),
        },
    ))
}

fn parse_cursor_forward(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b["), digit0, tag_no_case("c")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::CursorForward(str::parse(b).unwrap_or(1)),
        },
    ))
}

fn parse_cursor_back(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b["), digit0, tag_no_case("d")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::CursorBack(str::parse(b).unwrap_or(1)),
        },
    ))
}

fn parse_cursor_next_line(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b["), digit0, tag_no_case("e")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::CursorNextLine(str::parse(b).unwrap_or(1)),
        },
    ))
}

fn parse_cursor_previous_line(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b["), digit0, tag_no_case("f")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::CursorPreviousLine(str::parse(b).unwrap_or(1)),
        },
    ))
}

fn parse_cursor_horizontal(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b["), digit0, tag_no_case("g")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::CursorHorizontalAbsolute(str::parse(b).unwrap_or(1)),
        },
    ))
}

fn parse_cursor_position(input: &str) -> IResult<&str, Token> {
    let (rem, (_, x, _, y, _)) =
        (tag("\x1b["), digit0, tag(":"), digit0, tag_no_case("h")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::CursorPosition(str::parse(x).unwrap_or(0), str::parse(y).unwrap_or(0)),
        },
    ))
}

fn parse_erase_in_display(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b["), digit0, tag_no_case("j")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::EraseInDisplay(str::parse(b).unwrap_or(0)),
        },
    ))
}

fn parse_erase_in_line(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b["), digit0, tag_no_case("k")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::EraseInLine(str::parse(b).unwrap_or(0)),
        },
    ))
}

fn parse_scroll_up(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b["), digit0, tag_no_case("s")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::ScrollUp(str::parse(b).unwrap_or(0)),
        },
    ))
}
fn parse_scroll_down(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b["), digit0, tag_no_case("t")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::ScrollDown(str::parse(b).unwrap_or(0)),
        },
    ))
}

fn parse_horizontal_vertical_position(input: &str) -> IResult<&str, Token> {
    let (rem, (_, x, _, y, _)) =
        (tag("\x1b["), digit0, tag(":"), digit0, tag_no_case("f")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::HorizontalVerticalPosition(
                str::parse(x).unwrap_or(0),
                str::parse(y).unwrap_or(0),
            ),
        },
    ))
}

fn parse_aux_port_on(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tag("\x1b5i").parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::AUXPortOn,
        },
    ))
}
fn parse_aux_port_off(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tag("\x1b[4i").parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::AUXPortOff,
        },
    ))
}
fn parse_device_status_report(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tag("\x1b[6n").parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::DeviceStatusReport,
        },
    ))
}

fn parse_cursor_hide(input: &str) -> IResult<&str, Token> {
    let (rem, _) = (tag("\x1b["), opt(tag("?")), digit0, tag_no_case("l")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::CursorHide,
        },
    ))
}
fn parse_cursor_hide_windows(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tag("\x1b[?2004h").parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::CursorHide,
        },
    ))
}

fn parse_title(input: &str) -> IResult<&str, Token> {
    let (rem, (_, s, _)) = (tag("\x1b]0;"), take_until("\x07"), tag("\x07")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::Title(s.into()),
        },
    ))
}

fn parse_cwd(input: &str) -> IResult<&str, Token> {
    let (rem, (_, s, _)) = (tag("\x1b]7;"), take_until("\x07"), tag("\x07")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::Cwd(s.into()),
        },
    ))
}
fn parse_prompt(input: &str) -> IResult<&str, Token> {
    let (rem, (_, _)) = (
        tag("\x1b]133;"),
        take_while1(|c: char| !(c.is_control() || c.is_whitespace())),
    )
        .parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::Prompt,
        },
    ))
}
fn parse_bold(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tag("\x1b(B").parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::Bold,
        },
    ))
}

fn parse_cursor_show(input: &str) -> IResult<&str, Token> {
    let (rem, _) = (tag("\x1b["), opt(tag("?")), digit0, tag_no_case("h")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::CursorShow,
        },
    ))
}

fn parse_color_foreground(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b[38;5;"), digit0, tag_no_case("m")).parse(input)?;

    let b = str::parse(b).unwrap_or_default();

    let c = match b {
        0..=7 => b + 30,
        8..=15 => b + 82,
        _ => {
            return Ok((
                rem,
                Token {
                    range: (input.chars().count(), rem.chars().count()),
                    sgr: Sgr::ColorForeground(AnsiColor::Color256(b)),
                },
            ));
        }
    };
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::ColorForeground(AnsiColor::from_u8(c)),
        },
    ))
}
fn parse_color_background(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b[48;5;"), digit0, tag_no_case("m")).parse(input)?;
    let b = str::parse(b).unwrap_or_default();

    let c = match b {
        0..=7 => b + 40,
        8..=15 => b + 92,
        _ => {
            return Ok((
                rem,
                Token {
                    range: (input.chars().count(), rem.chars().count()),
                    sgr: Sgr::ColorBackground(AnsiColor::Color256(b)),
                },
            ));
        }
    };
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::ColorBackground(AnsiColor::from_u8(c)),
        },
    ))
}

fn parse_color_underline(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b[58;5;"), digit0, tag_no_case("m")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::ColorUnderLine(AnsiColor::from_u8(str::parse(b).unwrap_or(1))),
        },
    ))
}

pub fn get_sgr(n: u8) -> Sgr {
    match n {
        0 => Sgr::ColorReset,
        1 => Sgr::Bold,
        2 => Sgr::Dim,
        3 => Sgr::Italic,
        4 => Sgr::Underline,
        5 => Sgr::SlowBlink,
        6 => Sgr::RapidBlink,
        7 => Sgr::Reverse,
        8 => Sgr::CursorHide,
        9 => Sgr::Strike,
        10 => Sgr::PrimaryFont,
        11..=19 => Sgr::AlternativeFont(n - 10),
        20 => Sgr::Fraktur,
        21 => Sgr::DoublyUnderlined,
        24 => Sgr::UnUnderlined,
        25 => Sgr::UnBlink,
        30..=37 | 90..=97 => Sgr::ColorForeground(AnsiColor::from_u8(n)),
        40..=47 | 100..=107 => Sgr::ColorBackground(AnsiColor::from_u8(n)),
        39 => Sgr::ColorDefaultForeground,
        49 => Sgr::ColorDefaultBackground,
        59 => Sgr::ColorDefaultUnderline,
        22 => Sgr::NormalIntensity,
        27 => Sgr::UnReversed,
        29 => Sgr::UnStrike,
        28 => Sgr::UnHide,
        23 => Sgr::UnItalic,

        _ => Sgr::Unknown(n),
    }
}

fn parse_sgr1(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = (tag("\x1b["), digit0, tag_no_case("m")).parse(input)?;

    let n: u8 = str::parse(b).unwrap_or_default();

    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: get_sgr(n),
        },
    ))
}

fn parse_color_reset(input: &str) -> IResult<&str, Token> {
    let (rem, _) = (tag("\x1b[0"), tag_no_case("m")).parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::ColorReset,
        },
    ))
}

fn parse_anychar(input: &str) -> IResult<&str, Token> {
    let (rem, c) = anychar(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::Char(c),
        },
    ))
}

fn parse_bell(input: &str) -> IResult<&str, Token> {
    let (rem, _) = nom::character::complete::char('\x07').parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::Bell,
        },
    ))
}

fn parse_backspace(input: &str) -> IResult<&str, Token> {
    let (rem, _) = nom::character::complete::char('\x08').parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::Backspace,
        },
    ))
}

fn parse_tab(input: &str) -> IResult<&str, Token> {
    let (rem, _) = nom::character::complete::char('\x09').parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::Bell,
        },
    ))
}

fn parse_line_feed(input: &str) -> IResult<&str, Token> {
    let (rem, _) = nom::character::complete::char('\x0A').parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::LineFeed,
        },
    ))
}

fn parse_form_feed(input: &str) -> IResult<&str, Token> {
    let (rem, _) = nom::character::complete::char('\x0C').parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::FormFeed,
        },
    ))
}

fn parse_carriage_return(input: &str) -> IResult<&str, Token> {
    let (rem, _) = nom::character::complete::char('\x0D').parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::CarriageReturn,
        },
    ))
}

fn parse_sgr2(input: &str) -> IResult<&str, Token> {
    let (rem, (_, front, _, background, _)) =
        (tag("\x1b["), digit0, tag(";"), digit0, tag_no_case("m")).parse(input)?;

    let a = front.parse().unwrap_or(0);
    let b = background.parse().unwrap_or(0);
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::List(vec![get_sgr(a), get_sgr_color(b)]),
        },
    ))
}

fn parse_sgr3(input: &str) -> IResult<&str, Token> {
    let (rem, (_, ctrl, _, front, _, background, _)) = (
        tag("\x1b["),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag_no_case("m"),
    )
        .parse(input)?;

    let a = ctrl.parse().unwrap_or(0);
    let b = front.parse().unwrap_or(0);
    let c = background.parse().unwrap_or(0);

    if a == 38 && b == 5 {
        return Ok((
            rem,
            Token {
                range: (input.chars().count(), rem.chars().count()),
                sgr: Sgr::ColorForeground(AnsiColor::from_u8(c)),
            },
        ));
    }
    if a == 48 && b == 5 {
        return Ok((
            rem,
            Token {
                range: (input.chars().count(), rem.chars().count()),
                sgr: Sgr::ColorBackground(AnsiColor::from_u8(c)),
            },
        ));
    }
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::List(vec![get_sgr(a), get_sgr_color(b), get_sgr_color(c)]),
        },
    ))
}

fn parse_sgr4(input: &str) -> IResult<&str, Token> {
    let (rem, (_, reset, _, ctrl, _, front, _, background, _)) = (
        tag("\x1b["),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag_no_case("m"),
    )
        .parse(input)?;
    let a: u8 = reset.parse().unwrap_or(0);
    let b: u8 = ctrl.parse().unwrap_or(0);
    let c: u8 = front.parse().unwrap_or(0);
    let d: u8 = background.parse().unwrap_or(0);

    let mut v = vec![get_sgr(a)];

    match b {
        48 => match c {
            5 => {
                v.push(Sgr::ColorBackground(AnsiColor::Color256(d)));
            }
            _ => {
                v.push(Sgr::ColorBackground(AnsiColor::from_u8(d)));
            }
        },
        38 => match c {
            5 => {
                v.push(Sgr::ColorForeground(AnsiColor::Color256(d)));
            }
            _ => {
                v.push(Sgr::ColorForeground(AnsiColor::from_u8(d)));
            }
        },
        5 | 6 => {
            v.push(get_sgr(b));
            v.push(Sgr::ColorForeground(AnsiColor::from_u8(c)));
            v.push(Sgr::ColorBackground(AnsiColor::from_u8(d)));
        }
        _ => {
            v.push(get_sgr(b));
            v.push(Sgr::ColorForeground(AnsiColor::Color256(c)));
            v.push(Sgr::ColorBackground(AnsiColor::Color256(d)));
        }
    };

    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::List(v),
        },
    ))
}

fn parse_sgr5(input: &str) -> IResult<&str, Token> {
    let (rem, (_, ctrl, _, ty, _, r, _, g, _, b, _)) = (
        tag("\x1b["),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag_no_case("m"),
    )
        .parse(input)?;
    let ctrl = ctrl.parse().unwrap_or(0);
    let ty = ty.parse().unwrap_or(0);
    let r = r.parse().unwrap_or(0);
    let g = g.parse().unwrap_or(0);
    let b = b.parse().unwrap_or(0);
    if ctrl == 38 && ty == 2 {
        return Ok((
            rem,
            Token {
                range: (input.chars().count(), rem.chars().count()),
                sgr: Sgr::ColorForeground(AnsiColor::Rgb(r, g, b)),
            },
        ));
    }

    if ctrl == 48 && ty == 2 {
        return Ok((
            rem,
            Token {
                range: (input.chars().count(), rem.chars().count()),
                sgr: Sgr::ColorBackground(AnsiColor::Rgb(r, g, b)),
            },
        ));
    }
    todo!()
}

fn parse_sgr6(input: &str) -> IResult<&str, Token> {
    let (rem, (_, ctrl, _, ty, _, r, _, g, _, b, _, n, _)) = (
        tag("\x1b["),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag_no_case("m"),
    )
        .parse(input)?;
    let ctrl = ctrl.parse().unwrap_or(0);
    let ty = ty.parse().unwrap_or(0);
    let r = r.parse().unwrap_or(0);
    let g = g.parse().unwrap_or(0);
    let b = b.parse().unwrap_or(0);
    let n = n.parse().unwrap_or(0);
    if ctrl == 38 && ty == 2 {
        return Ok((
            rem,
            Token {
                range: (input.chars().count(), rem.chars().count()),
                sgr: Sgr::ColorFgBg(AnsiColor::Rgb(r, g, b), AnsiColor::from_u8(n)),
            },
        ));
    }

    if ctrl == 48 && ty == 2 {
        return Ok((
            rem,
            Token {
                range: (input.chars().count(), rem.chars().count()),
                sgr: Sgr::ColorFgBg(AnsiColor::from_u8(n), AnsiColor::Rgb(r, g, b)),
            },
        ));
    }

    if ctrl == 48 && ty == 5 && g == 38 && b == 5 {
        return Ok((
            rem,
            Token {
                range: (input.chars().count(), rem.chars().count()),
                sgr: Sgr::ColorFgBg(AnsiColor::Color256(n), AnsiColor::Color256(r)),
            },
        ));
    }
    if ctrl == 38 && ty == 5 && g == 48 && b == 5 {
        return Ok((
            rem,
            Token {
                range: (input.chars().count(), rem.chars().count()),
                sgr: Sgr::ColorFgBg(AnsiColor::Color256(r), AnsiColor::Color256(n)),
            },
        ));
    }
    todo!()
}

fn parse_sgr7(input: &str) -> IResult<&str, Token> {
    let (rem, (_, ctrl, _, a, _, b, _, c, _, d, _, e, _, f, _)) = (
        tag("\x1b["),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag_no_case("m"),
    )
        .parse(input)?;
    let ctrl = ctrl.parse().unwrap_or(0);
    let a = a.parse().unwrap_or(0);
    let b = b.parse().unwrap_or(0);
    let c = c.parse().unwrap_or(0);
    let d = d.parse().unwrap_or(0);
    let e = e.parse().unwrap_or(0);
    let f = f.parse().unwrap_or(0);
    let mut v = vec![get_sgr(ctrl)];

    match a {
        38 => match b {
            5 => {
                v.push(Sgr::ColorForeground(AnsiColor::Color256(c)));
            }
            2 => {
                v.push(Sgr::ColorForeground(AnsiColor::Color8(Color8::from_u8(c))));
            }
            _ => {}
        },
        48 => match b {
            5 => {
                v.push(Sgr::ColorBackground(AnsiColor::Color256(c)));
            }
            2 => {
                v.push(Sgr::ColorBackground(AnsiColor::Color8(Color8::from_u8(c))));
            }
            _ => {}
        },
        _ => {}
    }

    match d {
        38 => match e {
            5 => {
                v.push(Sgr::ColorForeground(AnsiColor::Color256(f)));
            }
            2 => {
                v.push(Sgr::ColorForeground(AnsiColor::Color8(Color8::from_u8(f))));
            }
            _ => {}
        },
        48 => match e {
            5 => {
                v.push(Sgr::ColorBackground(AnsiColor::Color256(f)));
            }
            2 => {
                v.push(Sgr::ColorBackground(AnsiColor::Color8(Color8::from_u8(f))));
            }
            _ => {}
        },
        _ => {}
    }

    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::List(v),
        },
    ))
}

fn parse_sgr10(input: &str) -> IResult<&str, Token> {
    let (rem, (_, c1, _, t1, _, r1, _, g1, _, b1, _, c2, _, t2, _, r2, _, g2, _, b2, _)) = (
        tag("\x1b["),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag_no_case("m"),
    )
        .parse(input)?;
    let mut fg = AnsiColor::Default;
    let mut bg = AnsiColor::Default;
    let c1: u8 = c1.parse().unwrap_or_default();
    let t1: u8 = t1.parse().unwrap_or_default();
    let r1: u8 = r1.parse().unwrap_or_default();
    let g1: u8 = g1.parse().unwrap_or_default();
    let b1: u8 = b1.parse().unwrap_or_default();
    if c1 == 38 && t1 == 2 {
        fg = AnsiColor::Rgb(r1, g1, b1)
    }
    if c1 == 48 && t1 == 2 {
        bg = AnsiColor::Rgb(r1, g1, b1)
    }

    let c2: u8 = c2.parse().unwrap_or_default();
    let t2: u8 = t2.parse().unwrap_or_default();
    let r2: u8 = r2.parse().unwrap_or_default();
    let g2: u8 = g2.parse().unwrap_or_default();
    let b2: u8 = b2.parse().unwrap_or_default();
    if c2 == 38 && t2 == 2 {
        fg = AnsiColor::Rgb(r2, g2, b2)
    }
    if c2 == 48 && t2 == 2 {
        bg = AnsiColor::Rgb(r2, g2, b2)
    }
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::ColorFgBg(fg, bg),
        },
    ))
}
fn parse_unknown(input: &str) -> IResult<&str, Token> {
    let (rem, n) = alt((
        nom::character::complete::char('\x00'),
        nom::character::complete::char('\x01'),
        nom::character::complete::char('\x02'),
        nom::character::complete::char('\x03'),
        nom::character::complete::char('\x04'),
        nom::character::complete::char('\x05'),
        nom::character::complete::char('\x06'),
        nom::character::complete::char('\x0e'),
        nom::character::complete::char('\x0f'),
        nom::character::complete::char('\x10'),
        nom::character::complete::char('\x11'),
        nom::character::complete::char('\x12'),
        nom::character::complete::char('\x14'),
        nom::character::complete::char('\x16'),
        nom::character::complete::char('\x19'),
        nom::character::complete::char('\x1a'),
        nom::character::complete::char('\x1b'),
        nom::character::complete::char('\x1c'),
        nom::character::complete::char('\x1e'),
    ))
    .parse(input)?;

    if n == '\x1a' {
        return Ok((
            rem,
            Token {
                range: (input.chars().count(), rem.chars().count()),
                sgr: Sgr::Char('␦'),
            },
        ));
    }

    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::Unknown(n as u8),
        },
    ))
}

fn parse_link_no_title(input: &str) -> IResult<&str, Token> {
    let (rem, (_, _, url, _)) = (
        tag("\x1b]8;"),
        opt(tag(";")),
        alt((take_until("\x1b]8;;\x1b\\"), take_until("\x1b[!p"))),
        alt((tag("\x1b]8;;\x1b\\"), tag("\x1b[!p"))),
    )
        .parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::Link(url.to_string(), url.to_string()),
        },
    ))
}

fn parse_link_with_title(input: &str) -> IResult<&str, Token> {
    let (rem, (_, _, url, _, title, _)) = (
        tag("\x1b]8;"),
        opt(tag(";")),
        take_until("\x1b\\"),
        tag("\x1b\\"),
        alt((take_until("\x1b]8;;\x1b\\"), take_until("\x1b[!p"))),
        alt((tag("\x1b]8;;\x1b\\"), tag("\x1b[!p"))),
    )
        .parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::Link(url.to_string(), title.to_string()),
        },
    ))
}

fn parse_link_ll(input: &str) -> IResult<&str, Token> {
    let (rem, (_, url, _, title, _)) = (
        tag("\x1b]8;;"),
        take_until("\x07"),
        tag("\x07"),
        take_until("\x1b]8;;\x07"),
        tag("\x1b]8;;\x07"),
    )
        .parse(input)?;
    Ok((
        rem,
        Token {
            range: (input.chars().count(), rem.chars().count()),
            sgr: Sgr::Link(url.to_string(), title.to_string()),
        },
    ))
}

pub(crate) fn parse_ansi(input: &str) -> IResult<&str, Vec<Token>> {
    let mut v = many0(alt((
        alt((
            parse_bell,
            parse_backspace,
            parse_tab,
            parse_line_feed,
            parse_form_feed,
            parse_carriage_return,
            parse_title,
        )),
        alt((
            parse_cursor_up,
            parse_cursor_down,
            parse_cursor_forward,
            parse_cursor_back,
            parse_cursor_next_line,
            parse_cursor_previous_line,
            parse_cursor_horizontal,
            parse_cursor_position,
            parse_erase_in_display,
            parse_erase_in_line,
            parse_scroll_up,
            parse_scroll_down,
            parse_horizontal_vertical_position,
            parse_aux_port_on,
            parse_aux_port_off,
            parse_device_status_report,
        )),
        alt((
            parse_bold,
            parse_cursor_hide,
            parse_cursor_hide_windows,
            parse_cursor_show,
            parse_color_foreground,
            parse_color_background,
            parse_color_underline,
            parse_color_reset,
            parse_sgr1,
        )),
        alt((
            parse_sgr2,
            parse_sgr3,
            parse_sgr4,
            parse_sgr5,
            parse_sgr6,
            parse_sgr7,
            parse_sgr10,
        )),
        alt((
            parse_link_with_title,
            parse_link_no_title,
            parse_link_ll,
            parse_cwd,
            parse_prompt,
        )),
        parse_unknown,
        parse_anychar,
    )))
    .parse(input)?;

    for i in v.1.iter_mut() {
        i.range = (
            input.chars().count() - i.range.0,
            input.chars().count() - i.range.1,
        )
    }
    Ok(v)
}
