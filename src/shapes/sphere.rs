use crate::base::{
    interval::Interval,
    material::Material,
    point::Point3f,
    ray::Ray,
    shape::{Intersection, Shape},
};

/// Sphere in 3-dim space defined by center position and radius.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    /// Center position.
    center: Point3f,

    /// Sphere radius.
    radius: f32,

    /// Surface material.
    material: Material,
}

impl Sphere {
    /// Creates sphere with center position and radius.
    pub fn new(center: Point3f, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: Ray, ray_t: Interval) -> Option<Intersection> {
        // Solve quadratic equation.
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None; // No intersection.
        }
        let discriminant_sqrt = discriminant.sqrt();

        // Find nearest root that lies in specified interval.
        let mut root = (-half_b - discriminant_sqrt) / a;
        if !ray_t.contains(root) {
            root = (-half_b + discriminant_sqrt) / a;
            if !ray_t.contains(root) {
                return None; // Outside interval.
            }
        }
        let point = ray.at(root);

        // Calculate normal.
        let normal = (point - self.center) / self.radius;

        // Return intersection struct.
        let intersection = Intersection {
            point,
            material: &self.material,
            normal,
            t: root,
        };
        Some(intersection)
    }
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::vector::Vector3f;

    #[test]
    fn intersect() {
        let s = Sphere::new(Point3f::new(0.0, 3.0, 0.0), 2.0, Material::None);

        // Outside ray.
        let r1 = Ray::new(Point3f::default(), Vector3f::new(0.0, 1.0, 0.0));
        let i1 = Intersection {
            point: Point3f::new(0.0, 1.0, 0.0),
            material: &Material::None,
            normal: Vector3f::new(0.0, -1.0, 0.0),
            t: 1.0,
        };
        let i2 = Intersection {
            point: Point3f::new(0.0, 5.0, 0.0),
            material: &Material::None,
            normal: Vector3f::new(0.0, 1.0, 0.0),
            t: 5.0,
        };
        assert_eq!(s.intersect(r1, Interval::new(0.0, f32::INFINITY)), Some(i1));
        assert_eq!(s.intersect(r1, Interval::new(1.0, f32::INFINITY)), Some(i2));
        assert_eq!(s.intersect(r1, Interval::new(2.0, f32::INFINITY)), Some(i2));
        assert_eq!(s.intersect(r1, Interval::new(0.0, 1.0)), None);
        assert_eq!(s.intersect(r1, Interval::new(5.0, f32::INFINITY)), None);

        // Inside ray.
        let r2 = Ray::new(s.center, Vector3f::new(0.0, 1.0, 0.0));
        let i3 = Intersection {
            point: Point3f::new(0.0, 5.0, 0.0),
            material: &Material::None,
            normal: Vector3f::new(0.0, 1.0, 0.0),
            t: 2.0,
        };
        let i4 = Intersection {
            point: Point3f::new(0.0, 1.0, 0.0),
            material: &Material::None,
            normal: Vector3f::new(0.0, -1.0, 0.0),
            t: -2.0,
        };
        assert_eq!(s.intersect(r2, Interval::new(0.0, f32::INFINITY)), Some(i3));
        assert_eq!(
            s.intersect(r2, Interval::new(f32::NEG_INFINITY, f32::INFINITY)),
            Some(i4)
        );
        assert_eq!(s.intersect(r2, Interval::new(-2.0, 2.0)), None);

        // Parallel ray.
        let r3 = Ray::new(Point3f::default(), Vector3f::new(1.0, 0.0, 1.0));
        assert_eq!(
            s.intersect(r3, Interval::new(f32::NEG_INFINITY, f32::INFINITY)),
            None
        );
    }
}
