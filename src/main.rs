mod base;
mod camera;
mod materials;
mod scene;
mod shapes;

use crate::{
    base::{color::Color3f, material::Material, point::Point3f, vector::Vector3f},
    camera::Camera,
    materials::{dielectric::Dielectric, lambert::Lambert, metal::Metal},
    scene::Scene,
    shapes::sphere::Sphere,
};
use rand::{thread_rng, Rng};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

/// Entry point.
fn main() {
    // Camera.
    let image_width = 1200;
    let image_height = 675;
    let mut camera = Camera::new(image_width, image_height);
    camera.set_samples_per_pixel(500);
    camera.set_max_depth(50);

    camera.set_vfov(20.0);
    camera.set_look_from(Point3f::new(13.0, 2.0, 3.0));
    camera.set_look_at(Point3f::new(0.0, 0.0, 0.0));
    camera.set_view_up(Vector3f::new(0.0, 1.0, 0.0));

    camera.set_defocus_angle(0.6);
    camera.set_focus_distance(10.0);

    // Scene.
    let mut scene = Scene::new();

    let ground_material = Lambert::new(Color3f::new(0.5, 0.5, 0.5));
    scene.add(Sphere::new(
        Point3f::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambert(ground_material),
    ));

    let rnd_f32 = || thread_rng().gen::<f32>();
    let rnd_color = || {
        let mut rng = thread_rng();
        Color3f::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        )
    };
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rnd_f32();
            let center = Point3f::new(a as f32 + 0.9 * rnd_f32(), 0.2, b as f32 + 0.9 * rnd_f32());

            if (center - Point3f::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse.
                    let albedo = rnd_color() * rnd_color();
                    let l = Lambert::new(albedo);
                    scene.add(Sphere::new(center, 0.2, Material::Lambert(l)));
                } else if choose_mat < 0.95 {
                    // Metal.
                    let albedo = (rnd_color() / 4.0) + 0.75;
                    let fuzz = rnd_f32() * 0.5;
                    let m = Metal::new(albedo, fuzz);
                    scene.add(Sphere::new(center, 0.2, Material::Metal(m)));
                } else {
                    // Glass.
                    let d = Dielectric::new(1.5);
                    scene.add(Sphere::new(center, 0.2, Material::Dielectric(d)));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    scene.add(Sphere::new(
        Point3f::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(material1),
    ));

    let material2 = Lambert::new(Color3f::new(0.4, 0.2, 0.1));
    scene.add(Sphere::new(
        Point3f::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambert(material2),
    ));

    let material3 = Metal::new(Color3f::new(0.7, 0.6, 0.5), 0.0);
    scene.add(Sphere::new(
        Point3f::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(material3),
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
