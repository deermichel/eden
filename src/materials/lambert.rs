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
