struct Tuple {
    _x: f64,
    _y: f64,
    _z: f64,
    _w: f64,
}

impl Tuple {
    pub(crate) fn is_point(&self) -> bool {
        return self._w == 1.0;
    }

    pub(crate) fn is_vector(&self) -> bool {
        return self._w == 0.0;
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
}
