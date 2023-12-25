use crate::base::{color::Color3f, ray::Ray, shape::Intersection};

/// A material defines how an object interacts with light rays.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Lambertian(Lambertian),
    None,
}

impl Interactable for Material {
    fn interact(&self, incident_ray: Ray, intersection: Intersection) -> Option<Interaction> {
        match self {
            Material::Lambertian(l) => l.interact(incident_ray, intersection),
            Material::None => None,
        }
    }
}

/// Lambertian material model.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Lambertian {
    /// Fraction of light that the object reflects.
    albedo: Color3f,
}

impl Lambertian {
    /// Creates lambertian material with given albedo.
    pub fn new(albedo: Color3f) -> Self {
        Lambertian { albedo }
    }
}

impl Interactable for Lambertian {
    fn interact(&self, incident_ray: Ray, intersection: Intersection) -> Option<Interaction> {
        None
    }
}

/// An interactable object can interact with light rays.
trait Interactable {
    /// Evaluates interactable at a given intersection point. Returns interaction struct if not absorbed.
    fn interact(&self, incident_ray: Ray, intersection: Intersection) -> Option<Interaction>;
}

/// Struct holding interaction properties.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Interaction {
    /// Color attenuation to be applied to scattered ray.
    attenuation: Color3f,

    /// Scattered ray.
    scattered_ray: Ray,
}
