use crate::color::Color;
use crate::matrices::{to_matrix, to_tuple, Matrix};
use crate::tuple::Tuple;

pub trait Normal {
    fn normal_at(&self, point: &Tuple) -> Tuple;
}

#[derive(Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: Color::color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[derive(Debug)]
pub struct Sphere {
    pub pos: Tuple,
    pub radius: f64,
    pub transformation: Matrix<4, 4>,
    pub material: Material,
}

impl Sphere {
    pub fn new_unit_sphere() -> Sphere {
        Sphere {
            pos: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
            transformation: Matrix::<4, 4>::new_identity(),
            material: Material::new(),
        }
    }
}

impl Normal for Sphere {
    fn normal_at(&self, point: &Tuple) -> Tuple {
        let object_point = &self.transformation.invert().unwrap() * &to_matrix(point);
        let object_normal = to_tuple(&object_point) - Tuple::point(0.0, 0.0, 0.0);
        let world_normal = to_tuple(
            &(&self.transformation.invert().unwrap().transpose() * &to_matrix(&object_normal)),
        );
        return world_normal.normalize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.00001;

    pub(crate) fn approx_eq(lhs: Tuple, rhs: Tuple, epsilon: f64) -> bool {
        (lhs.x - rhs.x).abs() < epsilon
            && (lhs.y - rhs.y).abs() < epsilon
            && (lhs.z - rhs.z).abs() < epsilon
            && (lhs.w - rhs.w).abs() < epsilon
    }

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

    #[test]
    fn normal_on_sphere_at_x_axis() {
        let s = Sphere::new_unit_sphere();
        assert!(approx_eq(
            s.normal_at(&Tuple::point(1.0, 0.0, 0.0)),
            Tuple::vector(1.0, 0.0, 0.0),
            EPSILON
        ));
    }

    #[test]
    fn normal_on_sphere_at_y_axis() {
        let s = Sphere::new_unit_sphere();
        assert!(approx_eq(
            s.normal_at(&Tuple::point(0.0, 1.0, 0.0)),
            Tuple::vector(0.0, 1.0, 0.0),
            EPSILON
        ));
    }

    #[test]
    fn normal_on_sphere_at_z_axis() {
        let s = Sphere::new_unit_sphere();
        assert!(approx_eq(
            s.normal_at(&Tuple::point(0.0, 0.0, 1.0)),
            Tuple::vector(0.0, 0.0, 1.0),
            EPSILON
        ));
    }

    #[test]
    fn normal_on_sphere_at_non_axial_point() {
        let s = Sphere::new_unit_sphere();
        assert!(approx_eq(
            s.normal_at(&Tuple::point(
                3_f64.sqrt() / 3.0,
                3_f64.sqrt() / 3.0,
                3_f64.sqrt() / 3.0
            )),
            Tuple::vector(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0),
            EPSILON
        ));
    }

    #[test]
    fn normal_on_translated_sphere() {
        let mut s = Sphere::new_unit_sphere();
        s.transformation = Matrix::new_identity().translate(0.0, 1.0, 0.0);
        assert!(approx_eq(
            s.normal_at(&Tuple::point(0.0, 1.70711, -0.70711)),
            Tuple::vector(0.0, 0.70711, -0.70711),
            EPSILON
        ));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = Sphere::new_unit_sphere();
        s.transformation = Matrix::new_identity()
            .rotate_z(std::f64::consts::PI / 5.0)
            .scale(1.0, 0.5, 1.0);
        assert!(approx_eq(
            s.normal_at(&Tuple::point(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0)),
            Tuple::vector(0.0, 0.97014, -0.24254),
            EPSILON
        ));
    }

    #[test]
    fn default_material() {
        let m = Material::new();
        assert_eq!(m.color, Color::color(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new_unit_sphere();
        let m = Material::new();
        assert_eq!(s.material, m);
    }

    #[test]
    fn sphere_can_be_assigned_material() {
        let mut s = Sphere::new_unit_sphere();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material.ambient, 1.0);
    }
}
