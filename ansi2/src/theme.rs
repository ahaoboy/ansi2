#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
// https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Vscode,
    Ubuntu,
    Vga,
    Xterm,
}

impl<'a> From<&'a str> for Theme {
    fn from(value: &'a str) -> Self {
        match value {
            "vscode" => Theme::Vscode,
            "ubuntu" => Theme::Ubuntu,
            "vga" => Theme::Vga,
            "xterm" => Theme::Xterm,
            _ => Theme::Vscode,
        }
    }
}

// https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
pub const COLOR256: [(u8, u8, u8); 256] = [
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
    (0, 0, 0),
    (0, 0, 95),
    (0, 0, 135),
    (0, 0, 175),
    (0, 0, 215),
    (0, 0, 255),
    (0, 95, 0),
    (0, 95, 95),
    (0, 95, 135),
    (0, 95, 175),
    (0, 95, 215),
    (0, 95, 255),
    (0, 135, 0),
    (0, 135, 95),
    (0, 135, 135),
    (0, 135, 175),
    (0, 135, 215),
    (0, 135, 255),
    (0, 175, 0),
    (0, 175, 95),
    (0, 175, 135),
    (0, 175, 175),
    (0, 175, 215),
    (0, 175, 255),
    (0, 215, 0),
    (0, 215, 95),
    (0, 215, 135),
    (0, 215, 175),
    (0, 215, 215),
    (0, 215, 255),
    (0, 255, 0),
    (0, 255, 95),
    (0, 255, 135),
    (0, 255, 175),
    (0, 255, 215),
    (0, 255, 255),
    (95, 0, 0),
    (95, 0, 95),
    (95, 0, 135),
    (95, 0, 175),
    (95, 0, 215),
    (95, 0, 255),
    (95, 95, 0),
    (95, 95, 95),
    (95, 95, 135),
    (95, 95, 175),
    (95, 95, 215),
    (95, 95, 255),
    (95, 135, 0),
    (95, 135, 95),
    (95, 135, 135),
    (95, 135, 175),
    (95, 135, 215),
    (95, 135, 255),
    (95, 175, 0),
    (95, 175, 95),
    (95, 175, 135),
    (95, 175, 175),
    (95, 175, 215),
    (95, 175, 255),
    (95, 215, 0),
    (95, 215, 95),
    (95, 215, 135),
    (95, 215, 175),
    (95, 215, 215),
    (95, 215, 255),
    (95, 255, 0),
    (95, 255, 95),
    (95, 255, 135),
    (95, 255, 175),
    (95, 255, 215),
    (95, 255, 255),
    (135, 0, 0),
    (135, 0, 95),
    (135, 0, 135),
    (135, 0, 175),
    (135, 0, 215),
    (135, 0, 255),
    (135, 95, 0),
    (135, 95, 95),
    (135, 95, 135),
    (135, 95, 175),
    (135, 95, 215),
    (135, 95, 255),
    (135, 135, 0),
    (135, 135, 95),
    (135, 135, 135),
    (135, 135, 175),
    (135, 135, 215),
    (135, 135, 255),
    (135, 175, 0),
    (135, 175, 95),
    (135, 175, 135),
    (135, 175, 175),
    (135, 175, 215),
    (135, 175, 255),
    (135, 215, 0),
    (135, 215, 95),
    (135, 215, 135),
    (135, 215, 175),
    (135, 215, 215),
    (135, 215, 255),
    (135, 255, 0),
    (135, 255, 95),
    (135, 255, 135),
    (135, 255, 175),
    (135, 255, 215),
    (135, 255, 255),
    (175, 0, 0),
    (175, 0, 95),
    (175, 0, 135),
    (175, 0, 175),
    (175, 0, 215),
    (175, 0, 255),
    (175, 95, 0),
    (175, 95, 95),
    (175, 95, 135),
    (175, 95, 175),
    (175, 95, 215),
    (175, 95, 255),
    (175, 135, 0),
    (175, 135, 95),
    (175, 135, 135),
    (175, 135, 175),
    (175, 135, 215),
    (175, 135, 255),
    (175, 175, 0),
    (175, 175, 95),
    (175, 175, 135),
    (175, 175, 175),
    (175, 175, 215),
    (175, 175, 255),
    (175, 215, 0),
    (175, 215, 95),
    (175, 215, 135),
    (175, 215, 175),
    (175, 215, 215),
    (175, 215, 255),
    (175, 255, 0),
    (175, 255, 95),
    (175, 255, 135),
    (175, 255, 175),
    (175, 255, 215),
    (175, 255, 255),
    (215, 0, 0),
    (215, 0, 95),
    (215, 0, 135),
    (215, 0, 175),
    (215, 0, 215),
    (215, 0, 255),
    (215, 95, 0),
    (215, 95, 95),
    (215, 95, 135),
    (215, 95, 175),
    (215, 95, 215),
    (215, 95, 255),
    (215, 135, 0),
    (215, 135, 95),
    (215, 135, 135),
    (215, 135, 175),
    (215, 135, 215),
    (215, 135, 255),
    (215, 175, 0),
    (215, 175, 95),
    (215, 175, 135),
    (215, 175, 175),
    (215, 175, 215),
    (215, 175, 255),
    (215, 215, 0),
    (215, 215, 95),
    (215, 215, 135),
    (215, 215, 175),
    (215, 215, 215),
    (215, 215, 255),
    (215, 255, 0),
    (215, 255, 95),
    (215, 255, 135),
    (215, 255, 175),
    (215, 255, 215),
    (215, 255, 255),
    (255, 0, 0),
    (255, 0, 95),
    (255, 0, 135),
    (255, 0, 175),
    (255, 0, 215),
    (255, 0, 255),
    (255, 95, 0),
    (255, 95, 95),
    (255, 95, 135),
    (255, 95, 175),
    (255, 95, 215),
    (255, 95, 255),
    (255, 135, 0),
    (255, 135, 95),
    (255, 135, 135),
    (255, 135, 175),
    (255, 135, 215),
    (255, 135, 255),
    (255, 175, 0),
    (255, 175, 95),
    (255, 175, 135),
    (255, 175, 175),
    (255, 175, 215),
    (255, 175, 255),
    (255, 215, 0),
    (255, 215, 95),
    (255, 215, 135),
    (255, 215, 175),
    (255, 215, 215),
    (255, 215, 255),
    (255, 255, 0),
    (255, 255, 95),
    (255, 255, 135),
    (255, 255, 175),
    (255, 255, 215),
    (255, 255, 255),
    (8, 8, 8),
    (18, 18, 18),
    (28, 28, 28),
    (38, 38, 38),
    (48, 48, 48),
    (58, 58, 58),
    (68, 68, 68),
    (78, 78, 78),
    (88, 88, 88),
    (98, 98, 98),
    (108, 108, 108),
    (118, 118, 118),
    (128, 128, 128),
    (138, 138, 138),
    (148, 148, 148),
    (158, 158, 158),
    (168, 168, 168),
    (178, 178, 178),
    (188, 188, 188),
    (198, 198, 198),
    (208, 208, 208),
    (218, 218, 218),
    (228, 228, 228),
    (238, 238, 238),
];

