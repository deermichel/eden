use crate::{
    base::{interval::Interval, material::Material, point::Point3f, ray::Ray, vector::Vector3f},
    shapes::sphere::Sphere,
};

/// An intersectable shape in 3-dim space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
}

impl Intersectable for Shape {
    fn intersect(&self, ray: Ray, ray_t: Interval) -> Option<Intersection> {
        match self {
            Shape::Sphere(s) => s.intersect(ray, ray_t),
        }
    }
}

/// An intersectable object can be intersected by rays.
pub trait Intersectable {
    /// Tests for ray intersection in given t-interval. Returns intersection struct if exists.
    fn intersect(&self, ray: Ray, ray_t: Interval) -> Option<Intersection>;
}

/// Struct holding intersection properties.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    /// Intersection point.
    pub point: Point3f,

    /// Normal at intersection.
    pub normal: Vector3f,

    /// Material at intersection.
    pub material: &'a Material,

    /// Value of ray parameter t.
    pub t: f32,
}
