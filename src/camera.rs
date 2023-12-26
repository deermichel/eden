use crate::{
    base::{
        color::Color3f, interval::Interval, material::Interactable, point::Point3f, ray::Ray,
        shape::Intersectable, vector::Vector3f,
    },
    scene::Scene,
};
use rand::{thread_rng, Rng};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::{
    io::Write,
    sync::atomic::{AtomicU32, Ordering},
};

/// Perspective camera in 3-dim space.
pub struct Camera {
    /// Image width in pixels.
    image_width: u32,

    /// Image height in pixels.
    image_height: u32,

    /// Count of random samples per pixel.
    samples_per_pixel: u32,

    /// Max number of recursive ray bounces into scene.
    max_depth: u32,

    /// Vertical view angle.
    vfov: f32,

    /// Point camera is looking from.
    look_from: Point3f,

    /// Point camera is looking at.
    look_at: Point3f,

    /// Camera-relative 'up' direction.
    view_up: Vector3f,

    /// Variation angle of rays from thin lens through each pixel.
    defocus_angle: f32,

    /// Distance from 'look from' point to plane of perfect focus.
    focus_distance: f32,

    /// Location of top left pixel (0,0).
    pixel00_location: Point3f,

    /// Offset to pixel to the right.
    pixel_delta_u: Vector3f,

    /// Offset to pixel below.
    pixel_delta_v: Vector3f,

    /// Defocus disk horizontal basis.
    defocus_disk_u: Vector3f,

    /// Defocus disk vertical basis.
    defocus_disk_v: Vector3f,
}

impl Camera {
    /// Creates camera with image resolution.
    pub fn new(image_width: u32, image_height: u32) -> Self {
        assert!(image_width > 0 && image_height > 0);
        Camera {
            image_width,
            image_height,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            look_from: Point3f::new(0.0, 0.0, -1.0),
            look_at: Point3f::default(),
            view_up: Vector3f::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_distance: 1.0,
            pixel00_location: Point3f::default(),
            pixel_delta_u: Vector3f::default(),
            pixel_delta_v: Vector3f::default(),
            defocus_disk_u: Vector3f::default(),
            defocus_disk_v: Vector3f::default(),
        }
    }

    /// Renders scene.
    pub fn render(&mut self, scene: &Scene) -> Vec<Color3f> {
        self.initialize();
        let mut pixels = vec![Color3f::black(); (self.image_width * self.image_height) as usize];

        // Render loop.
        let progress = AtomicU32::new(0);
        pixels
            .par_chunks_mut(self.image_width as usize)
            .enumerate()
            .for_each(|(y, line)| {
                line.iter_mut().enumerate().for_each(|(x, pixel)| {
                    // Multi sample rendering.
                    for _ in 0..self.samples_per_pixel {
                        let ray = self.get_ray(x as u32, y as u32);
                        *pixel += self.ray_color(ray, self.max_depth, &scene);
                    }

                    // Average samples.
                    *pixel = *pixel / self.samples_per_pixel as f32;
                });

                // Progress stdout.
                let progress = progress.fetch_add(1, Ordering::Relaxed);
                print!(
                    "\r{:.2}%",
                    progress as f32 / (self.image_height - 1) as f32 * 100.0
                );
                std::io::stdout().flush().unwrap();
            });

        pixels
    }

    /// Sets samples per pixel.
    pub fn set_samples_per_pixel(&mut self, samples_per_pixel: u32) {
        self.samples_per_pixel = samples_per_pixel;
    }

    /// Sets max depth.
    pub fn set_max_depth(&mut self, max_depth: u32) {
        self.max_depth = max_depth;
    }

    /// Sets vertical field of view.
    pub fn set_vfov(&mut self, vfov: f32) {
        self.vfov = vfov;
    }

    /// Sets point camera is looking from.
    pub fn set_look_from(&mut self, look_from: Point3f) {
        self.look_from = look_from;
    }

    /// Sets point camera is looking at.
    pub fn set_look_at(&mut self, look_at: Point3f) {
        self.look_at = look_at;
    }

    /// Sets camera-relative 'up' direction.
    pub fn set_view_up(&mut self, view_up: Vector3f) {
        self.view_up = view_up;
    }

