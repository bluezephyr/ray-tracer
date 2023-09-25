use crate::color::Color;
use crate::shapes::Material;
use crate::tuple::{dot, reflect, Tuple};

#[derive(Debug)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

// Lighting calculates the combination of the ambient, diffuse, and specular reflection for a point
// at a material that is affected by a light and observed at a specified location.
pub fn lighting(
    material: Material,
    light: PointLight,
    point: Tuple,
    eye_vector: Tuple,
    normal: Tuple,
) -> Color {
    let mut diffuse = Color::color(0.0, 0.0, 0.0);
    let mut specular = Color::color(0.0, 0.0, 0.0);

    // Combination of the material and the light intensity
    let effective_color = material.color * light.intensity;

    // The ambient contribution depend only on the material and the light
    let ambient = effective_color * material.ambient;

    let light_vector = (light.position - point).normalize();
    let light_dot_normal = dot(&light_vector, &normal);

    if light_dot_normal > 0.0 {
        // The diffuse reflection depends only on the angle between the light source and the normal
        diffuse = effective_color * material.diffuse * light_dot_normal;

        // The specular part depends on the angle between the reflection vector and the eye vector
        // |a| dot |b| = |a||b|cos v, where v is the angle between the vectors a and b
        // Note that the dot product is the cos value if the vectors are normalized
        let reflect_vector = reflect(&-light_vector, &normal).normalize();
        let reflect_dot_eye = dot(&reflect_vector, &eye_vector);

        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    // Add the components to get the lighting value
    // return ambient + diffuse + specular;
    return ambient + diffuse + specular;
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.00001;

    pub fn approx_eq(lhs: Color, rhs: Color) -> bool {
        (lhs.red - rhs.red).abs() < EPSILON
            && (lhs.green - rhs.green).abs() < EPSILON
            && (lhs.blue - rhs.blue).abs() < EPSILON
    }

    #[test]
    fn point_light_has_position_and_intensity() {
        let white = Color::color(1.0, 1.0, 1.0);
        let origo = Tuple::point(0.0, 0.0, 0.0);
        let light = PointLight::new(origo, white);
        assert_eq!(light.position, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(light.intensity, Color::color(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_when_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::point(0.0, 0.0, -1.0);
        let normal = Tuple::point(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::color(1.0, 1.0, 1.0));
        assert_eq!(
            lighting(m, light, position, eyev, normal),
            Color::color(1.9, 1.9, 1.9)
        );
    }

    #[test]
    fn lighting_eye_offset_45_deg() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::point(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normal = Tuple::point(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::color(1.0, 1.0, 1.0));
        assert_eq!(
            lighting(m, light, position, eyev, normal),
            Color::color(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn lighting_eye_offset_45_deg_opposite() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::point(0.0, 0.0, -1.0);
        let normal = Tuple::point(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::color(1.0, 1.0, 1.0));
        assert!(approx_eq(
            lighting(m, light, position, eyev, normal),
            Color::color(0.7364, 0.7364, 0.7364)
        ));
    }

    #[test]
    fn lighting_eye_in_path_of_reflection() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::point(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normal = Tuple::point(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::color(1.0, 1.0, 1.0));
        assert!(approx_eq(
            lighting(m, light, position, eyev, normal),
            Color::color(1.6364, 1.6364, 1.6364)
        ));
    }

    #[test]
    fn lighting_light_behind_the_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::point(0.0, 0.0, -1.0);
        let normal = Tuple::point(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::color(1.0, 1.0, 1.0));
        assert!(approx_eq(
            lighting(m, light, position, eyev, normal),
            Color::color(0.1, 0.1, 0.1)
        ));
    }
}
