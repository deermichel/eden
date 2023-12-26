use crate::base::{
    color::Color3f,
    material::{Interactable, Interaction},
    ray::Ray,
    shape::Intersection,
    vector::Vector3f,
};
use rand::{thread_rng, Rng};

/// Dielectric material model.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dielectric {
    /// Index of refraction.
    ior: f32,
}

impl Dielectric {
    /// Creates dielectric material with given index of refraction.
    pub fn new(index_of_refraction: f32) -> Self {
        Dielectric {
            ior: index_of_refraction,
        }
    }

    /// Schlick's approximation for reflectance.
    fn schlick(&self, incident: Vector3f, normal: Vector3f, eta: f32) -> f32 {
        let cos_i = (-incident.dot(&normal)).min(1.0);
        let r0 = ((1.0 - eta) / (1.0 + eta)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos_i).powi(5)
    }
}

impl Interactable for Dielectric {
    fn interact(&self, incident_ray: Ray, intersection: Intersection) -> Option<Interaction> {
        let mut rng = thread_rng();

        // Determine whether ray is inside or outside object, flip outward normal.
        let front_face = incident_ray.direction().dot(&intersection.normal) <= 0.0;
        let normal = if front_face {
            intersection.normal
        } else {
            -intersection.normal
        };

        // Refract at intersection normal.
        let etai_over_etat = if front_face { 1.0 / self.ior } else { self.ior };
        let incident = incident_ray.direction().normalize();
        let scattered = if self.schlick(incident, normal, etai_over_etat) > rng.gen() {
            incident.reflect(normal) // Schlick's approximation.
        } else {
            match incident.refract(normal, etai_over_etat) {
                Some(refracted) => refracted,
                None => incident.reflect(normal), // Total internal reflection.
            }
        };

        // Return interaction struct.
        let interaction = Interaction {
            attenuation: Color3f::white(),
            scattered_ray: Ray::new(intersection.point, scattered),
        };
        Some(interaction)
    }
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::{material::Material, point::Point3f};

    #[test]
    fn interact() {
        let d = Dielectric::new(1.5);
        let mat = Material::Dielectric(d);
        let r = Ray::new(Point3f::default(), Vector3f::new(2.0, -2.0, 0.0));
        let isect = Intersection {
            point: Point3f::new(1.0, 1.0, 1.0),
            normal: Vector3f::new(0.0, 1.0, 0.0),
            material: &mat,
            t: 1.0,
        };
        let iact = mat.interact(r, isect).unwrap();
        assert_eq!(iact.attenuation, Color3f::white());
        assert_eq!(iact.scattered_ray.origin(), isect.point);
        // Schlick's approximation is currently nondeterministic.
        let refracted = r
            .direction()
            .normalize()
            .refract(isect.normal, 1.0 / 1.5)
            .unwrap();
        let reflected = r.direction().normalize().reflect(isect.normal);
        assert!(
            iact.scattered_ray.direction() == refracted
                || iact.scattered_ray.direction() == reflected
        );
    }
}
