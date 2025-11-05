#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }

    pub const fn from_hex(hex: u32) -> Color {
        // we assume no transparency
        let r = hex / 65536;
        let hex = hex - r*65536;
        let g = hex / 256;
        let hex = hex - g*256;
        let b = hex;
        let a = 255;

        Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a
        }
    }

    pub const fn to_hex(&self) -> u32 {
        self.r as u32 * 65536 + self.g as u32 * 256 + self.b as u32
    }

    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);
}
