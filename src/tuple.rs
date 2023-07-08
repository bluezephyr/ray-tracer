use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Tuple {
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
    fn inifinity() {
        let _inf = f64::INFINITY;
    }
}
