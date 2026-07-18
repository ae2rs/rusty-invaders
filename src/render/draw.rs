use super::Pixel;

pub fn draw(
    pixels: Vec<Vec<Option<Pixel>>>,
    pos: (usize, usize),
    screen: &mut Vec<Vec<Option<Pixel>>>,
) {
    for (row, pixels) in pixels.iter().enumerate() {
        for (col, pixel) in pixels.iter().enumerate() {
            if pixel.is_some() {
                screen[pos.1 + row][pos.0 + col] = Some(pixel.unwrap());
            }
        }
    }
}
