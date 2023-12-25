use crate::base::{interval::Interval, point::Point3f, ray::Ray, vector::Vector3f};

// TODO: For perf: trait -> Intersectable, Shape -> enum.

/// Intersectable shape in 3-dim space.
pub trait Shape {
    /// Tests for ray intersection in given t-interval. Returns intersection struct if exists.
    fn intersect(&self, ray: Ray, ray_t: Interval) -> Option<Intersection>;
}

/// Struct holding intersection properties.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection {
    /// Intersection point.
    pub point: Point3f,

    /// Normal at intersection.
    pub normal: Vector3f,

    /// Value of ray parameter t.
    pub t: f32,
}
