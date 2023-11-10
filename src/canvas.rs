use crate::color::Color;
use std::vec;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Color>>,
    _x: usize,
    _y: usize,
}

pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Canvas {
    pub fn new(x: usize, y: usize) -> Canvas {
        let mut c = Canvas {
            width: x,
            height: y,
            pixels: vec![],
            _x: 0,
            _y: 0,
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

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            let row: &mut Vec<Color> = &mut self.pixels[y];
            row[x] = color;
        }
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if x < self.width && y < self.height {
            let row = &self.pixels[y];
            return Some(row[x]);
        } else {
            return None;
        }
    }
}

impl Iterator for Canvas {
    type Item = crate::color::Color;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self._x;
        let y = self._y;

        if self._y == self.height {
            self._y = 0;
            return None;
        }

        self._x = self._x + 1;
        if self._x == self.width {
            self._x = 0;
            self._y = self._y + 1;
        }
        return self.read_pixel(x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const COLS: usize = 20;
    const ROWS: usize = 10;

    #[test]
    fn canvas_create() {
        let c = Canvas::new(COLS, ROWS);

        for row in 0..ROWS {
            for col in 0..COLS {
                assert!(c.read_pixel(col, row) == Some(Color::color(0.0, 0.0, 0.0)));
            }
        }
    }

    #[test]
    fn canvas_write_pixel() {
        let mut c = Canvas::new(COLS, ROWS);
        let red = Color::color(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert!(c.read_pixel(2, 3) == Some(Color::color(1.0, 0.0, 0.0)));
        assert!(c.read_pixel(2, 2) == Some(Color::color(0.0, 0.0, 0.0)));
    }

    #[test]
    fn canvas_write_ignore_if_pixel_outside() {
        let mut c = Canvas::new(COLS, ROWS);
        let red = Color::color(1.0, 0.0, 0.0);
        c.write_pixel(21, 3, red);
        c.write_pixel(2, 13, red);
        assert!(c.read_pixel(20, 3) == None);
        assert!(c.read_pixel(2, 13) == None);
    }

    #[test]
    fn canvas_iterate_over_all_pixels() {
        let mut c = Canvas::new(COLS, ROWS);
        let red = Color::color(1.0, 0.0, 0.0);
        c.write_pixel(10, 3, red);
        c.write_pixel(2, 5, red);

        for y in 0..ROWS {
            for x in 0..COLS {
                assert!(c.next() == c.read_pixel(x, y));
            }
        }
    }
}
