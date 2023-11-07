use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub(crate) fn point(x: f64, y: f64, z: f64) -> Tuple {
        return Tuple { x, y, z, w: 1.0 };
    }

    pub(crate) fn vector(x: f64, y: f64, z: f64) -> Tuple {
        return Tuple { x, y, z, w: 0.0 };
    }

    pub(crate) fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub(crate) fn normalize(&self) -> Tuple {
        return Tuple {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: 0.0,
        };
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Tuple {
        return Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        };
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Tuple {
        return Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        };
    }
}

impl Sub for &Tuple {
    type Output = Tuple;

    fn sub(self, rhs: &Tuple) -> Tuple {
        return Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        };
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        return Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        };
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Tuple {
        return Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        };
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Tuple {
        return Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        };
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 0.00001;
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

pub fn dot(lhs: &Tuple, rhs: &Tuple) -> f64 {
    return lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w;
}

pub fn cross(lhs: &Tuple, rhs: &Tuple) -> Tuple {
    return Tuple::vector(
        lhs.y * rhs.z - lhs.z * rhs.y,
        lhs.z * rhs.x - lhs.x * rhs.z,
        lhs.x * rhs.y - lhs.y * rhs.x,
    );
}

pub fn reflect(vector: &Tuple, normal: &Tuple) -> Tuple {
    return vector.sub(&(*normal * 2.0 * dot(vector, normal)));
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Tuple {
        pub(crate) fn is_point(&self) -> bool {
            return self.w == 1.0;
        }

        pub(crate) fn is_vector(&self) -> bool {
            return self.w == 0.0;
        }
    }

    #[test]
    fn tuple_with_w_1_0_is_a_point() {
        let a = Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        };
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_0_0_is_a_vector() {
        let a = Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
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
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let neg_a = Tuple {
            x: -1.0,
            y: 2.0,
            z: -3.0,
            w: 4.0,
        };
        assert!(-a == neg_a);
    }

    #[test]
    fn multiply_tuple_with_scalar() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let m_a = Tuple {
            x: 3.5,
            y: -7.0,
            z: 10.5,
            w: -14.0,
        };
        assert!(a * 3.5 == m_a);
        println!("{:?}", a);
    }

    #[test]
    fn multiply_tuple_with_fraction() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let m_a = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };
        assert!(a * 0.5 == m_a);
        println!("{:?}", a);
    }

    #[test]
    fn divide_tuple_with_scalar() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let m_a = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
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

    #[test]
    fn reflect_vector_at_45_deg() {
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let normal = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(reflect(&v, &normal), Tuple::vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_vector_off_slanted_surface() {
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let normal = Tuple::vector(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        assert_eq!(reflect(&v, &normal), Tuple::vector(1.0, 0.0, 0.0));
    }
}
