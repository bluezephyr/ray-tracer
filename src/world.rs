use crate::lights::PointLight;
use crate::shapes::{Material, Sphere};
use crate::Color;
use crate::Matrix;
use crate::Tuple;

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    // The default world contains a light source and two spheres
    pub fn default_world() -> World {
        let mut w = World::new();
        let light = PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Color::color(1.0, 1.0, 1.0),
        );
        let m1 = Material {
            color: Color::color(0.8, 1.0, 0.6),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
        };
        let s1 = Sphere {
            pos: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
            transformation: Matrix::<4, 4>::new_identity(),
            material: m1,
        };
        let mut s2 = Sphere::new();
        s2.transformation = Matrix::new_identity().scale(0.5, 0.5, 0.5);

        w.lights.push(light);
        w.objects.push(s1);
        w.objects.push(s2);
        return w;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_world() {
        let world = World::new();

        assert!(world.objects.len() == 0);
        assert!(world.lights.len() == 0);
    }

    #[test]
    fn default_world() {
        let light = PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Color::color(1.0, 1.0, 1.0),
        );
        let m1 = Material {
            color: Color::color(0.8, 1.0, 0.6),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
        };
        let s1 = Sphere {
            pos: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
            transformation: Matrix::<4, 4>::new_identity(),
            material: m1,
        };
        let mut s2 = Sphere::new();
        s2.transformation = Matrix::new_identity().scale(0.5, 0.5, 0.5);
        let world = World::default_world();

        assert!(world.lights.contains(&light));
        assert!(world.objects.contains(&s1));
        assert!(world.objects.contains(&s2));
    }
}
