use crate::base::{color::Color3f, ray::Ray, shape::Intersection, vector::Vector3f};
use rand::thread_rng;

/// A material defines how an object interacts with light rays.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Lambert(Lambert),
    None,
}

impl Interactable for Material {
    fn interact(&self, incident_ray: Ray, intersection: Intersection) -> Option<Interaction> {
        match self {
            Material::Lambert(l) => l.interact(incident_ray, intersection),
            Material::None => None,
        }
    }
}

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
    fn interact(&self, incident_ray: Ray, intersection: Intersection) -> Option<Interaction> {
        let mut rng = thread_rng();

        // Lambertian distribution.
        let mut scatter_direction = intersection.normal + Vector3f::random_unit_vector(&mut rng);

        // Catch degenerate scatter direction.
        if scatter_direction.near_zero() {
            scatter_direction = intersection.normal;
        }

        // Return interaction struct.
        let interaction = Interaction {
            attenuation: self.albedo,
            scattered_ray: Ray::new(intersection.point, scatter_direction),
        };
        Some(interaction)
    }
}

/// An interactable object can interact with light rays.
pub trait Interactable {
    /// Evaluates interactable at a given intersection point. Returns interaction struct if not absorbed.
    fn interact(&self, incident_ray: Ray, intersection: Intersection) -> Option<Interaction>;
}

/// Struct holding interaction properties.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Interaction {
    /// Color attenuation to be applied to scattered ray.
    pub attenuation: Color3f,

    /// Scattered ray.
    pub scattered_ray: Ray,
}
