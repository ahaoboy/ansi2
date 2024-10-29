use crate::color::{AnsiColor, Color8};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_until};
use nom::character::complete::{anychar, digit0};
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Clone)]
pub enum Token {
    Char(char),

    Bell,
    Backspace,
    Tab,
    LineFeed,
    FormFeed,
    CarriageReturn,
    Title(String),

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

    List(Vec<Token>),

    // url, title
    Link(String, String),

    AlternativeFont(u8),
    Fraktur,
    UnReversed,
    UnHide,
    Unknown(u8),
}

fn get_token_color(n: u8) -> Token {
    match n {
        39 => Token::ColorDefaultForeground,
        49 => Token::ColorDefaultBackground,
        30..=37 | 90..=97 => Token::ColorForeground(AnsiColor::from_u8(n)),
        40..=47 | 100..=107 => Token::ColorBackground(AnsiColor::from_u8(n)),
        _ => Token::Unknown(0),
    }
}

fn parse_cursor_up(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b["), digit0, tag_no_case("a")))(input)?;
    Ok((rem, Token::CursorUp(str::parse(b).unwrap_or(1))))
}

fn parse_cursor_down(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b["), digit0, tag_no_case("b")))(input)?;
    Ok((rem, Token::CursorDown(str::parse(b).unwrap_or(1))))
}

fn parse_cursor_forward(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b["), digit0, tag_no_case("c")))(input)?;
    Ok((rem, Token::CursorForward(str::parse(b).unwrap_or(1))))
}

fn parse_cursor_back(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b["), digit0, tag_no_case("d")))(input)?;
    Ok((rem, Token::CursorBack(str::parse(b).unwrap_or(1))))
}

fn parse_cursor_next_line(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b["), digit0, tag_no_case("e")))(input)?;
    Ok((rem, Token::CursorNextLine(str::parse(b).unwrap_or(1))))
}

fn parse_cursor_previous_line(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b["), digit0, tag_no_case("f")))(input)?;
    Ok((rem, Token::CursorPreviousLine(str::parse(b).unwrap_or(1))))
}

fn parse_cursor_horizontal(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b["), digit0, tag_no_case("g")))(input)?;
    Ok((
        rem,
        Token::CursorHorizontalAbsolute(str::parse(b).unwrap_or(1)),
    ))
}

fn parse_cursor_position(input: &str) -> IResult<&str, Token> {
    let (rem, (_, x, _, y, _)) =
        tuple((tag("\x1b["), digit0, tag(":"), digit0, tag_no_case("h")))(input)?;
    Ok((
        rem,
        Token::CursorPosition(str::parse(x).unwrap_or(0), str::parse(y).unwrap_or(0)),
    ))
}

fn parse_erase_in_display(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b["), digit0, tag_no_case("j")))(input)?;
    Ok((rem, Token::EraseInDisplay(str::parse(b).unwrap_or(0))))
}

fn parse_erase_in_line(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b["), digit0, tag_no_case("k")))(input)?;
    Ok((rem, Token::EraseInLine(str::parse(b).unwrap_or(0))))
}

fn parse_scroll_up(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b["), digit0, tag_no_case("s")))(input)?;
    Ok((rem, Token::ScrollUp(str::parse(b).unwrap_or(0))))
}
fn parse_scroll_down(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b["), digit0, tag_no_case("t")))(input)?;
    Ok((rem, Token::ScrollDown(str::parse(b).unwrap_or(0))))
}

fn parse_horizontal_vertical_position(input: &str) -> IResult<&str, Token> {
    let (rem, (_, x, _, y, _)) =
        tuple((tag("\x1b["), digit0, tag(":"), digit0, tag_no_case("f")))(input)?;
    Ok((
        rem,
        Token::HorizontalVerticalPosition(str::parse(x).unwrap_or(0), str::parse(y).unwrap_or(0)),
    ))
}

fn parse_aux_port_on(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tag("\x1b5i")(input)?;
    Ok((rem, Token::AUXPortOn))
}
fn parse_aux_port_off(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tag("\x1b[4i")(input)?;
    Ok((rem, Token::AUXPortOff))
}
fn parse_device_status_report(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tag("\x1b[6n")(input)?;
    Ok((rem, Token::DeviceStatusReport))
}

fn parse_cursor_hide(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tuple((tag("\x1b["), opt(tag("?")), digit0, tag_no_case("l")))(input)?;
    Ok((rem, Token::CursorHide))
}
fn parse_cursor_hide_windows(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tag("\x1b[?2004h")(input)?;
    Ok((rem, Token::CursorHide))
}

fn parse_title(input: &str) -> IResult<&str, Token> {
    let (rem, (_, s, _)) = tuple((tag("\x1b]0;"), take_until("\x07"), tag("\x07")))(input)?;
    Ok((rem, Token::Title(s.into())))
}

fn parse_bold(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tag("\x1b(B")(input)?;
    Ok((rem, Token::Bold))
}

fn parse_cursor_show(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tuple((tag("\x1b["), opt(tag("?")), digit0, tag_no_case("h")))(input)?;
    Ok((rem, Token::CursorShow))
}

fn parse_color_foreground(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b[38;5;"), digit0, tag_no_case("m")))(input)?;

    let b = str::parse(b).unwrap_or_default();

    let c = match b {
        0..=7 => b + 30,
        8..=15 => b + 82,
        _ => return Ok((rem, Token::ColorForeground(AnsiColor::Color256(b)))),
    };
    Ok((rem, Token::ColorForeground(AnsiColor::from_u8(c))))
}
fn parse_color_background(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b[48;5;"), digit0, tag_no_case("m")))(input)?;
    let b = str::parse(b).unwrap_or_default();

    let c = match b {
        0..=7 => b + 40,
        8..=15 => b + 92,
        _ => return Ok((rem, Token::ColorBackground(AnsiColor::Color256(b)))),
    };
    Ok((rem, Token::ColorBackground(AnsiColor::from_u8(c))))
}

fn parse_color_underline(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b[58;5;"), digit0, tag_no_case("m")))(input)?;
    Ok((
        rem,
        Token::ColorUnderLine(AnsiColor::from_u8(str::parse(b).unwrap_or(1))),
    ))
}

pub fn get_sgr(n: u8) -> Token {
    match n {
        0 => Token::ColorReset,
        1 => Token::Bold,
        2 => Token::Dim,
        3 => Token::Italic,
        4 => Token::Underline,
        5 => Token::SlowBlink,
        6 => Token::RapidBlink,
        7 => Token::Reverse,
        8 => Token::CursorHide,
        9 => Token::Strike,
        10 => Token::PrimaryFont,
        11..=19 => Token::AlternativeFont(n - 10),
        20 => Token::Fraktur,
        21 => Token::DoublyUnderlined,
        24 => Token::UnUnderlined,
        25 => Token::UnBlink,
        30..=37 | 90..=97 => Token::ColorForeground(AnsiColor::from_u8(n)),
        40..=47 | 100..=107 => Token::ColorBackground(AnsiColor::from_u8(n)),
        39 => Token::ColorDefaultForeground,
        49 => Token::ColorDefaultBackground,
        59 => Token::ColorDefaultUnderline,
        22 => Token::NormalIntensity,
        27 => Token::UnReversed,
        29 => Token::UnStrike,
        28 => Token::UnHide,
        23 => Token::UnItalic,

        _ => Token::Unknown(n),
    }
}

fn parse_sgr1(input: &str) -> IResult<&str, Token> {
    let (rem, (_, b, _)) = tuple((tag("\x1b["), digit0, tag_no_case("m")))(input)?;

    let n: u8 = str::parse(b).unwrap_or_default();

    Ok((rem, get_sgr(n)))
}

fn parse_color_reset(input: &str) -> IResult<&str, Token> {
    let (rem, _) = tuple((tag("\x1b[0"), tag_no_case("m")))(input)?;
    Ok((rem, Token::ColorReset))
}

fn parse_anychar(input: &str) -> IResult<&str, Token> {
    let (rem, c) = anychar(input)?;
    Ok((rem, Token::Char(c)))
}

fn parse_bell(input: &str) -> IResult<&str, Token> {
    let (rem, _) = nom::character::complete::char('\x07')(input)?;
    Ok((rem, Token::Bell))
}

fn parse_backspace(input: &str) -> IResult<&str, Token> {
    let (rem, _) = nom::character::complete::char('\x08')(input)?;
    Ok((rem, Token::Backspace))
}

fn parse_tab(input: &str) -> IResult<&str, Token> {
    let (rem, _) = nom::character::complete::char('\x09')(input)?;
    Ok((rem, Token::Bell))
}

fn parse_line_feed(input: &str) -> IResult<&str, Token> {
    let (rem, _) = nom::character::complete::char('\x0A')(input)?;
    Ok((rem, Token::LineFeed))
}

fn parse_form_feed(input: &str) -> IResult<&str, Token> {
    let (rem, _) = nom::character::complete::char('\x0C')(input)?;
    Ok((rem, Token::FormFeed))
}

fn parse_carriage_return(input: &str) -> IResult<&str, Token> {
    let (rem, _) = nom::character::complete::char('\x0D')(input)?;
    Ok((rem, Token::CarriageReturn))
}

fn parse_sgr2(input: &str) -> IResult<&str, Token> {
    let (rem, (_, front, _, background, _)) =
        tuple((tag("\x1b["), digit0, tag(";"), digit0, tag_no_case("m")))(input)?;

    let a = front.parse().unwrap_or(0);
    let b = background.parse().unwrap_or(0);
    Ok((rem, Token::List(vec![get_sgr(a), get_token_color(b)])))
}

fn parse_sgr3(input: &str) -> IResult<&str, Token> {
    let (rem, (_, ctrl, _, front, _, background, _)) = tuple((
        tag("\x1b["),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag_no_case("m"),
    ))(input)?;

    let a = ctrl.parse().unwrap_or(0);
    let b = front.parse().unwrap_or(0);
    let c = background.parse().unwrap_or(0);

    if a == 38 && b == 5 {
        return Ok((rem, Token::ColorForeground(AnsiColor::from_u8(c))));
    }
    if a == 48 && b == 5 {
        return Ok((rem, Token::ColorBackground(AnsiColor::from_u8(c))));
    }
    Ok((
        rem,
        Token::List(vec![get_sgr(a), get_token_color(b), get_token_color(c)]),
    ))
}

fn parse_sgr4(input: &str) -> IResult<&str, Token> {
    let (rem, (_, reset, _, ctrl, _, front, _, background, _)) = tuple((
        tag("\x1b["),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag(";"),
        digit0,
        tag_no_case("m"),
    ))(input)?;
    let a: u8 = reset.parse().unwrap_or(0);
    let b: u8 = ctrl.parse().unwrap_or(0);
    let c: u8 = front.parse().unwrap_or(0);
    let d: u8 = background.parse().unwrap_or(0);

    let mut v = vec![get_sgr(a)];

    match b {
        48 => match c {
            5 => {
                v.push(Token::ColorBackground(AnsiColor::Color256(d)));
            }
            _ => {
                v.push(Token::ColorBackground(AnsiColor::from_u8(d)));
            }
        },
        38 => match c {
            5 => {
                v.push(Token::ColorForeground(AnsiColor::Color256(d)));
            }
            _ => {
                v.push(Token::ColorForeground(AnsiColor::from_u8(d)));
            }
        },
        5 | 6 => {
            v.push(get_sgr(b));
            v.push(Token::ColorForeground(AnsiColor::from_u8(c)));
            v.push(Token::ColorBackground(AnsiColor::from_u8(d)));
        }
        _ => {
            v.push(get_sgr(b));
            v.push(Token::ColorForeground(AnsiColor::Color256(c)));
            v.push(Token::ColorBackground(AnsiColor::Color256(d)));
        }
    };

    Ok((rem, Token::List(v)))
}

fn parse_sgr5(input: &str) -> IResult<&str, Token> {
    let (rem, (_, ctrl, _, ty, _, r, _, g, _, b, _)) = tuple((
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
    ))(input)?;
    let ctrl = ctrl.parse().unwrap_or(0);
    let ty = ty.parse().unwrap_or(0);
    let r = r.parse().unwrap_or(0);
    let g = g.parse().unwrap_or(0);
    let b = b.parse().unwrap_or(0);
    if ctrl == 38 && ty == 2 {
        return Ok((rem, Token::ColorForeground(AnsiColor::Rgb(r, g, b))));
    }

    if ctrl == 48 && ty == 2 {
        return Ok((rem, Token::ColorBackground(AnsiColor::Rgb(r, g, b))));
    }
    todo!()
}

fn parse_sgr6(input: &str) -> IResult<&str, Token> {
    let (rem, (_, ctrl, _, ty, _, r, _, g, _, b, _, n, _)) = tuple((
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
    ))(input)?;
    let ctrl = ctrl.parse().unwrap_or(0);
    let ty = ty.parse().unwrap_or(0);
    let r = r.parse().unwrap_or(0);
    let g = g.parse().unwrap_or(0);
    let b = b.parse().unwrap_or(0);
    let n = n.parse().unwrap_or(0);
    if ctrl == 38 && ty == 2 {
        return Ok((
            rem,
            Token::ColorFgBg(AnsiColor::Rgb(r, g, b), AnsiColor::from_u8(n)),
        ));
    }

    if ctrl == 48 && ty == 2 {
        return Ok((
            rem,
            Token::ColorFgBg(AnsiColor::from_u8(n), AnsiColor::Rgb(r, g, b)),
        ));
    }

    if ctrl == 48 && ty == 5 && g == 38 && b == 5 {
        return Ok((
            rem,
            Token::ColorFgBg(AnsiColor::Color256(n), AnsiColor::Color256(r)),
        ));
    }
    if ctrl == 38 && ty == 5 && g == 48 && b == 5 {
        return Ok((
            rem,
            Token::ColorFgBg(AnsiColor::Color256(r), AnsiColor::Color256(n)),
        ));
    }
    todo!()
}

fn parse_sgr7(input: &str) -> IResult<&str, Token> {
    let (rem, (_, ctrl, _, a, _, b, _, c, _, d, _, e, _, f, _)) = tuple((
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
    ))(input)?;
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
                v.push(Token::ColorForeground(AnsiColor::Color256(c)));
            }
            2 => {
                v.push(Token::ColorForeground(AnsiColor::Color8(Color8::from_u8(
                    c,
                ))));
            }
            _ => {}
        },
        48 => match b {
            5 => {
                v.push(Token::ColorBackground(AnsiColor::Color256(c)));
            }
            2 => {
                v.push(Token::ColorBackground(AnsiColor::Color8(Color8::from_u8(
                    c,
                ))));
            }
            _ => {}
        },
        _ => {}
    }

    match d {
        38 => match e {
            5 => {
                v.push(Token::ColorForeground(AnsiColor::Color256(f)));
            }
            2 => {
                v.push(Token::ColorForeground(AnsiColor::Color8(Color8::from_u8(
                    f,
                ))));
            }
            _ => {}
        },
        48 => match e {
            5 => {
                v.push(Token::ColorBackground(AnsiColor::Color256(f)));
            }
            2 => {
                v.push(Token::ColorBackground(AnsiColor::Color8(Color8::from_u8(
                    f,
                ))));
            }
            _ => {}
        },
        _ => {}
    }

    Ok((rem, Token::List(v)))
}

fn parse_sgr10(input: &str) -> IResult<&str, Token> {
    let (rem, (_, c1, _, t1, _, r1, _, g1, _, b1, _, c2, _, t2, _, r2, _, g2, _, b2, _)) =
        tuple((
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
        ))(input)?;
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
    Ok((rem, Token::ColorFgBg(fg, bg)))
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
    ))(input)?;

    if n == '\x1a' {
        return Ok((rem, Token::Char('␦')));
    }

    Ok((rem, Token::Unknown(n as u8)))
}

fn parse_link_no_title(input: &str) -> IResult<&str, Token> {
    let (rem, (_, _, url, _)) = tuple((
        tag("\x1b]8;"),
        opt(tag(";")),
        alt((take_until("\x1b]8;;\x1b\\"), take_until("\x1b[!p"))),
        alt((tag("\x1b]8;;\x1b\\"), tag("\x1b[!p"))),
    ))(input)?;
    Ok((rem, Token::Link(url.to_string(), url.to_string())))
}

fn parse_link_with_title(input: &str) -> IResult<&str, Token> {
    let (rem, (_, _, url, _, title, _)) = tuple((
        tag("\x1b]8;"),
        opt(tag(";")),
        take_until("\x1b\\"),
        tag("\x1b\\"),
        alt((take_until("\x1b]8;;\x1b\\"), take_until("\x1b[!p"))),
        alt((tag("\x1b]8;;\x1b\\"), tag("\x1b[!p"))),
    ))(input)?;
    Ok((rem, Token::Link(url.to_string(), title.to_string())))
}

fn parse_link_ll(input: &str) -> IResult<&str, Token> {
    let (rem, (_, url, _, title, _)) = tuple((
        tag("\x1b]8;;"),
        take_until("\x07"),
        tag("\x07"),
        take_until("\x1b]8;;\x07"),
        tag("\x1b]8;;\x07"),
    ))(input)?;
    Ok((rem, Token::Link(url.to_string(), title.to_string())))
}

pub(crate) fn parse_ansi(input: &str) -> IResult<&str, Vec<Token>> {
    many0(alt((
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
        alt((parse_link_with_title, parse_link_no_title, parse_link_ll)),
        parse_unknown,
        parse_anychar,
    )))(input)
}
