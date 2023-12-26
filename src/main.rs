mod base;
mod camera;
mod materials;
mod scene;
mod shapes;

use crate::{
    base::{
        color::{Color, Color3f},
        material::Material,
        point::Point3f,
    },
    camera::Camera,
    materials::{dielectric::Dielectric, lambert::Lambert, metal::Metal},
    scene::Scene,
    shapes::sphere::Sphere,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

/// Entry point.
fn main() {
    // Camera.
    let image_width = 400;
    let image_height = 225;
    let mut camera = Camera::new(image_width, image_height);
    camera.set_samples_per_pixel(100);
    camera.set_max_depth(50);

    // Materials.
    let material_ground = Lambert::new(Color3f::new(0.8, 0.8, 0.0));
    let material_center = Lambert::new(Color3f::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color3f::new(0.8, 0.6, 0.2), 0.0);

    // Scene.
    let mut scene = Scene::new();
    scene.add(Sphere::new(
        Point3f::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambert(material_ground),
    ));
    scene.add(Sphere::new(
        Point3f::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambert(material_center),
    ));
    scene.add(Sphere::new(
        Point3f::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Dielectric(material_left),
    ));
    scene.add(Sphere::new(
        Point3f::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal(material_right),
    ));

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
