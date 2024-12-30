use std::collections::BinaryHeap;

use crate::{
    canvas::{pixels_to_ans, Canvas},
    color::AnsiColor,
    node::Node,
};
#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    state: Node,
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
    Bg(AnsiColor),

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
    fn to_ans(&self) -> String {
        match self {
            // FIXME: remove magic number
            Step::Reset => "0".to_string(),
            Step::Color(ansi_color) => match ansi_color {
                AnsiColor::Default => "39".to_string(),
                AnsiColor::Color8(color8) => format!("{}", color8.to_u8()),
                AnsiColor::Color256(n) => format!("38;5;{}", n),
                AnsiColor::Rgb(r, g, b) => format!("38;2;{};{};{}", r, g, b),
            },
            Step::Bg(ansi_color) => match ansi_color {
                AnsiColor::Default => "49".to_string(),
                AnsiColor::Color8(color8) => format!("{}", color8.to_u8() + 10),
                AnsiColor::Color256(n) => format!("48;5;{}", n),
                AnsiColor::Rgb(r, g, b) => format!("48;2;{};{};{}", r, g, b),
            },
            Step::Bold => "1".to_string(),
            Step::Blink => "5".to_string(),
            Step::Dim => "2".to_string(),
            Step::Italic => "3".to_string(),
            Step::Underline => "4".to_string(),
            Step::Hide => "8".to_string(),
            Step::Strike => "9".to_string(),
            Step::UnBold => "22".to_string(),
            Step::UnBlink => "25".to_string(),
            Step::UnDim => "22".to_string(),
            Step::UnItalic => "23".to_string(),
            Step::UnUnderline => "24".to_string(),
            Step::UnHide => "28".to_string(),
            Step::UnStrike => "29".to_string(),
            Step::List(vec) => vec.iter().map(|i| i.to_ans()).collect::<Vec<_>>().join(";"),
        }
    }

    fn apply(&self, n: &Node) -> Node {
        let mut new_node = n.clone();

        match self {
            Step::Reset => {
                new_node = Node::default();
            }
            Step::Color(ansi_color) => new_node.color = *ansi_color,
            Step::Bg(ansi_color) => new_node.bg_color = *ansi_color,
            Step::Bold => new_node.bold = true,
            Step::Blink => new_node.blink = true,
            Step::Dim => new_node.dim = true,
            Step::Underline => new_node.underline = true,
            Step::Italic => new_node.italic = true,
            Step::Hide => new_node.hide = true,
            Step::Strike => new_node.strike = true,
            Step::UnBold | Step::UnDim => {
                new_node.dim = false;
                new_node.bold = false
            }
            Step::UnBlink => new_node.blink = false,
            Step::UnUnderline => new_node.underline = false,
            Step::UnItalic => new_node.italic = false,
            Step::UnHide => new_node.hide = false,
            Step::UnStrike => new_node.strike = false,

            Step::List(vec) => {
                for i in vec {
                    new_node = i.apply(&new_node);
                }
            }
        }

        new_node
    }
}

fn bfs(from: &Node, to: &Node) -> Vec<Step> {
    if from.same_style(to) {
        return Vec::new();
    }

    let mut q = BinaryHeap::new();
    q.push(Item {
        state: from.clone(),
        steps: Vec::new(),
        len: 0,
    });
    while let Some(top) = q.pop() {
        if top.state.same_style(to) {
            return top.steps;
        }
        let mut ctrl = vec![Step::Reset];

        if top.state.bold != to.bold {
            ctrl.push(if to.bold { Step::Bold } else { Step::UnBold });
        }

        if top.state.blink != to.blink {
            ctrl.push(if to.blink { Step::Blink } else { Step::UnBlink });
        }
        if top.state.dim != to.dim {
            ctrl.push(if to.dim { Step::Dim } else { Step::UnDim });
        }
        if top.state.hide != to.hide {
            ctrl.push(if to.hide { Step::Hide } else { Step::UnHide });
        }

        if top.state.strike != to.strike {
            ctrl.push(if to.strike {
                Step::Strike
            } else {
                Step::UnStrike
            });
        }

        if top.state.italic != to.italic {
            ctrl.push(if to.italic {
                Step::Italic
            } else {
                Step::UnItalic
            });
        }
        if top.state.underline != to.underline {
            ctrl.push(if to.underline {
                Step::Underline
            } else {
                Step::UnUnderline
            });
        }
        if top.state.hide != to.hide {
            ctrl.push(if to.hide { Step::Hide } else { Step::UnHide });
        }

        let mut sgr1: Vec<Step> = vec![];
        let mut sgr2: Vec<Step> = vec![];
        let mut sgr3: Vec<Step> = vec![];

        match (
            top.state.color == to.color,
            top.state.bg_color == to.bg_color,
        ) {
            (true, true) => {}
            (true, false) => {
                sgr1.push(Step::Bg(to.bg_color));
                for i in ctrl.iter() {
                    sgr2.push(Step::List(vec![i.clone(), Step::Bg(to.bg_color)]));
                }
            }
            (false, true) => {
                sgr1.push(Step::Color(to.color));
                for i in ctrl.iter() {
                    sgr2.push(Step::List(vec![i.clone(), Step::Color(to.color)]));
                }
            }
            (false, false) => {
                sgr1.push(Step::Color(to.color));
                sgr1.push(Step::Bg(to.bg_color));
                for i in ctrl.iter() {
                    sgr3.push(Step::List(vec![
                        i.clone(),
                        Step::Color(to.color),
                        Step::Bg(to.bg_color),
                    ]));
                }
            }
        }

        for i in ctrl
            .into_iter()
            .chain(sgr1.into_iter())
            .chain(sgr2.into_iter())
            .chain(sgr3.into_iter())
        {
            if top.steps.contains(&i) {
                continue;
            }

            let state = i.apply(&top.state);
            let mut steps = top.steps.clone();
            let len = top.len + i.to_ans().len() + 3;
            steps.push(i);
            q.push(Item { state, steps, len });
        }
    }
    Vec::new()
}

