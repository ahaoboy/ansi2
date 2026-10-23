pub type Rgb = (u8, u8, u8);

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
// https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Vscode,
    Ubuntu,
    Vga,
    Xterm,
}

const VSCODE: [Rgb; 16] = [
    (0, 0, 0),
    (205, 49, 49),
    (13, 188, 121),
    (229, 229, 16),
    (36, 114, 200),
    (188, 63, 188),
    (17, 168, 205),
    (229, 229, 229),
    (102, 102, 102),
    (241, 76, 76),
    (35, 209, 139),
    (245, 245, 67),
    (59, 142, 234),
    (214, 112, 214),
    (41, 184, 219),
    (229, 229, 229),
];

const UBUNTU: [Rgb; 16] = [
    (1, 1, 1),
    (222, 56, 43),
    (57, 181, 74),
    (255, 199, 6),
    (0, 111, 184),
    (118, 38, 113),
    (44, 181, 233),
    (204, 204, 204),
    (128, 128, 128),
    (255, 0, 0),
    (0, 255, 0),
    (255, 255, 0),
    (0, 0, 255),
    (255, 0, 255),
    (0, 255, 255),
    (255, 255, 255),
];

const VGA: [Rgb; 16] = [
    (0, 0, 0),
    (170, 0, 0),
    (0, 170, 0),
    (170, 85, 0),
    (0, 0, 170),
    (170, 0, 170),
    (0, 170, 170),
    (170, 170, 170),
    (85, 85, 85),
    (255, 85, 85),
    (85, 255, 85),
    (255, 255, 85),
    (85, 85, 255),
    (255, 85, 255),
    (85, 255, 255),
    (255, 255, 255),
];

const XTERM: [Rgb; 16] = [
    (0, 0, 0),
    (128, 0, 0),
    (0, 128, 0),
    (128, 128, 0),
    (0, 0, 128),
    (128, 0, 128),
    (0, 128, 128),
    (192, 192, 192),
    (128, 128, 128),
    (255, 0, 0),
    (0, 255, 0),
    (255, 255, 0),
    (0, 0, 255),
    (255, 0, 255),
    (0, 255, 255),
    (255, 255, 255),
];

impl Theme {
    pub const fn palette(self) -> &'static [Rgb; 16] {
        match self {
            Self::Vscode => &VSCODE,
            Self::Ubuntu => &UBUNTU,
            Self::Vga => &VGA,
            Self::Xterm => &XTERM,
        }
    }

    pub const fn color(self, index: usize) -> Rgb {
        self.palette()[index]
    }
}

/// The first 16 entries of the xterm 256-colour palette.
// https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
const BASE_COLORS: [Rgb; 16] = [
    (0, 0, 0),
    (128, 0, 0),
    (0, 128, 0),
    (128, 128, 0),
    (0, 0, 128),
    (128, 0, 128),
    (0, 128, 128),
    (192, 192, 192),
    (128, 128, 128),
    (255, 0, 0),
    (0, 255, 0),
    (255, 255, 0),
    (0, 0, 255),
    (255, 0, 255),
    (0, 255, 255),
    (255, 255, 255),
];

const CUBE_LEVELS: [u8; 6] = [0, 95, 135, 175, 215, 255];

/// Resolve an ANSI 256-colour index without storing the whole palette.
pub fn color256(index: u8) -> Rgb {
    match index {
        0..=15 => BASE_COLORS[index as usize],
        16..=231 => {
            let n = index - 16;
            (
                CUBE_LEVELS[(n / 36) as usize],
                CUBE_LEVELS[((n / 6) % 6) as usize],
                CUBE_LEVELS[(n % 6) as usize],
            )
        }
        _ => {
            let level = 8 + 10 * (index - 232);
            (level, level, level)
        }
    }
}
