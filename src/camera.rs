use crate::matrices::{to_tuple, Matrix};
use crate::rays::Ray;
use crate::tuple::{cross, Tuple};
use crate::world::World;
use crate::Canvas;
use std::f64;

#[derive(Debug)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
    pub field_of_view: f64,
    pub transform: Matrix<4, 4>,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.).tan();
        let aspect = hsize as f64 / vsize as f64;
        let half_width = if aspect >= 1. {
            half_view
        } else {
            half_view * aspect
        };
        let half_height = if aspect >= 1. {
            half_view / aspect
        } else {
            half_view
        };
        Camera {
            hsize,
            vsize,
            half_width,
            half_height,
            pixel_size: (half_width * 2.) / hsize as f64,
            field_of_view,
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

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize - 1 {
            for x in 0..self.hsize - 1 {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write_pixel(x, y, color);
            }
        }

        image
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        // The untransformed pixel coordinates in the world space
        // Note that the camera looks toward -z, so +x is to the *left*
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        // Transform the canvas point and the origin point using the camera's transform
        // matrix and then compute the direction vector. Note that the canvas is at z = -1
        let pixel: Tuple =
            to_tuple(&(&self.transform.invert().unwrap() * &Tuple::point(world_x, world_y, -1.)));
        let origin: Tuple =
            to_tuple(&(&self.transform.invert().unwrap() * &Tuple::point(0., 0., 0.)));

        Ray {
            origin,
            direction: (pixel - origin).normalize(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;

    #[test]
    fn new_camera() {
        let camera = Camera::new(160, 120, f64::consts::PI / 2.);
        assert_eq!(camera.hsize, 160);
        assert_eq!(camera.vsize, 120);
        assert_eq!(camera.field_of_view, f64::consts::PI / 2.);
        assert_eq!(camera.transform, Matrix::new_identity());
    }

    #[test]
    fn camera_pixel_size_for_horizontal_canvas() {
        let camera = Camera::new(200, 125, f64::consts::PI / 2.);
        assert!((camera.pixel_size - 0.01).abs() < 0.000001);
    }

    #[test]
    fn camera_pixel_size_for_vertical_canvas() {
        let camera = Camera::new(125, 200, f64::consts::PI / 2.);
        assert!((camera.pixel_size - 0.01).abs() < 0.000001);
    }

    #[test]
    fn transformation_matrix_for_default_orientation() {
        let mut camera = Camera::new(1, 1, f64::consts::PI / 2.);
        let from = Tuple::point(0., 0., 0.);
        let to = Tuple::point(0., 0., -1.);
        let up = Tuple::vector(0., 1., 0.);
        camera.set_view_transformation(&from, &to, &up);
        assert_eq!(camera.transform, Matrix::new_identity());
    }

    #[test]
    fn transformation_matrix_looking_in_positive_z_direction() {
        let mut camera = Camera::new(1, 1, f64::consts::PI / 2.);
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
        let mut camera = Camera::new(1, 1, f64::consts::PI / 2.);
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
        let mut camera = Camera::new(1, 1, f64::consts::PI / 2.);
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

    #[test]
    fn ray_through_center_of_canvas() {
        let camera = Camera::new(201, 101, f64::consts::PI / 2.);
        let ray = camera.ray_for_pixel(100, 50);
        assert_eq!(ray.origin, Tuple::point(0., 0., 0.));
        assert_eq!(ray.direction, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn ray_through_a_corner_of_canvas() {
        let camera = Camera::new(201, 101, f64::consts::PI / 2.);
        let ray = camera.ray_for_pixel(0, 0);
        assert_eq!(ray.origin, Tuple::point(0., 0., 0.));
        assert_eq!(ray.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn ray_through_canvas_when_camera_transformed() {
        let mut camera = Camera::new(201, 101, f64::consts::PI / 2.);
        camera.transform = Matrix::new_identity()
            .translate(0., -2., 5.)
            .rotate_y(f64::consts::PI / 4.);
        let ray = camera.ray_for_pixel(100, 50);
        assert_eq!(ray.origin, Tuple::point(0., 2., -5.));
        assert_eq!(
            ray.direction,
            Tuple::vector(2_f64.sqrt() / 2., 0., -2_f64.sqrt() / 2.)
        );
    }

    #[test]
    fn render_world_in_camera() {
        let world = World::default_world();
        let mut camera = Camera::new(11, 11, f64::consts::PI / 2.);
        camera.set_view_transformation(
            &Tuple::point(0., 0., -5.),
            &Tuple::point(0., 0., 0.),
            &Tuple::vector(0., 1., 0.),
        );
        let image = camera.render(&world);
        assert_eq!(
            image.read_pixel(5, 5),
            Some(Color::color(0.38066, 0.47583, 0.2855))
        );
    }
}