pub fn min_distance(from: &Node, to: &Node) -> String {
    if from.same_style(to) {
        return String::new();
    }
    let steps = bfs(from, to);
    steps
        .into_iter()
        .map(|i| {
            let ans = i.to_ans();
            if !ans.is_empty() {
                ["\x1b[", &ans, "m"].concat()
            } else {
                String::new()
            }
        })
        .collect()
}

pub fn to_ans<S: AsRef<str>>(str: S, width: Option<usize>, compress: bool) -> String {
    let s = str.as_ref();
    let canvas = Canvas::new(s, width);

    let pixels = if compress {
        canvas.minify()
    } else {
        canvas.pixels
    };
    pixels_to_ans(pixels)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use insta::assert_debug_snapshot;

    use crate::{
        ans::{bfs, to_ans},
        canvas::Canvas,
        color::AnsiColor,
        node::Node,
    };

    use super::min_distance;

    #[test]
    fn test() {
        let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let assets_dir = Path::new(&cargo_dir).parent().unwrap().join("assets");
        let v = std::fs::read_dir(assets_dir).unwrap();
        for i in v {
            let p = i.unwrap().path().to_string_lossy().to_string();
            if !p.ends_with(".ans") {
                continue;
            }
            if p.ends_with(".min.ans") {
                continue;
            }
            let s = std::fs::read_to_string(&p).unwrap();
            let min = to_ans(&s, None, true);

            let c1 = Canvas::new(&s, None);
            let c2 = Canvas::new(&min, None);
            assert_eq!(c1, c2);

            let min2 = to_ans(&min, None, true);
            assert_eq!(min2, min);
        }
    }

    #[test]
    fn test_min_eq() {
        let mut from = Node::default();
        let mut to = Node::default();
        let s = min_distance(&from, &to);
        assert_eq!(s, "");

        from.bold = true;
        to.bold = true;
        let s = min_distance(&from, &to);
        assert_eq!(s, "");

        from.color = AnsiColor::from_u8(31);
        to.color = AnsiColor::from_u8(31);
        let s = min_distance(&from, &to);
        assert_eq!(s, "");

        from.bg_color = AnsiColor::from_u8(31);
        to.bg_color = AnsiColor::from_u8(31);
        let s = min_distance(&from, &to);
        assert_eq!(s, "");
    }

    #[test]
    fn test_min_italic() {
        let mut from = Node::default();
        let mut to = Node::default();
        from.bold = true;
        to.italic = true;
        let s = bfs(&from, &to);
        assert_debug_snapshot!(s);
    }
    #[test]
    fn test_min_underline() {
        let mut from = Node::default();
        let to = Node::default();
        from.underline = true;
        let s = bfs(&from, &to);
        assert_debug_snapshot!(s);
    }
    #[test]
    fn test_min_color() {
        let from = Node::default();
        let to = Node {
            color: AnsiColor::from_u8(31),
            bg_color: AnsiColor::from_u8(33),
            ..Default::default()
        };
        let s = bfs(&from, &to);
        assert_debug_snapshot!(s);
    }

    #[test]
    fn test_min_bold_dim() {
        let mut from = Node::default();
        let mut to = Node::default();
        from.bold = true;
        to.dim = true;
        let s = bfs(&from, &to);
        assert_debug_snapshot!(s);
    }
}
