use crate::lights::{lighting, PointLight};
use crate::rays::{hit, Computation, Ray};
use crate::shapes::{Material, Sphere};
use crate::Color;
use crate::Matrix;
use crate::Tuple;

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<PointLight>,
}

// Only one light used in the calculations at the moment
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

    pub fn shade_hit(&self, computation: &Computation) -> Color {
        lighting(
            &computation.object.material,
            &self.lights[0],
            &computation.point,
            &computation.eyev,
            &computation.normalv,
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        match hit(&ray.intersections_in_world(self)) {
            Some(intersection) => self.shade_hit(&ray.prepare_computation(intersection)),
            None => Color::color(0., 0., 0.),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rays::Intersection;

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

    #[test]
    fn shade_an_intersection() {
        let world = World::default_world();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        assert_eq!(
            world.shade_hit(&ray.prepare_computation(&Intersection::new(4., &world.objects[0]))),
            Color::color(0.38066, 0.47583, 0.2855)
        );
    }

    #[test]
    fn shade_an_intersection_from_inside() {
        let mut world = World::default_world();
        world.lights[0] = PointLight::new(Tuple::point(0., 0.25, 0.), Color::color(1.0, 1.0, 1.0));
        let ray = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        // Check the second object in the default world
        assert_eq!(
            world.shade_hit(&ray.prepare_computation(&Intersection::new(0.5, &world.objects[1]))),
            Color::color(0.90498, 0.90498, 0.90498)
        );
    }

    #[test]
    fn color_black_when_a_ray_misses() {
        let world = World::default_world();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.));
        assert_eq!(world.color_at(&ray), Color::color(0., 0., 0.));
    }

    #[test]
    fn color_when_a_ray_hits() {
        let world = World::default_world();
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        assert_eq!(world.color_at(&ray), Color::color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_the_ray() {
        let mut world = World::default_world();
        world.objects[0].material.ambient = 1.; // Outer object
        world.objects[1].material.ambient = 1.; // Inner object
        let ray = Ray::new(Tuple::point(0., 0., 0.75), Tuple::vector(0., 0., -1.));
        let color = world.color_at(&ray);
        assert_eq!(color, world.objects[1].material.color);
    }
}
