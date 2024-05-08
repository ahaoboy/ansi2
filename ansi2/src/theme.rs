#[derive(Debug, Clone, Copy)]
pub struct VsCodeTheme;

pub trait ColorTable {
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

impl ColorTable for VsCodeTheme {
    fn black(&self) -> (u8, u8, u8) {
        (0, 0, 0)
    }

    fn red(&self) -> (u8, u8, u8) {
        (205, 49, 49)
    }

    fn green(&self) -> (u8, u8, u8) {
        (13, 188, 121)
    }

    fn yellow(&self) -> (u8, u8, u8) {
        (229, 229, 16)
    }

    fn blue(&self) -> (u8, u8, u8) {
        (36, 114, 200)
    }

    fn magenta(&self) -> (u8, u8, u8) {
        (188, 63, 188)
    }

    fn cyan(&self) -> (u8, u8, u8) {
        (17, 168, 205)
    }

    fn white(&self) -> (u8, u8, u8) {
        (229, 229, 229)
    }

    fn bright_black(&self) -> (u8, u8, u8) {
        (102, 102, 102)
    }

    fn bright_red(&self) -> (u8, u8, u8) {
        (241, 76, 76)
    }

    fn bright_green(&self) -> (u8, u8, u8) {
        (35, 209, 139)
    }

    fn bright_yellow(&self) -> (u8, u8, u8) {
        (245, 245, 67)
    }

    fn bright_blue(&self) -> (u8, u8, u8) {
        (59, 142, 234)
    }

    fn bright_magenta(&self) -> (u8, u8, u8) {
        (214, 112, 214)
    }

    fn bright_cyan(&self) -> (u8, u8, u8) {
        (41, 184, 219)
    }

    fn bright_white(&self) -> (u8, u8, u8) {
        (229, 229, 229)
    }
}

pub struct UbuntuTheme;

impl ColorTable for UbuntuTheme {
    fn black(&self) -> (u8, u8, u8) {
        (1, 1, 1)
    }

    fn red(&self) -> (u8, u8, u8) {
        (222, 56, 43)
    }

    fn green(&self) -> (u8, u8, u8) {
        (57, 181, 74)
    }

    fn yellow(&self) -> (u8, u8, u8) {
        (255, 199, 6)
    }

    fn blue(&self) -> (u8, u8, u8) {
        (0, 111, 184)
    }

    fn magenta(&self) -> (u8, u8, u8) {
        (118, 38, 113)
    }

    fn cyan(&self) -> (u8, u8, u8) {
        (44, 181, 233)
    }

    fn white(&self) -> (u8, u8, u8) {
        (204, 204, 204)
    }

    fn bright_black(&self) -> (u8, u8, u8) {
        (128, 128, 128)
    }

    fn bright_red(&self) -> (u8, u8, u8) {
        (255, 0, 0)
    }

    fn bright_green(&self) -> (u8, u8, u8) {
        (0, 255, 0)
    }

    fn bright_yellow(&self) -> (u8, u8, u8) {
        (255, 255, 0)
    }

    fn bright_blue(&self) -> (u8, u8, u8) {
        (0, 0, 255)
    }

    fn bright_magenta(&self) -> (u8, u8, u8) {
        (255, 0, 255)
    }

    fn bright_cyan(&self) -> (u8, u8, u8) {
        (0, 255, 255)
    }

    fn bright_white(&self) -> (u8, u8, u8) {
        (255, 255, 255)
    }
}
