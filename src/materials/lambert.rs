use crate::base::{
    color::Color3f,
    material::{Interactable, Interaction},
    ray::Ray,
    shape::Intersection,
    vector::Vector3f,
};
use rand::thread_rng;

/// Lambertian material model.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lambert {
    /// Fraction of light that the object reflects.
    albedo: Color3f,
}

impl Lambert {
    /// Creates lambertian material with given albedo.
    pub fn new(albedo: Color3f) -> Self {
        Lambert { albedo }
    }
}

impl Interactable for Lambert {
    fn interact(&self, _incident_ray: Ray, intersection: Intersection) -> Option<Interaction> {
        let mut rng = thread_rng();

        // Lambertian distribution.
        let mut scattered = intersection.normal + Vector3f::random_unit_vector(&mut rng);

        // Catch degenerate scatter direction.
        if scattered.near_zero() {
            scattered = intersection.normal;
        }

        // Return interaction struct.
        let interaction = Interaction {
            attenuation: self.albedo,
            scattered_ray: Ray::new(intersection.point, scattered),
        };
        Some(interaction)
    }
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::{material::Material, point::Point3f};

    #[test]
    fn interact() {
        let albedo = Color3f::new(1.0, 1.0, 0.0);
        let l = Lambert::new(albedo);
        let mat = Material::Lambert(l);
        let r = Ray::new(Point3f::default(), Vector3f::new(2.0, -2.0, 0.0));
        let isect = Intersection {
            point: Point3f::new(1.0, 1.0, 1.0),
            normal: Vector3f::new(0.0, 1.0, 0.0),
            material: &mat,
            t: 1.0,
        };
        let iact = mat.interact(r, isect).unwrap();
        assert_eq!(iact.attenuation, albedo);
        assert_eq!(iact.scattered_ray.origin(), isect.point);
        assert!(iact.scattered_ray.direction().dot(&isect.normal) >= 0.0);
    }
}
