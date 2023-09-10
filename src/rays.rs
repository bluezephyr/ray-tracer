use crate::shapes::Sphere;
use crate::tuple::{dot, Tuple};

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

// The referenced object must live at least as long as the intersection object
impl<'a> Intersection<'a> {
    fn new(t: f64, object: &'a Sphere) -> Intersection<'a> {
        Intersection { t, object }
    }
}

// Return a reference to the intersection with the lowest non-negative t value
pub fn hit<'a>(intersections: &'a Vec<Intersection<'a>>) -> Option<&'a Intersection<'a>> {
    if intersections.is_empty() {
        None
    } else {
        let mut min_t = None;
        for intersection in intersections {
            if intersection.t >= 0.0 {
                if min_t.is_none() {
                    min_t = Some(intersection);
                } else if intersection.t < min_t.unwrap().t {
                    min_t = Some(intersection);
                }
            }
        }
        min_t
    }
}

impl Ray {
    // origin is a point and direction is a vector
    fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    // Find all points where the ray intersects the sphere.
    // Needs to be refactored when more shapes are added.
    fn intersects<'a>(&self, sphere: &'a Sphere) -> Vec<Intersection<'a>> {
        let mut intersections = Vec::new();
        let sphere_to_ray = self.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = dot(&self.direction, &self.direction);
        let b = 2.0 * dot(&self.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
        let discriminant = b.powf(2.0) - 4.0 * a * c;
        if discriminant >= 0.0 {
            intersections.push(Intersection {
                t: (-b - discriminant.sqrt()) / (2.0 * a),
                object: sphere,
            });
            intersections.push(Intersection {
                t: (-b + discriminant.sqrt()) / (2.0 * a),
                object: sphere,
            });
        }
        return intersections;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn create_and_query_a_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn compute_a_point_from_a_distance() {
        let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new_unit_sphere();
        let intersections = r.intersects(&s);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 5.0);
        assert_eq!(intersections[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new_unit_sphere();
        let intersections = r.intersects(&s);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn ray_originates_within_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new_unit_sphere();
        let intersections = r.intersects(&s);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -1.0);
        assert_eq!(intersections[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new_unit_sphere();
        let intersections = r.intersects(&s);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -6.0);
        assert_eq!(intersections[1].t, -4.0);
    }

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new_unit_sphere();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert!(ptr::eq(i.object, &s));
    }

    #[test]
    fn intersect_registers_object_at_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new_unit_sphere();
        let intersections = r.intersects(&s);
        assert_eq!(intersections.len(), 2);
        assert!(ptr::eq(intersections[0].object, &s));
        assert!(ptr::eq(intersections[1].object, &s));
    }

    #[test]
    fn no_hit_when_all_intersections_negative() {
        let s = Sphere::new_unit_sphere();
        let intersections = vec![Intersection::new(-2.0, &s), Intersection::new(-1.0, &s)];
        assert!(hit(&intersections).is_none());
    }

    #[test]
    fn get_hit_when_all_intersections_positive() {
        let s = Sphere::new_unit_sphere();
        let intersections = vec![Intersection::new(1.0, &s), Intersection::new(2.0, &s)];
        let i = hit(&intersections).unwrap();
        assert!(ptr::eq(i, &intersections[0] as *const Intersection));
    }

    #[test]
    fn get_hit_when_some_intersections_negative() {
        let s = Sphere::new_unit_sphere();
        let intersections = vec![Intersection::new(-1.0, &s), Intersection::new(2.0, &s)];
        let i = hit(&intersections).unwrap();
        assert!(ptr::eq(i, &intersections[1] as *const Intersection));
    }

    #[test]
    fn hit_intersection_with_lowest_positive_t() {
        let s = Sphere::new_unit_sphere();
        let intersections = vec![
            Intersection::new(5.0, &s),
            Intersection::new(7.0, &s),
            Intersection::new(-3.0, &s),
            Intersection::new(2.0, &s),
        ];
        assert!(ptr::eq(
            hit(&intersections).unwrap(),
            &intersections[3] as *const Intersection
        ));
    }
}
