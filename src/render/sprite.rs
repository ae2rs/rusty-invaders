use super::{Color, Pixel};

pub const DEFAULT_PIXEL: Pixel = Pixel::new(Color::new(0xff, 0xff, 0xff, 0xff));
pub const PIXEL_SIZE: usize = 6;

#[derive(Debug, Clone)]
pub struct Sprite {
    height: usize,
    width: usize,
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
            pixels,
        }
    }

    pub fn render(&self, pos: (usize, usize), buffer: &mut raqote::DrawTarget) {
        for (row, pixels) in self.pixels.iter().enumerate() {
            for (col, pixel) in pixels.iter().enumerate().filter(|p| p.1.is_some()) {
                let pixel = pixel.unwrap();
                let mut pb = raqote::PathBuilder::new();
                pb.rect(
                    ((col + pos.0) * PIXEL_SIZE) as f32,
                    ((row + pos.1) * PIXEL_SIZE) as f32,
                    PIXEL_SIZE as f32,
                    PIXEL_SIZE as f32,
                );
                let path = pb.finish();
                let pixel_color = pixel.color();
                buffer.fill(&path, &pixel_color.source(), &raqote::DrawOptions::new());
            }
        }
    }
}
