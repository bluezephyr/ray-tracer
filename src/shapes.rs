use crate::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub pos: Tuple,
    pub radius: f64,
}

impl Sphere {
    pub fn new_unit_sphere() -> Sphere {
        Sphere {
            pos: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }
}
