pub mod canvas;
pub mod cell;
pub mod color;
pub mod css;
#[cfg(feature = "image")]
pub mod image;
pub mod lexer;
pub mod options;
pub mod render;
#[cfg(feature = "sequin")]
pub mod sequin;
pub mod span;
pub mod theme;

pub use canvas::Canvas;
pub use options::{LengthAdjust, Mode, Options};
pub use render::{ans::to_ans, html::to_html, svg::to_svg, text::to_text};
pub use theme::Theme;

#[cfg(feature = "sequin")]
pub use sequin::to_sequin;

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Canvas, lexer::parse};

    const CASES: &[(&str, &str)] = &[
        ("plain", "ansi"),
        ("sgr", "\x1b[0;5;35;45m"),
        ("reset", "\x1b[m\x1b"),
        (
            "starship_title",
            "\x1b[?2004h\x1b]0;/c/wt\x07\x1b[30m\x1b(B\x1b[m\x1b[J\x1b[K",
        ),
        (
            "starship_prompt",
            "\x1b[38;2;218;98;125m?\x1b[48;2;218;98;125;30mwin\x1b[38;2;218;98;125m?\x1b[30mC:/wt \x1b[48;2;252;161;125;38;2;218;98;125m?\x1b[48;2;134;187;216;38;2;252;161;125m?\x1b[48;2;6;150;154;38;2;134;187;216m?\x1b[48;2;51;101;138;38;2;6;150;154m?\x1b[0m\x1b[K",
        ),
        (
            "vitest_bench",
            "\x1b[36m\x1b[7m\x1b[1m BENCH \x1b[22m\x1b[27m\x1b[39m \x1b[36mSummary\x1b[39m",
        ),
        ("fastfetch", "\x1b[1G\x1b[19A\x1b[47C"),
        ("color256", "\x1b[38;5;99ma\x1b[48;5;99mb"),
        ("color24", "\x1b[38;2;0;0;114m"),
        (
            "base",
            "\x1b[30mblack\x1b[0m    \x1b[90mbright black\x1b[0m     \x1b[40mblack\x1b[0m    \x1b[100mbright black\x1b[0m",
        ),
        (
            "link",
            "\x1b]8;;file:///Users/xxx/src/new-nu-parser/Cargo.toml\x1b\\Cargo.toml\x1b]8;;\x1b",
        ),
        ("link_hide", "\x1b[8mhttp://example.com/how_about_me\x1b[m"),
        (
            "link_id",
            "\x1b]8;id=1;http://example.com/id\x1b\\twice\x1b]8;;\x1b\\",
        ),
        ("empty_link", "\x1b]8;;\x1b\\"),
        (
            "link_soft_reset",
            "\x1b]8;;http://example.com/softreset\x1b\\\x1b[3;31mfoo\x1b[!pbar",
        ),
        (
            "link_no_close",
            "\x1b]8;;http://example.com/foo\x1b\\foo\x1b]8;;http://example.com/foo\x1b\\foo\x1b]8;;\x1b\\ \x1b]8;;http://example.com/foo\x1b\\foo\x1b]8;;http://example.com/bar\x1b\\bar\x1b]8;;\x1b\\",
        ),
        (
            "link_ll",
            "\x1b]8;;file://win/c/code/ansi2/target\x07target\x1b]8;;\x07",
        ),
        ("sgr6", "\x1b[48;5;186;38;5;16m"),
        (
            "style",
            "aaa\x1b[1mbold\x1b[0m \x1b[2mdim\x1b[0m \x1b[3mitalic\x1b[3m \x1b[4munderline\x1b[4m",
        ),
        ("dim", "\x1b[39m\x1b[2;39mNUSHELL\x1b"),
        ("sgr4", "\x1b[0;48;5;45maaa"),
        (
            "vitest",
            "\x1b[36m\x1b[7m\x1b[1m BENCH \x1b[22m\x1b[27m\x1b[39m \x1b[36mSummary\x1b[39m",
        ),
        ("svg2", "\x1b[39;40m 39;40m \x1b[0m\x1b[39;41m 39;41m"),
    ];

    #[test]
    fn snapshots() {
        for (name, input) in CASES {
            assert_debug_snapshot!(format!("{name}-tokens"), parse(input));

            let canvas = Canvas::new(input, None);
            assert_debug_snapshot!(format!("{name}-canvas"), canvas);
            assert_debug_snapshot!(format!("{name}-min"), canvas.minify());
        }
    }

    #[test]
    fn starship_metadata() {
        let input = "\x1b]7;file://win/c/code/ansi2\x07\x1b]0;/c/c/ansi2\x07\x1b[30m\x1b(B\x1b[m\x1b[K\x1b]133;A;special_key=1\x07\x1b[J";
        assert_debug_snapshot!(parse(input));
    }

    #[test]
    fn canvas_width_is_content_width() {
        let canvas = Canvas::new("ansi", None);
        assert_eq!(canvas.w, 4);
        assert_eq!(canvas.h, 1);
        assert_eq!(canvas.pixels[0].len(), 4);
    }

    #[test]
    fn canvas_wraps_at_width() {
        let canvas = Canvas::new("abcdef", Some(3));
        assert_eq!(canvas.w, 3);
        assert_eq!(canvas.h, 2);
        assert_eq!(canvas.pixels[0][0].text, "a");
        assert_eq!(canvas.pixels[1][0].text, "d");
    }

    #[test]
    fn canvas_zero_width_means_unlimited() {
        let canvas = Canvas::new("abcdef", Some(0));
        assert_eq!(canvas.w, 6);
        assert_eq!(canvas.h, 1);
    }

    #[test]
    fn style_sequences_do_not_change_dimensions() {
        assert_eq!(Canvas::new("abc\x1b[0m", None).w, 3);
        assert_eq!(Canvas::new("\x1b[0m", None).w, 0);
        assert_eq!(Canvas::new("\x1b[0m", None).h, 0);
    }

    #[test]
    fn cursor_movements_define_a_canvas() {
        let canvas = Canvas::new("\x1b[1G\x1b[19A\x1b[47C", None);
        assert_eq!((canvas.w, canvas.h), (48, 1));
    }

    #[test]
    fn cursor_position_uses_row_and_column() {
        let canvas = Canvas::new("\x1b[2;3Hx", None);
        assert_eq!(canvas.pixels[1][2].text, "x");
    }

    #[test]
    fn erase_line_clears_to_the_right() {
        let canvas = Canvas::new("abcd\x1b[1;3H\x1b[K", None);
        assert_eq!(canvas.pixels[0][0].text, "a");
        assert_eq!(canvas.pixels[0][1].text, "b");
        assert_eq!(canvas.pixels[0][2].text, " ");
        assert_eq!(canvas.pixels[0][3].text, " ");
    }
}
