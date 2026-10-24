use std::collections::BinaryHeap;

use crate::{
    canvas::Canvas,
    cell::{Cell, Style},
    color::AnsiColor,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    state: Style,
    steps: Vec<Step>,
    len: usize,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.len.cmp(&self.len)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Step {
    Reset,
    Color(AnsiColor),
    Background(AnsiColor),

    Bold,
    Blink,
    Dim,
    Italic,
    Underline,
    Hide,
    Strike,

    UnBold,
    UnBlink,
    UnDim,
    UnItalic,
    UnUnderline,
    UnHide,
    UnStrike,
    List(Vec<Step>),
}

impl Step {
    fn to_ansi(&self) -> String {
        match self {
            Self::Reset => "0".into(),
            Self::Color(color) => match color {
                AnsiColor::Default => "39".into(),
                AnsiColor::Color8(color) => color.to_u8().to_string(),
                AnsiColor::Color256(index) => format!("38;5;{index}"),
                AnsiColor::Rgb(r, g, b) => format!("38;2;{r};{g};{b}"),
            },
            Self::Background(color) => match color {
                AnsiColor::Default => "49".into(),
                AnsiColor::Color8(color) => (color.to_u8() + 10).to_string(),
                AnsiColor::Color256(index) => format!("48;5;{index}"),
                AnsiColor::Rgb(r, g, b) => format!("48;2;{r};{g};{b}"),
            },
            Self::Bold => "1".into(),
            Self::Blink => "5".into(),
            Self::Dim => "2".into(),
            Self::Italic => "3".into(),
            Self::Underline => "4".into(),
            Self::Hide => "8".into(),
            Self::Strike => "9".into(),
            Self::UnBold | Self::UnDim => "22".into(),
            Self::UnBlink => "25".into(),
            Self::UnItalic => "23".into(),
            Self::UnUnderline => "24".into(),
            Self::UnHide => "28".into(),
            Self::UnStrike => "29".into(),
            Self::List(steps) => steps
                .iter()
                .map(Self::to_ansi)
                .collect::<Vec<_>>()
                .join(";"),
        }
    }

    fn apply(&self, style: &Style) -> Style {
        let mut style = *style;
        match self {
            Self::Reset => style = Style::default(),
            Self::Color(color) => style.fg = *color,
            Self::Background(color) => style.bg = *color,
            Self::Bold => style.bold = true,
            Self::Blink => style.blink = true,
            Self::Dim => style.dim = true,
            Self::Underline => style.underline = true,
            Self::Italic => style.italic = true,
            Self::Hide => style.hide = true,
            Self::Strike => style.strike = true,
            Self::UnBold | Self::UnDim => {
                style.bold = false;
                style.dim = false;
            }
            Self::UnBlink => style.blink = false,
            Self::UnUnderline => style.underline = false,
            Self::UnItalic => style.italic = false,
            Self::UnHide => style.hide = false,
            Self::UnStrike => style.strike = false,
            Self::List(steps) => {
                for step in steps {
                    style = step.apply(&style);
                }
            }
        }
        style
    }
}

fn bfs(from: &Style, to: &Style) -> Vec<Step> {
    if from == to {
        return Vec::new();
    }

    let mut queue = BinaryHeap::new();
    queue.push(Item {
        state: *from,
        steps: Vec::new(),
        len: 0,
    });

    while let Some(top) = queue.pop() {
        if top.state == *to {
            return top.steps;
        }

        let mut controls = vec![Step::Reset];
        push_toggle(
            &mut controls,
            top.state.bold,
            to.bold,
            Step::Bold,
            Step::UnBold,
        );
        push_toggle(
            &mut controls,
            top.state.blink,
            to.blink,
            Step::Blink,
            Step::UnBlink,
        );
        push_toggle(&mut controls, top.state.dim, to.dim, Step::Dim, Step::UnDim);
        push_toggle(
            &mut controls,
            top.state.hide,
            to.hide,
            Step::Hide,
            Step::UnHide,
        );
        push_toggle(
            &mut controls,
            top.state.strike,
            to.strike,
            Step::Strike,
            Step::UnStrike,
        );
        push_toggle(
            &mut controls,
            top.state.italic,
            to.italic,
            Step::Italic,
            Step::UnItalic,
        );
        push_toggle(
            &mut controls,
            top.state.underline,
            to.underline,
            Step::Underline,
            Step::UnUnderline,
        );

        let mut color_only = Vec::new();
        let mut reset_with_bg = Vec::new();
        let mut reset_with_fg = Vec::new();
        let mut reset_with_both = Vec::new();

        match (top.state.fg == to.fg, top.state.bg == to.bg) {
            (true, true) => {}
            (true, false) => {
                color_only.push(Step::Background(to.bg));
                for control in &controls {
                    reset_with_bg.push(Step::List(vec![control.clone(), Step::Background(to.bg)]));
                }
            }
            (false, true) => {
                color_only.push(Step::Color(to.fg));
                for control in &controls {
                    reset_with_fg.push(Step::List(vec![control.clone(), Step::Color(to.fg)]));
                }
            }
            (false, false) => {
                color_only.push(Step::Color(to.fg));
                color_only.push(Step::Background(to.bg));
                for control in &controls {
                    reset_with_both.push(Step::List(vec![
                        control.clone(),
                        Step::Color(to.fg),
                        Step::Background(to.bg),
                    ]));
                }
            }
        }

        for step in controls
            .into_iter()
            .chain(color_only)
            .chain(reset_with_bg)
            .chain(reset_with_fg)
            .chain(reset_with_both)
        {
            if top.steps.contains(&step) {
                continue;
            }

            let mut steps = top.steps.clone();
            let len = top.len + step.to_ansi().len() + 3;
            let state = step.apply(&top.state);
            steps.push(step);
            queue.push(Item { state, steps, len });
        }
    }

    Vec::new()
}

fn push_toggle(controls: &mut Vec<Step>, from: bool, to: bool, on: Step, off: Step) {
    if from != to {
        controls.push(if to { on } else { off });
    }
}

pub fn min_distance(from: &Style, to: &Style) -> String {
    if from == to {
        return String::new();
    }

    bfs(from, to)
        .into_iter()
        .map(|step| {
            let ansi = step.to_ansi();
            if ansi.is_empty() {
                String::new()
            } else {
                format!("\x1b[{ansi}m")
            }
        })
        .collect()
}

pub fn cells_to_ans(pixels: &[Vec<Cell>]) -> String {
    let mut output = String::new();
    let mut last = Style::default();

    for (index, row) in pixels.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }
        for cell in row {
            output.push_str(&min_distance(&last, &cell.style));
            output.push_str(&cell.text);
            last = cell.style;
        }

        // Do not let a background/foreground bleed into the next line. The
        // raw input may reset after the last cell (neofetch does), but a
        // style-only reset has no cell of its own and would otherwise be lost.
        if last != Style::default() {
            output.push_str("\x1b[0m");
            last = Style::default();
        }
    }

    output
}

