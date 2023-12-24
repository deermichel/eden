use crate::geometry::{point::Point3f, vector::Vector3f};

/// Ray in 3-dim space defined by origin and direction.
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    /// Initial point.
    origin: Point3f,

    /// Direction vector.
    direction: Vector3f,
}

impl Ray {
    /// Creates ray defined by origin and direction.
    pub fn new(origin: Point3f, direction: Vector3f) -> Self {
        Ray { origin, direction }
    }

    /// Position on ray for parameter t.
    pub fn at(&self, t: f32) -> Point3f {
        self.origin + t * self.direction
    }
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let o = Point3f::new(1.0, 2.0, 3.0);
        let d = Vector3f::new(2.0, 3.0, 4.0);
        let r = Ray::new(o, d);
        assert_eq!(r.at(0.0), o);
        assert_eq!(r.at(1.0), o + d);
        assert_eq!(r.at(-2.0), Point3f::new(-3.0, -4.0, -5.0));
    }
}
