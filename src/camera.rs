use crate::tuple::{cross, Tuple};
use crate::Matrix;

#[derive(Debug)]
pub struct Camera {
    pub transform: Matrix<4, 4>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            transform: Matrix::new_identity(),
        }
    }

    pub fn set_view_transformation(&mut self, from: &Tuple, to: &Tuple, up: &Tuple) {
        let forward = (to - from).normalize();
        let up_normalized = up.normalize();
        let left = cross(&forward, &up_normalized);
        let true_up = cross(&left, &forward);
        let orientation = Matrix::<4, 4>::new_init([
            [left.x, left.y, left.z, 0.],
            [true_up.x, true_up.y, true_up.z, 0.],
            [-forward.x, -forward.y, -forward.z, 0.],
            [0., 0., 0., 1.],
        ]);

        self.transform =
            &orientation * &Matrix::new_identity().translate(-from.x, -from.y, -from.z);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transformation_matrix_for_default_orientation() {
        let mut camera = Camera::new();
        let from = Tuple::point(0., 0., 0.);
        let to = Tuple::point(0., 0., -1.);
        let up = Tuple::vector(0., 1., 0.);
        camera.set_view_transformation(&from, &to, &up);
        assert_eq!(camera.transform, Matrix::new_identity());
    }

    #[test]
    fn transformation_matrix_looking_in_positive_z_direction() {
        let mut camera = Camera::new();
        let from = Tuple::point(0., 0., 0.);
        let to = Tuple::point(0., 0., 1.);
        let up = Tuple::vector(0., 1., 0.);
        camera.set_view_transformation(&from, &to, &up);
        assert_eq!(
            camera.transform,
            Matrix::new_identity().scale(-1.0, 1.0, -1.0)
        );
    }

    #[test]
    fn transformation_moves_the_world() {
        let mut camera = Camera::new();
        camera.set_view_transformation(
            &Tuple::point(0., 0., 8.),
            &Tuple::point(0., 0., 0.),
            &Tuple::vector(0., 1., 0.),
        );
        assert_eq!(
            camera.transform,
            Matrix::new_identity().translate(0., 0., -8.)
        );
    }

    #[test]
    fn arbitrary_transformation() {
        let mut camera = Camera::new();
        camera.set_view_transformation(
            &Tuple::point(1., 3., 2.),
            &Tuple::point(4., -2., 8.),
            &Tuple::vector(1., 1., 0.),
        );
        assert_eq!(
            camera.transform,
            Matrix::new_init([
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.],
                [0., 0., 0., 1.],
            ])
        );
    }
}
