use raqote::{SolidSource, Source};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }

    pub const fn alpha(self) -> u8 {
        self.a
    }

    pub const fn red(self) -> u8 {
        self.r
    }

    pub const fn green(self) -> u8 {
        self.g
    }

    pub const fn blue(self) -> u8 {
        self.b
    }

    pub const fn solid_source(self) -> SolidSource {
        SolidSource {
            a: self.a,
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    pub const fn source(self) -> Source<'static> {
        Source::Solid(self.solid_source())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SolidColor {
    Black,
    White,
    Red,
    Blue,
    Green,
}

impl From<SolidColor> for Color {
    fn from(value: SolidColor) -> Self {
        match value {
            SolidColor::Black => Color::new(0xff, 0, 0, 0),
            SolidColor::White => Color::new(0xff, 0xff, 0xff, 0xff),
            SolidColor::Red => Color::new(0xff, 0xff, 0, 0),
            SolidColor::Green => Color::new(0xff, 0, 0xff, 0),
            SolidColor::Blue => Color::new(0xff, 0, 0, 0xff),
        }
    }
}