pub fn to_ans(input: &str, width: Option<usize>) -> String {
    let canvas = Canvas::new(input, width);
    cells_to_ans(&canvas.minify())
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use insta::assert_debug_snapshot;

    use super::{bfs, min_distance, to_ans};
    use crate::{Canvas, cell::Style, color::AnsiColor};

    #[test]
    fn test() {
        let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let assets_dir = Path::new(&cargo_dir).parent().unwrap().join("assets");

        for entry in std::fs::read_dir(assets_dir).unwrap() {
            let path = entry.unwrap().path().to_string_lossy().to_string();
            if !path.ends_with(".ans") || path.ends_with(".min.ans") {
                continue;
            }
            // FIXME: skip on windows
            if cfg!(windows) && path.contains("hyperlink-demo") {
                continue;
            }

            let input = std::fs::read_to_string(&path).unwrap();
            let minified = to_ans(&input, None);

            let original = Canvas::new(&input, None);
            let roundtrip = Canvas::new(&minified, None);
            assert_eq!(original, roundtrip, "roundtrip changed {path}");

            assert_eq!(to_ans(&minified, None), minified, "not idempotent {path}");
        }
    }

    #[test]
    fn test_min_eq() {
        let mut from = Style::default();
        let mut to = Style::default();
        assert_eq!(min_distance(&from, &to), "");

        from.bold = true;
        to.bold = true;
        assert_eq!(min_distance(&from, &to), "");

        from.fg = AnsiColor::from_u8(31);
        to.fg = AnsiColor::from_u8(31);
        assert_eq!(min_distance(&from, &to), "");

        from.bg = AnsiColor::from_u8(31);
        to.bg = AnsiColor::from_u8(31);
        assert_eq!(min_distance(&from, &to), "");
    }

    #[test]
    fn test_min_italic() {
        let mut from = Style::default();
        let mut to = Style::default();
        from.bold = true;
        to.italic = true;
        assert_debug_snapshot!(bfs(&from, &to));
    }

    #[test]
    fn test_min_underline() {
        let from = Style {
            underline: true,
            ..Style::default()
        };
        let to = Style::default();
        assert_debug_snapshot!(bfs(&from, &to));
    }

    #[test]
    fn test_min_color() {
        let from = Style::default();
        let to = Style {
            fg: AnsiColor::from_u8(31),
            bg: AnsiColor::from_u8(33),
            ..Style::default()
        };
        assert_debug_snapshot!(bfs(&from, &to));
    }

    #[test]
    fn test_min_bold_dim() {
        let from = Style {
            bold: true,
            ..Style::default()
        };
        let to = Style {
            dim: true,
            ..Style::default()
        };
        assert_debug_snapshot!(bfs(&from, &to));
    }

    #[test]
    fn resets_style_before_newline() {
        let output = to_ans("\x1b[107mA\x1b[0m\n\nB", None);
        assert_eq!(output, "\x1b[107mA\x1b[0m\n\nB");
    }
}