const VSCODE_COLOR: [(u8, u8, u8); 16] = [
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

const UBUNTU_COLOR: [(u8, u8, u8); 16] = [
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

const VGA_COLOR: [(u8, u8, u8); 16] = [
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

const XTERM_COLOR: [(u8, u8, u8); 16] = [
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

const COLORS: [[(u8, u8, u8); 16]; 4] = [VSCODE_COLOR, UBUNTU_COLOR, VGA_COLOR, XTERM_COLOR];

impl Theme {
    fn discriminant(&self) -> usize {
        unsafe { (*(self as *const _ as *const u8)) as usize }
    }
}

pub trait ColorTable: Copy + Sized {
    fn black(&self) -> (u8, u8, u8);
    fn red(&self) -> (u8, u8, u8);
    fn green(&self) -> (u8, u8, u8);
    fn yellow(&self) -> (u8, u8, u8);
    fn blue(&self) -> (u8, u8, u8);
    fn magenta(&self) -> (u8, u8, u8);
    fn cyan(&self) -> (u8, u8, u8);
    fn white(&self) -> (u8, u8, u8);

    fn bright_black(&self) -> (u8, u8, u8);
    fn bright_red(&self) -> (u8, u8, u8);
    fn bright_green(&self) -> (u8, u8, u8);
    fn bright_yellow(&self) -> (u8, u8, u8);
    fn bright_blue(&self) -> (u8, u8, u8);
    fn bright_magenta(&self) -> (u8, u8, u8);
    fn bright_cyan(&self) -> (u8, u8, u8);
    fn bright_white(&self) -> (u8, u8, u8);
}

impl ColorTable for Theme {
    fn black(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][0]
    }

    fn red(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][1]
    }

    fn green(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][2]
    }

    fn yellow(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][3]
    }

    fn blue(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][4]
    }

    fn magenta(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][5]
    }

    fn cyan(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][6]
    }

    fn white(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][7]
    }

    fn bright_black(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][8]
    }

    fn bright_red(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][9]
    }

    fn bright_green(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][10]
    }

    fn bright_yellow(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][11]
    }

    fn bright_blue(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][12]
    }

    fn bright_magenta(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][13]
    }

    fn bright_cyan(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][14]
    }

    fn bright_white(&self) -> (u8, u8, u8) {
        COLORS[self.discriminant()][15]
    }
}
