use crate::{
    base::{color::Color3f, ray::Ray, shape::Intersection},
    materials::lambert::Lambert,
};

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
