use super::{Color, Pixel};

pub const DEFAULT_PIXEL: Pixel = Pixel::new(Color::new(0xff, 0xff, 0xff, 0xff));
pub const PIXEL_SIZE: usize = 6;

pub struct Sprite {
    height: usize,
    width: usize,
    surface: raqote::DrawTarget,
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
            surface: Self::bake(&pixels),
            pixels,
        }
    }

    fn bake(pixels: &Vec<Vec<Option<Pixel>>>) -> raqote::DrawTarget {
        let rows = pixels.len() as i32;
        let cols = pixels.iter().map(Vec::len).max().unwrap_or(0) as i32;
        let pixel_size = PIXEL_SIZE as i32;

        let mut surface = raqote::DrawTarget::new(cols * pixel_size, rows * pixel_size);

        let options = raqote::DrawOptions {
            blend_mode: raqote::BlendMode::SrcOver,
            alpha: 1.0,
            antialias: raqote::AntialiasMode::None,
        };

        for (row, pixels) in pixels.iter().enumerate() {
            for (col, pixel) in pixels.iter().enumerate() {
                let Some(pixel) = pixel else {
                    continue;
                };

                let source = pixel.color().source();

                surface.fill_rect(
                    (col as i32 * pixel_size) as f32,
                    (row as i32 * pixel_size) as f32,
                    pixel_size as f32,
                    pixel_size as f32,
                    &source,
                    &options,
                );
            }
        }

        surface
    }

    pub fn render(&self, pos: (isize, isize), buffer: &mut raqote::DrawTarget) {
        let pixel_size = PIXEL_SIZE as isize;

        let Some(x) = pos
            .0
            .checked_mul(pixel_size)
            .and_then(|x| i32::try_from(x).ok())
        else {
            return;
        };

        let Some(y) = pos
            .1
            .checked_mul(pixel_size)
            .and_then(|y| i32::try_from(y).ok())
        else {
            return;
        };

        let source_rect = raqote::IntRect::new(
            raqote::IntPoint::new(0, 0),
            raqote::IntPoint::new(self.surface.width(), self.surface.height()),
        );

        buffer.blend_surface(
            &self.surface,
            source_rect,
            raqote::IntPoint::new(x, y),
            raqote::BlendMode::SrcOver,
        );
    }
}
