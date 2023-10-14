use crate::lights::PointLight;
use crate::shapes::Sphere;

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
}
