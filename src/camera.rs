use crate::{
    base::{
        color::Color3f, interval::Interval, point::Point3f, ray::Ray, shape::Shape,
        vector::Vector3f,
    },
    scene::Scene,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::io::Write;

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

    /// Camera position in scene.
    position: Point3f,

    /// Location of top left pixel (0,0).
    pixel00_location: Point3f,

    /// Offset to pixel to the right.
    pixel_delta_u: Vector3f,

    /// Offset to pixel below.
    pixel_delta_v: Vector3f,

    /// Random number generator.
    rng: ThreadRng,
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
            position: Point3f::default(),
            pixel00_location: Point3f::default(),
            pixel_delta_u: Vector3f::default(),
            pixel_delta_v: Vector3f::default(),
            rng: thread_rng(),
        }
    }

    /// Renders scene.
    pub fn render(&mut self, scene: &Scene) -> Vec<Color3f> {
        self.initialize();
        let mut pixels = Vec::with_capacity((self.image_width * self.image_height) as usize);

        // Render loop.
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let mut color = Color3f::black();

                // Multi sample rendering.
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color += self.ray_color(ray, self.max_depth, &scene);
                }

                // Average samples.
                pixels.push(color / self.samples_per_pixel as f32);
            }

            // Progress stdout.
            print!(
                "\r{:.2}%",
                y as f32 / (self.image_height - 1) as f32 * 100.0
            );
            std::io::stdout().flush().unwrap();
        }

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

    /// Initializes rendering vars.
    fn initialize(&mut self) {
        // Viewport.
        let aspect_ratio = (self.image_width as f32) / (self.image_height as f32);
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let viewport_u = Vector3f::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3f::new(0.0, -viewport_height, 0.0);

        // Pixel deltas in space.
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Pixel positions in space.
        let viewport_top_left = self.position
            - Vector3f::new(0.0, 0.0, focal_length)
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);
        self.pixel00_location = viewport_top_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    /// Generates ray for pixel x,y.
    fn get_ray(&mut self, x: u32, y: u32) -> Ray {
        let pixel_center = self.pixel00_location
            + (x as f32 * self.pixel_delta_u)
            + (y as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.position;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    /// Calculate color shading for ray into scene.
    fn ray_color(&mut self, ray: Ray, depth: u32, scene: &Scene) -> Color3f {
        // Recursion limit.
        if depth <= 0 {
            return Color3f::black();
        }

        // Intersect with scene.
        if let Some(i) = scene.intersect(ray, Interval::new(0.001, f32::INFINITY)) {
            // Lambertian distribution.
            let random = Vector3f::new(
                self.rng.gen_range(-1.0..1.0),
                self.rng.gen_range(-1.0..1.0),
                self.rng.gen_range(-1.0..1.0),
            );
            let new_direction = i.normal + random;

            // Recurse and attenuate.
            let new_ray = Ray::new(i.point, new_direction);
            return 0.5 * self.ray_color(new_ray, depth - 1, scene);
        }

        // Background based on y component of ray direction.
        let normalized_direction = ray.direction().normalize();
        let a = 0.5 * (normalized_direction.y() + 1.0);
        (1.0 - a) * Color3f::white() + a * Color3f::new(0.5, 0.7, 1.0)
    }

    /// Samples random offset in pixel square.
    fn pixel_sample_square(&mut self) -> Vector3f {
        let dx = -0.5 + self.rng.gen::<f32>();
        let dy = -0.5 + self.rng.gen::<f32>();
        (dx * self.pixel_delta_u) + (dy * self.pixel_delta_v)
    }
}
