use super::{Color, Pixel};

pub const DEFAULT_PIXEL: Pixel = Pixel::new(Color::new(0xff, 0xff, 0xff, 0xff));

#[derive(Debug, Clone)]
pub struct Sprite {
    height: usize,
    width: usize,
    flip: bool,
    pixels: Vec<Vec<Option<Pixel>>>,
}

impl Sprite {
    pub fn from_asset(schema: &[&[u8]]) -> Self {
        let mut pixels = Vec::with_capacity(schema.len());

        for line in schema {
            let mut pixel_line = Vec::with_capacity(line.len());
            for p in *line {
                match *p {
                    1 => pixel_line.push(Some(DEFAULT_PIXEL)),
                    _ => pixel_line.push(None),
                }
            }
            pixels.push(pixel_line);
        }

        Self {
            height: schema.len(),
            width: schema[0].len(),
            flip: false,
            pixels,
        }
    }

    pub fn pixels(&self) -> &Vec<Vec<Option<Pixel>>> {
        &self.pixels
    }
}
