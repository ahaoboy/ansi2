pub mod ans;
pub mod canvas;
pub mod color;
pub mod css;
pub mod html;
pub mod image;
pub mod lex;
pub mod node;
pub mod svg;
pub mod text;
pub mod theme;

#[cfg(test)]
mod test {
    use crate::{canvas::Canvas, lex::parse_ansi};
    use insta::assert_debug_snapshot;
    #[test]
    fn test_plain() {
        let s = "ansi";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test() {
        let s = "\x1b[0;5;35;45m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_reset() {
        let s = "\x1b[m\x1b";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_starship_title() {
        let s = "\x1b[?2004h\x1b]0;/c/wt\x1b[30m\x1b(B\x1b[m\x1b[J\x1b[K";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_starship_prompt() {
        let s = "\x1b[38;2;218;98;125m\x1b[48;2;218;98;125;30mwin\x1b[38;2;218;98;125m\x1b[30mC:/wt \x1b[48;2;252;161;125;38;2;218;98;125m\x1b[48;2;134;187;216;38;2;252;161;125m\x1b[48;2;6;150;154;38;2;134;187;216m\x1b[48;2;51;101;138;38;2;6;150;154m\x1b[0m\x1b[K";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_vitest_bench() {
        let s = "\x1b[36m\x1b[7m\x1b[1m BENCH \x1b[22m\x1b[27m\x1b[39m \x1b[36mSummary\x1b[39m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_fastfetch() {
        let s = "\x1b[1G\x1b[19A\x1b[47C";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_color256() {
        let s = "\x1b[38;5;99ma\x1b[48;5;99mb";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_color24() {
        let s = "\x1b[38;2;0;0;114m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_base() {
        let s =
            "\x1b[30mblack\x1b[0m    \x1b[90mbright black\x1b[0m     \x1b[40mblack\x1b[0m    \x1b[100mbright black\x1b[0m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_link() {
        let s =
            "\x1b]8;;file:///Users/xxx/src/new-nu-parser/Cargo.toml\x1b\\Cargo.toml\x1b]8;;\x1b";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_link_hide() {
        let s = "\x1b[8mhttp://example.com/how_about_me\x1b[m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_link_id() {
        let s = "\x1b]8;id=1;http://example.com/id\x1b\\twice\x1b]8;;\x1b\\";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_empty_link() {
        let s = "\x1b]8;;\x1b\\";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_link_soft_reset() {
        let s = "\x1b]8;;http://example.com/softreset\\\x1b[3;31mfoo[!pbar";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_link_no_close() {
        let s = "\x1b]8;;http://example.com/foo\x1b\\foo\x1b]8;;http://example.com/foo\x1b\\foo\x1b]8;;\x1b\\ \x1b]8;;http://example.com/foo\x1b\\foo\x1b]8;;http://example.com/bar\x1b\\bar\x1b]8;;\x1b\\";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_link_ll() {
        let s = "]8;;file://win/c/code/ansi2/targettarget]8;;";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_sgr6() {
        let s = "\x1b[48;5;186;38;5;16m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_style() {
        let s =
            "aaa\x1b[1mbold\x1b[0m \x1b[2mdim\x1b[0m \x1b[3mitalic\x1b[3m \x1b[4munderline\x1b[4m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_dim() {
        let s = "[39m[2;39mNUSHELL";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_sgr4() {
        let s = "[0;48;5;45maaa";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }
    #[test]
    fn test_vitest() {
        let s = "[36m[7m[1m BENCH [22m[27m[39m [36mSummary[39m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_svg2() {
        let s = "[39;40m 39;40m [0m[39;41m 39;41m";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);

        let canvas = Canvas::new(s, None);
        assert_debug_snapshot!(canvas);
        assert_debug_snapshot!(canvas.minify());
    }

    #[test]
    fn test_starship() {
        let s =
            "]7;file://win/c/code/ansi2]0;/c/c/ansi2[30m(B[m[K]133;A;special_key=1[J";
        let r = parse_ansi(s).unwrap();
        assert_debug_snapshot!(r);
    }
}
