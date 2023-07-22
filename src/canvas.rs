use crate::color::Color;
use std::vec;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    fn create(x: usize, y: usize) -> Canvas {
        let mut c = Canvas {
            width: x,
            height: y,
            pixels: vec![],
        };

        for _ in 0..y {
            let mut row: Vec<Color> = vec![];
            for _ in 0..x {
                let black_pixel = Color::color(0.0, 0.0, 0.0);
                row.push(black_pixel);
            }
            c.pixels.push(row);
        }
        return c;
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let row: &mut Vec<Color> = &mut self.pixels[y];
        row[x] = color;
    }

    fn read_pixel(&self, x: usize, y: usize) -> Color {
        let row = &self.pixels[y];
        return row[x];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_create() {
        let c = Canvas::create(10, 20);

        for row in 0..20 {
            for col in 0..10 {
                assert!(c.read_pixel(col, row) == Color::color(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn canvas_write_pixel() {
        let mut c = Canvas::create(10, 20);
        let red = Color::color(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert!(c.read_pixel(2, 3) == Color::color(1.0, 0.0, 0.0));
        assert!(c.read_pixel(2, 2) == Color::color(0.0, 0.0, 0.0));
    }
}
