use crate::{
    base::{color::Color3f, ray::Ray, shape::Intersection},
    materials::{dielectric::Dielectric, lambert::Lambert, metal::Metal},
};

/// A material defines how an object interacts with light rays.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Dielectric(Dielectric),
    Lambert(Lambert),
    Metal(Metal),
    None,
}

impl Interactable for Material {
    fn interact(&self, incident_ray: Ray, intersection: Intersection) -> Option<Interaction> {
        match self {
            Material::Dielectric(d) => d.interact(incident_ray, intersection),
            Material::Lambert(l) => l.interact(incident_ray, intersection),
            Material::Metal(m) => m.interact(incident_ray, intersection),
            Material::None => None,
        }
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