    /// Sets defocus angle.
    pub fn set_defocus_angle(&mut self, defocus_angle: f32) {
        self.defocus_angle = defocus_angle;
    }

    /// Sets focus distance.
    pub fn set_focus_distance(&mut self, focus_distance: f32) {
        self.focus_distance = focus_distance;
    }

    /// Initializes rendering vars.
    fn initialize(&mut self) {
        // Viewport dimensions.
        let aspect_ratio = (self.image_width as f32) / (self.image_height as f32);
        let h = (self.vfov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width = viewport_height * aspect_ratio;

        // Orthonormal basis u,v,w for camera coordinate system.
        let w = (self.look_from - self.look_at).normalize();
        let u = self.view_up.cross(&w).normalize();
        let v = w.cross(&u);

        // Viewport vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Pixel deltas in space.
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Pixel positions in space.
        let viewport_top_left =
            self.look_from - (self.focus_distance * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_location = viewport_top_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Defocus disk basis vectors.
        let defocus_radius = self.focus_distance * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = defocus_radius * u;
        self.defocus_disk_v = defocus_radius * v;
    }

    /// Generates ray for pixel x,y.
    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let pixel_center = self.pixel00_location
            + (x as f32 * self.pixel_delta_u)
            + (y as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.sample_pixel_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.look_from
        } else {
            self.sample_defocus_disk()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    /// Calculate color shading for ray into scene.
    fn ray_color(&self, ray: Ray, depth: u32, scene: &Scene) -> Color3f {
        // Recursion limit.
        if depth <= 0 {
            return Color3f::black();
        }

        // Intersect with scene.
        if let Some(isect) = scene.intersect(ray, Interval::new(0.001, f32::INFINITY)) {
            // Interact with material.
            if let Some(iact) = isect.material.interact(ray, isect) {
                // Recurse and attenuate.
                return iact.attenuation * self.ray_color(iact.scattered_ray, depth - 1, scene);
            } else {
                // Fully absorbed.
                return Color3f::black();
            }
        }

        // Background based on y component of ray direction.
        let normalized_direction = ray.direction().normalize();
        let a = 0.5 * (normalized_direction.y() + 1.0);
        (1.0 - a) * Color3f::white() + a * Color3f::new(0.5, 0.7, 1.0)
    }

    /// Samples random offset in pixel square.
    fn sample_pixel_square(&self) -> Vector3f {
        let mut rng = thread_rng();
        let dx = -0.5 + rng.gen::<f32>();
        let dy = -0.5 + rng.gen::<f32>();
        (dx * self.pixel_delta_u) + (dy * self.pixel_delta_v)
    }

    /// Samples random point in camera defocus disk.
    fn sample_defocus_disk(&self) -> Point3f {
        let mut rng = thread_rng();
        let mut dv = Vector3f::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if dv.length_squared() > 1.0 {
            dv = dv.normalize();
        }
        self.look_from + (dv.x() * self.defocus_disk_u) + (dv.y() * self.defocus_disk_v)
    }
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize() {
        let mut c = Camera::new(2000, 1000);
        c.set_vfov(90.0);
        c.set_look_from(Point3f::default());
        c.set_look_at(Point3f::new(1.0, 0.0, 0.0));
        c.set_defocus_angle(90.0);
        c.set_focus_distance(10.0);

        c.initialize();
        assert_eq!(c.pixel00_location, Point3f::new(10.0, 9.99, -19.99));
        assert_eq!(c.pixel_delta_u, Vector3f::new(0.0, 0.0, 0.02));
        assert_eq!(c.pixel_delta_v, Vector3f::new(0.0, -0.02, 0.0));
        assert_eq!(c.defocus_disk_u, Vector3f::new(0.0, 0.0, 10.0));
        assert_eq!(c.defocus_disk_v, Vector3f::new(0.0, 10.0, 0.0));
    }

    #[test]
    fn get_ray() {
        let mut c = Camera::new(2000, 1000);
        c.set_look_from(Point3f::new(1.0, 0.0, 0.0));
        c.initialize();

        let r = c.get_ray(10, 10);
        let pixel_center = c.pixel00_location + 10.0 * (c.pixel_delta_u + c.pixel_delta_v);
        assert_eq!(r.at(0.0), c.look_from);
        assert!((r.at(1.0) - pixel_center).length() <= c.pixel_delta_u.length());
    }
}
