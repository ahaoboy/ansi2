use clap::ValueEnum;

// https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Theme {
    Vscode,
    Ubuntu,
    Vga,
}

impl<'a> From<&'a str> for Theme {
    fn from(value: &'a str) -> Self {
        match value {
            "vscode" => Theme::Vscode,
            "ubuntu" => Theme::Ubuntu,
            "vga" => Theme::Vga,
            _ => Theme::Vscode,
        }
    }
}
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

const COLORS: [[(u8, u8, u8); 16]; 3] = [VSCODE_COLOR, UBUNTU_COLOR, VGA_COLOR];

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
