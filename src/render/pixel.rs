use super::color::{Color, SolidColor};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Pixel {
    color: Color,
}

impl Pixel {
    pub const fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn white() -> Self {
        Self {
            color: SolidColor::White.into(),
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

pub fn pixels_from_schema(schema: &[&[u8]], pixel: Pixel) -> Vec<Vec<Option<Pixel>>> {
    let mut pixels = Vec::with_capacity(schema.len());

    for line in schema {
        let mut pixel_line = Vec::with_capacity(line.len());
        for p in *line {
            match *p {
                1 => pixel_line.push(Some(pixel)),
                _ => pixel_line.push(None),
            }
        }
        pixels.push(pixel_line);
    }

    pixels
}
