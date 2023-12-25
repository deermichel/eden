use crate::base::{
    interval::Interval,
    ray::Ray,
    shape::{Intersection, Shape},
};

/// 3-dim scene holding shape objects.
pub struct Scene {
    /// Objects in scene.
    objects: Vec<Box<dyn Shape>>, // TODO: Performance enum vs dyn.
}

impl Scene {
    /// Creates empty scene.
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    /// Adds object to scene.
    pub fn add(&mut self, object: impl Shape + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Shape for Scene {
    fn intersect(&self, ray: Ray, ray_t: Interval) -> Option<Intersection> {
        let mut intersection = None;
        let mut closest_t = ray_t.end();

        // Find closest object to ray.
        for object in self.objects.iter() {
            if let Some(i) = object.intersect(ray, Interval::new(ray_t.start(), closest_t)) {
                intersection = Some(i);
                closest_t = i.t;
            }
        }

        intersection
    }
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        base::{point::Point3f, vector::Vector3f},
        shapes::sphere::Sphere,
    };

    #[test]
    fn intersect() {
        let mut scene = Scene::new();
        let s1 = Sphere::new(Point3f::new(2.0, 0.0, 0.0), 1.0);
        let s2 = Sphere::new(Point3f::new(8.0, 0.0, 0.0), 1.0);
        let s3 = Sphere::new(Point3f::new(5.0, 0.0, 0.0), 1.0);
        scene.add(s1);
        scene.add(s2);
        scene.add(s3);

        let r1 = Ray::new(Point3f::default(), Vector3f::new(1.0, 0.0, 0.0));
        let i1 = Interval::new(0.0, 10.0);
        assert_eq!(scene.intersect(r1, i1), s1.intersect(r1, i1));
        let i2 = Interval::new(2.9, 10.0);
        assert_eq!(scene.intersect(r1, i2), s1.intersect(r1, i2));
        let i3 = Interval::new(3.5, 10.0);
        assert_eq!(scene.intersect(r1, i3), s3.intersect(r1, i3));
        let i4 = Interval::new(6.5, 10.0);
        assert_eq!(scene.intersect(r1, i4), s2.intersect(r1, i4));
        let i5 = Interval::new(f32::MIN, 1.0);
        assert_eq!(scene.intersect(r1, i5), None);

        let s4 = Sphere::new(Point3f::new(7.9, 0.0, 0.0), 1.0);
        scene.add(s4);
        assert_eq!(scene.intersect(r1, i4), s4.intersect(r1, i4));
    }
}
