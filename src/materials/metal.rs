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

    /// Reflection fuzz factor.
    fuzz: f32,
}

impl Metal {
    /// Creates metal material with given albedo.
    pub fn new(albedo: Color3f, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Interactable for Metal {
    fn interact(&self, incident_ray: Ray, intersection: Intersection) -> Option<Interaction> {
        let mut rng = thread_rng();

        // Reflect at intersection normal.
        let reflected = incident_ray.direction().reflect(intersection.normal);

        // Apply fuzz.
        let mut scattered =
            reflected.normalize() + self.fuzz * Vector3f::random_unit_vector(&mut rng);

        // Catch degenerate scatter direction.
        if scattered.near_zero() {
            scattered = reflected;
        }

        // Absorb scatters below surface.
        if scattered.dot(&intersection.normal) < 0.0 {
            return None;
        }

        // Return interaction struct.
        let interaction = Interaction {
            attenuation: self.albedo,
            scattered_ray: Ray::new(intersection.point, scattered),
        };
        Some(interaction)
    }
}
