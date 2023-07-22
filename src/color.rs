use std::ops::{Add, Mul, Sub};
const EPSILON: f64 = 0.00001;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub fn approximate_eq(lhs: Color, rhs: Color) -> bool {
    (lhs.red - rhs.red).abs() < EPSILON
        && (lhs.red - rhs.red).abs() < EPSILON
        && (lhs.red - rhs.red).abs() < EPSILON
}

impl Color {
    pub(crate) fn color(red: f64, green: f64, blue: f64) -> Color {
        return Color { red, green, blue };
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        return Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        };
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        return Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        };
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        return Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        };
    }
}

// Hadamart product
impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        return Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        };
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        return self.red == other.red && self.green == other.green && self.blue == other.blue;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_create() {
        let c = Color::color(-0.5, 0.4, 1.7);
        assert!(c.red == -0.5);
        assert!(c.green == 0.4);
        assert!(c.blue == 1.7);
    }

    #[test]
    fn color_add() {
        let c1 = Color::color(0.9, 0.6, 0.75);
        let c2 = Color::color(0.7, 0.1, 0.25);
        assert!(c1 + c2 == Color::color(1.6, 0.7, 1.0));
    }

    #[test]
    fn color_subtract() {
        let c1 = Color::color(0.9, 0.6, 0.75);
        let c2 = Color::color(0.7, 0.1, 0.25);
        assert!(approximate_eq(c1 - c2, Color::color(0.2, 0.5, 0.5)));
    }

    #[test]
    fn color_multiply_with_scalar() {
        let c1 = Color::color(0.2, 0.3, 0.4);
        assert!(approximate_eq(c1 * 2.0, Color::color(0.4, 0.6, 0.8)));
    }

    #[test]
    fn color_multiply_with_color() {
        let c1 = Color::color(1.0, 0.2, 0.4);
        let c2 = Color::color(0.9, 1.0, 0.1);
        assert!(approximate_eq(c1 * c2, Color::color(0.9, 0.2, 0.04)));
    }
}
