use crate::matrices::Matrix;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Sphere {
    pub pos: Tuple,
    pub radius: f64,
    pub transformation: Matrix<4, 4>,
}

impl Sphere {
    pub fn new_unit_sphere() -> Sphere {
        Sphere {
            pos: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
            transformation: Matrix::<4, 4>::new_identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_matrix_default_transformation_for_sphere() {
        let s = Sphere::new_unit_sphere();
        assert_eq!(s.transformation, Matrix::<4, 4>::new_identity());
    }

    #[test]
    fn change_transformation_for_sphere() {
        let mut s = Sphere::new_unit_sphere();
        s.transformation = Matrix::new_identity().translate(2.0, 3.0, 4.0);
        assert_eq!(
            s.transformation,
            Matrix::new_identity().translate(2.0, 3.0, 4.0)
        );
    }
}
