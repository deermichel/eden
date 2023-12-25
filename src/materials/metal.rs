use crate::base::{
    color::Color3f,
    material::{Interactable, Interaction},
    ray::Ray,
    shape::Intersection,
    vector::Vector3f,
};
use rand::thread_rng;

/// Metal material model.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metal {
    /// Fraction of light that the object reflects.
    albedo: Color3f,
}

impl Metal {
    /// Creates metal material with given albedo.
    pub fn new(albedo: Color3f) -> Self {
        Metal { albedo }
    }
}

impl Interactable for Metal {
    fn interact(&self, incident_ray: Ray, intersection: Intersection) -> Option<Interaction> {
        // Reflect at intersection normal.
        let reflected = incident_ray.direction().reflect(intersection.normal);

        // Return interaction struct.
        let interaction = Interaction {
            attenuation: self.albedo,
            scattered_ray: Ray::new(intersection.point, reflected),
        };
        Some(interaction)
    }
}
