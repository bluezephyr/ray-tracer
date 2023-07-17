use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    _x: f64,
    _y: f64,
    _z: f64,
    _w: f64,
}

impl Tuple {
    pub(crate) fn point(x: f64, y: f64, z: f64) -> Tuple {
        return Tuple {
            _x: x,
            _y: y,
            _z: z,
            _w: 1.0,
        };
    }

    pub(crate) fn vector(x: f64, y: f64, z: f64) -> Tuple {
        return Tuple {
            _x: x,
            _y: y,
            _z: z,
            _w: 0.0,
        };
    }

    pub(crate) fn is_point(&self) -> bool {
        return self._w == 1.0;
    }

    pub(crate) fn is_vector(&self) -> bool {
        return self._w == 0.0;
    }

    pub(crate) fn magnitude(&self) -> f64 {
        (self._x.powf(2.0) + self._y.powf(2.0) + self._z.powf(2.0)).sqrt()
    }

    pub(crate) fn normalize(&self) -> Tuple {
        return Tuple {
            _x: self._x / self.magnitude(),
            _y: self._y / self.magnitude(),
            _z: self._z / self.magnitude(),
            _w: 0.0,
        };
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Tuple {
        return Tuple {
            _x: self._x + rhs._x,
            _y: self._y + rhs._y,
            _z: self._z + rhs._z,
            _w: self._w + rhs._w,
        };
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Tuple {
        return Tuple {
            _x: self._x - rhs._x,
            _y: self._y - rhs._y,
            _z: self._z - rhs._z,
            _w: self._w - rhs._w,
        };
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        return Tuple {
            _x: -self._x,
            _y: -self._y,
            _z: -self._z,
            _w: -self._w,
        };
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Tuple {
        return Tuple {
            _x: self._x * rhs,
            _y: self._y * rhs,
            _z: self._z * rhs,
            _w: self._w * rhs,
        };
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Tuple {
        return Tuple {
            _x: self._x / rhs,
            _y: self._y / rhs,
            _z: self._z / rhs,
            _w: self._w / rhs,
        };
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        return self._x == other._x
            && self._y == other._y
            && self._z == other._z
            && self._w == other._w;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

pub fn dot(lhs: &Tuple, rhs: &Tuple) -> f64 {
    return lhs._x * rhs._x + lhs._y * rhs._y + lhs._z * rhs._z + lhs._w * rhs._w;
}

pub fn cross(lhs: &Tuple, rhs: &Tuple) -> Tuple {
    return Tuple::vector(
        lhs._y * rhs._z - lhs._z * rhs._y,
        lhs._z * rhs._x - lhs._x * rhs._z,
        lhs._x * rhs._y - lhs._y * rhs._x,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_0_is_a_point() {
        let a = Tuple {
            _x: 0.0,
            _y: 0.0,
            _z: 0.0,
            _w: 1.0,
        };
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_0_0_is_a_vector() {
        let a = Tuple {
            _x: 0.0,
            _y: 0.0,
            _z: 0.0,
            _w: 0.0,
        };
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_method_creates_point() {
        let p = Tuple::point(1.0, 2.0, -3.0);
        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn vector_method_creates_vector() {
        let v = Tuple::vector(2.0, -3.0, 4.0);
        assert!(!v.is_point());
        assert!(v.is_vector());
    }

    #[test]
    fn add_point_and_vector_creates_point() {
        let p = Tuple::point(3.0, -2.0, 5.0);
        let v = Tuple::vector(-2.0, 3.0, 1.0);
        assert!(p + v == Tuple::point(1.0, 1.0, 6.0));
        println!("{:?}", p);
    }

    #[test]
    fn add_two_vectors_creates_vector() {
        let v1 = Tuple::vector(3.0, -3.0, 4.0);
        let v2 = Tuple::vector(-2.0, 4.0, 2.0);
        assert!(v1 + v2 == Tuple::vector(1.0, 1.0, 6.0));
        println!("{:?}", v1);
    }

    #[test]
    fn subtract_two_vectors_creates_vector() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        assert!(v1 - v2 == Tuple::vector(-2.0, -4.0, -6.0));
        println!("{:?}", v1);
    }

    #[test]
    fn subtract_vector_from_point_creates_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        assert!(p - v == Tuple::point(-2.0, -4.0, -6.0));
        println!("{:?}", p);
    }

    #[test]
    fn subtract_two_points_creates_vector() {
        let p1 = Tuple::point(3.0, 3.0, 4.0);
        let p2 = Tuple::point(2.0, 4.0, 2.0);
        assert!(p1 - p2 == Tuple::vector(1.0, -1.0, 2.0));
        println!("{:?}", p1);
    }

    #[test]
    fn subtract_vector_from_zero_vector_negates_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert!(zero - v == Tuple::vector(-1.0, 2.0, -3.0));
        println!("{:?}", v);
    }

    #[test]
    fn negate_tuple_negates_all_components_of_tuple() {
        let a = Tuple {
            _x: 1.0,
            _y: -2.0,
            _z: 3.0,
            _w: -4.0,
        };
        let neg_a = Tuple {
            _x: -1.0,
            _y: 2.0,
            _z: -3.0,
            _w: 4.0,
        };
        assert!(-a == neg_a);
    }

    #[test]
    fn multiply_tuple_with_scalar() {
        let a = Tuple {
            _x: 1.0,
            _y: -2.0,
            _z: 3.0,
            _w: -4.0,
        };
        let m_a = Tuple {
            _x: 3.5,
            _y: -7.0,
            _z: 10.5,
            _w: -14.0,
        };
        assert!(a * 3.5 == m_a);
        println!("{:?}", a);
    }

    #[test]
    fn multiply_tuple_with_fraction() {
        let a = Tuple {
            _x: 1.0,
            _y: -2.0,
            _z: 3.0,
            _w: -4.0,
        };
        let m_a = Tuple {
            _x: 0.5,
            _y: -1.0,
            _z: 1.5,
            _w: -2.0,
        };
        assert!(a * 0.5 == m_a);
        println!("{:?}", a);
    }

    #[test]
    fn divide_tuple_with_scalar() {
        let a = Tuple {
            _x: 1.0,
            _y: -2.0,
            _z: 3.0,
            _w: -4.0,
        };
        let m_a = Tuple {
            _x: 0.5,
            _y: -1.0,
            _z: 1.5,
            _w: -2.0,
        };
        assert!(a / 2.0 == m_a);
        println!("{:?}", a);
    }

    #[test]
    fn magnitude_vector_1_0_0_is_1() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert!(v.magnitude() == 1.0);
        println!("{:?}", v);
    }

    #[test]
    fn magnitude_vector_0_1_0_is_1() {
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert!(v.magnitude() == 1.0);
        println!("{:?}", v);
    }

    #[test]
    fn magnitude_vector_0_0_1_is_1() {
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert!(v.magnitude() == 1.0);
        println!("{:?}", v);
    }

    #[test]
    fn magnitude_vector_1_2_3_is_sqrt_14() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert!(v.magnitude() == 14.0_f64.sqrt());
        println!("{:?}", v);
    }

    #[test]
    fn normalize_vector_4_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert!(v.normalize() == Tuple::vector(1.0, 0.0, 0.0));
        println!("{:?}", v);
    }

    #[test]
    fn normalize_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert!(
            v.normalize()
                == Tuple::vector(
                    1.0 / 14.0_f64.sqrt(),
                    2.0 / 14.0_f64.sqrt(),
                    3.0 / 14.0_f64.sqrt()
                )
        );
        println!("{:?}", v);
    }

    #[test]
    fn magnitude_of_normalized_vector_is_1() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert!(v.normalize().magnitude() == 1.0);
        println!("{:?}", v);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        assert!(dot(&v1, &v2) == 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        assert!(cross(&v1, &v2) == Tuple::vector(-1.0, 2.0, -1.0));
        assert!(cross(&v2, &v1) == Tuple::vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn inifinity() {
        let _inf = f64::INFINITY;
    }
}
