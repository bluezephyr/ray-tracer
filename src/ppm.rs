use std::path::Path;
pub(crate) use std::fs::File;
use crate::{canvas::Canvas, color::Color};
use std::io::prelude::*;

pub struct Ppm {
    filename: String,
    x: usize,
    y: usize,
    pub lines: Vec<String>,
}

impl Ppm {
    pub fn new(filename: String) -> Ppm {
        let ppm = Ppm {
            filename,
            x: 0,
            y: 0,
            lines: vec![],
        };
        return ppm;
    }

    pub fn save_canvas(&mut self, canvas: Canvas) {
        self.x = canvas.width;
        self.y = canvas.height;
        self.add_header();
        let mut pixels: Vec<String> = vec![];

        for pixel in canvas {
            let pixel_string = self.color_to_ppm_string(pixel);
            pixels.push(pixel_string);
        }

        self.lines.push(pixels.join(" "));
        self.write();
    }

    fn color_to_ppm_string(&mut self, color: Color) -> String {
        let n_color = color.normalize_u8();
        return format!("{0} {1} {2}", n_color.red, n_color.green, n_color.blue)
    }

    // The header is three lines long
    // "P3" - Magic Number
    // Width Height
    // Max color value
    fn add_header(&mut self) {
        self.lines.push("P3".to_string());
        self.lines
            .push(format!("{0} {1}", self.x, self.y).to_string());
        self.lines.push("255".to_string());
    }

    // Write the lines to file
    fn write(&self) {
        let path = Path::new(&self.filename);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        for line in &self.lines {
            match file.write_all(line.as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => (),
            }
            match file.write("\n".as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ppm_empty_after_creation() {
        let p = Ppm::new("test".to_string());
        assert!(p.lines.len() == 0);
    }

    #[test]
    fn ppm_create_canvas_header() {
        let canvas = Canvas::create(5, 3);
        let mut p = Ppm::new("test".to_string());
        p.save_canvas(canvas);
        assert!(p.lines[0] == "P3"); // Magic number
        assert!(p.lines[1] == "5 3"); // Width and height
        assert!(p.lines[2] == "255"); // Max color value
        assert!(p.x == 5);
        assert!(p.y == 3);
    }

    #[test]
    fn ppm_create_canvas_pixel_data() {
        let mut canvas = Canvas::create(5, 3);
        canvas.write_pixel(0, 0, Color::color(1.5, 0.0, 0.0));
        canvas.write_pixel(2, 1, Color::color(0.0, 0.5, 0.0));
        canvas.write_pixel(4, 2, Color::color(-0.5, 0.0, 1.5));
        let mut p = Ppm::new("test".to_string());
        p.save_canvas(canvas);
        assert!(p.lines[3] == "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");

        // assert!(p.lines[3] == "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        // assert!(p.lines[4] == "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        // assert!(p.lines[5] == "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }
}
