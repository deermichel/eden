use crate::base::{
    interval::Interval,
    ray::Ray,
    shape::{Intersectable, Intersection, Shape},
};

/// 3-dim scene holding shape objects.
pub struct Scene {
    /// Objects in scene.
    objects: Vec<Shape>,
}

impl Scene {
    /// Creates empty scene.
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    /// Adds object to scene.
    pub fn add(&mut self, object: Shape) {
        self.objects.push(object);
    }
}

impl Intersectable for Scene {
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
        base::{material::Material, point::Point3f, vector::Vector3f},
        shapes::sphere::Sphere,
    };

    #[test]
    fn intersect() {
        let mut scene = Scene::new();
        let s1 = Sphere::new(Point3f::new(2.0, 0.0, 0.0), 1.0, Material::None);
        let s2 = Sphere::new(Point3f::new(8.0, 0.0, 0.0), 1.0, Material::None);
        let s3 = Sphere::new(Point3f::new(5.0, 0.0, 0.0), 1.0, Material::None);
        scene.add(Shape::Sphere(s1));
        scene.add(Shape::Sphere(s2));
        scene.add(Shape::Sphere(s3));

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

        let s4 = Sphere::new(Point3f::new(7.9, 0.0, 0.0), 1.0, Material::None);
        scene.add(Shape::Sphere(s4));
        assert_eq!(scene.intersect(r1, i4), s4.intersect(r1, i4));
    }
}
