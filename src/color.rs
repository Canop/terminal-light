use {
    crate::ansi_to_rgb,
};

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Ansi(AnsiColor),
    Rgb(Rgb),
}

impl Color {
    pub fn rgb(self) -> Rgb {
        match self {
            Self::Ansi(ansi) => ansi.to_rgb(),
            Self::Rgb(rgb) => rgb,
        }
    }
    pub fn luma(self) -> f32 {
        self.rgb().luma()
    }
}

/// Ansi Color Code
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnsiColor {
    pub code: u8,
}

impl AnsiColor {
    pub fn to_rgb(self) -> Rgb {
        ansi_to_rgb::ANSI_TO_RGB[self.code as usize]
    }
}

/// RGB color, with u16 components
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb {
    /// red
    pub r: u16,
    /// green
    pub g: u16,
    /// blue
    pub b: u16,
}

impl Rgb {
    /// Create a new RGB color from its components
    pub fn new(r: u16, g: u16, b: u16) -> Self {
        Self { r, g, b }
    }
    /// red part in `[0,1]`
    pub fn rp(self) -> f32 {
        self.r as f32 / 65535f32
    }
    /// green part in `[0,1]`
    pub fn gp(self) -> f32 {
        self.g as f32 / 65535f32
    }
    /// blue part in `[0,1]`
    pub fn bp(self) -> f32 {
        self.b as f32 / 65535f32
    }
    /// Compute the Luma value characterizing the "light" of the color,
    /// going from 0 (black) to 1 (white).
    ///
    /// Reference: <https://en.wikipedia.org/wiki/Luma_(video)>
    pub fn luma(self) -> f32 {
        0.2627 * self.rp() + 0.6780 * self.gp() + 0.0593 * self.bp()
    }
}

