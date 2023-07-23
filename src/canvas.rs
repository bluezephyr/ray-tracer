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
        if x < self.width && y < self.height {
            let row: &mut Vec<Color> = &mut self.pixels[y];
            row[x] = color;
        }
    }

    fn read_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if x < self.width && y < self.height {
            let row = &self.pixels[y];
            return Some(row[x]);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const COLS: usize = 20;
    const ROWS: usize = 10;

    #[test]
    fn canvas_create() {
        let c = Canvas::create(COLS, ROWS);

        for row in 0..ROWS {
            for col in 0..COLS {
                assert!(c.read_pixel(col, row) == Some(Color::color(0.0, 0.0, 0.0)));
            }
        }
    }

    #[test]
    fn canvas_write_pixel() {
        let mut c = Canvas::create(COLS, ROWS);
        let red = Color::color(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert!(c.read_pixel(2, 3) == Some(Color::color(1.0, 0.0, 0.0)));
        assert!(c.read_pixel(2, 2) == Some(Color::color(0.0, 0.0, 0.0)));
    }

    #[test]
    fn canvas_write_ignore_if_pixel_outside() {
        let mut c = Canvas::create(COLS, ROWS);
        let red = Color::color(1.0, 0.0, 0.0);
        c.write_pixel(21, 3, red);
        c.write_pixel(2, 13, red);
        assert!(c.read_pixel(21, 3) == None);
        assert!(c.read_pixel(2, 13) == None);
    }
}
