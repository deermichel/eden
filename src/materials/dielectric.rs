use crate::base::{
    color::Color3f,
    material::{Interactable, Interaction},
    ray::Ray,
    shape::Intersection,
    vector::Vector3f,
};
use rand::{thread_rng, Rng};

/// Dielectric material model.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dielectric {
    /// Index of refraction.
    ior: f32,
}

impl Dielectric {
    /// Creates dielectric material with given index of refraction.
    pub fn new(index_of_refraction: f32) -> Self {
        Dielectric {
            ior: index_of_refraction,
        }
    }
}

impl Interactable for Dielectric {
    fn interact(&self, incident_ray: Ray, intersection: Intersection) -> Option<Interaction> {
        // let mut rng = thread_rng();

        // Determine whether ray is inside or outside object, flip outward normal.
        let front_face = incident_ray.direction().dot(&intersection.normal) <= 0.0;
        let normal = if front_face {
            intersection.normal
        } else {
            -intersection.normal
        };

        // Refract at intersection normal.
        let etai_over_etat = if front_face { 1.0 / self.ior } else { self.ior };
        let incident = incident_ray.direction().normalize();
        let scattered = match incident.refract(normal, etai_over_etat) {
            Some(refracted) => refracted,
            None => incident.reflect(normal), // Total internal reflection.
        };

        // Return interaction struct.
        let interaction = Interaction {
            attenuation: Color3f::white(),
            scattered_ray: Ray::new(intersection.point, scattered),
        };
        Some(interaction)
    }
}
