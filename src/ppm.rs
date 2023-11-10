use crate::{canvas::Canvas, color::Color};
pub(crate) use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Ppm {
    filename: String,
    pub lines: Vec<String>,
}

const MAX_LINELEN: usize = 70;

impl Ppm {
    pub fn new(filename: String) -> Ppm {
        let ppm = Ppm {
            filename,
            lines: vec![],
        };
        return ppm;
    }

    pub fn add_canvas(&mut self, canvas: Canvas) {
        self.add_header(canvas.width, canvas.height);
        let mut pixels: Vec<String> = vec![];
        let mut linelen = 0;

        for pixel in canvas {
            let pixel_string = self.color_to_ppm_string(pixel);
            if linelen + pixel_string.len() > MAX_LINELEN {
                self.lines.push(pixels.join(" "));
                pixels.clear();
                linelen = 0;
            }
            linelen = linelen + pixel_string.len() + 1;
            pixels.push(pixel_string);
        }
        self.lines.push(pixels.join(" "));
    }

    fn color_to_ppm_string(&mut self, color: Color) -> String {
        let n_color = color.normalize_u8();
        return format!("{0} {1} {2}", n_color.red, n_color.green, n_color.blue);
    }

    // The header is three lines long
    // "P3" - Magic Number
    // Width Height
    // Max color value
    fn add_header(&mut self, x: usize, y: usize) {
        self.lines.push("P3".to_string());
        self.lines.push(format!("{0} {1}", x, y).to_string());
        self.lines.push("255".to_string());
    }

    // Write the lines to file
    pub fn write_file(&self) {
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
        let canvas = Canvas::new(5, 3);
        let mut p = Ppm::new("test".to_string());
        p.add_canvas(canvas);
        assert!(p.lines[0] == "P3"); // Magic number
        assert!(p.lines[1] == "5 3"); // Width and height
        assert!(p.lines[2] == "255"); // Max color value
    }

    #[test]
    fn ppm_create_canvas_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        canvas.write_pixel(0, 0, Color::color(1.5, 0.0, 0.0));
        canvas.write_pixel(2, 1, Color::color(0.0, 0.5, 0.0));
        canvas.write_pixel(4, 2, Color::color(-0.5, 0.0, 1.5));
        let mut p = Ppm::new("test".to_string());
        p.add_canvas(canvas);
        assert!(
            p.lines[3] == "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 0 0 0"
        );
        assert!(p.lines[4] == "0 0 0 0 0 0 0 0 0 0 0 255");
    }
}
