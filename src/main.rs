mod base;
mod camera;
mod scene;
mod shapes;

use crate::{base::point::Point3f, camera::Camera, scene::Scene, shapes::sphere::Sphere};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

/// Entry point.
fn main() {
    // Camera.
    let image_width = 600;
    let image_height = 337;
    let mut camera = Camera::new(image_width, image_height);
    camera.set_samples_per_pixel(50);
    camera.set_max_depth(50);

    // Scene.
    let mut scene = Scene::new();
    scene.add(Sphere::new(Point3f::new(0.0, 0.0, -1.0), 0.5));
    scene.add(Sphere::new(Point3f::new(0.0, -100.5, -1.0), 100.0));

    // Render.
    let image = camera.render(&scene);

    // Open file.
    let file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(file);

    // File header.
    writeln!(&mut writer, "P3").unwrap();
    writeln!(&mut writer, "{} {}", image_width, image_height).unwrap();
    writeln!(&mut writer, "255").unwrap();

    // Write pixel values.
    for color in image {
        // Gamma correction (gamma 2).
        let r = (color.r().sqrt() * 255.0).round() as u32;
        let g = (color.g().sqrt() * 255.0).round() as u32;
        let b = (color.b().sqrt() * 255.0).round() as u32;
        writeln!(&mut writer, "{} {} {}", r, g, b).unwrap();
    }

    // Cleanup.
    println!("\rDone.       ");
}
