use std::sync::{Arc, Mutex};

use image::{Rgba, RgbaImage};
use raylib::{random_scene, Image, Raytracer, RaytracerOptions};

use rayon::prelude::*;

fn main() {
    println!("Raytracing...");
    let width = 1200;
    let height = 800;
    let aspect_ratio = width as f64 / height as f64;
    let world = Arc::new(random_scene());
    let image_mutex = Arc::new(Mutex::new(Image::new(width, height)));
    let raytracer = Arc::new(Raytracer::new(
        world,
        &RaytracerOptions {
            image_width: width,
            aspect_ratio,
            max_depth: 50,
            samples_per_pixel: 5,
        },
    ));

    (0..height).into_par_iter().for_each(|line_number| {
        let line = raytracer.trace_line(line_number);
        let mut image = image_mutex.lock().unwrap();
        image.set_line(line_number, line);
    });

    let mut image = RgbaImage::new(width, height);
    let ray_img = image_mutex.lock().unwrap();

    for j in 0..height {
        for i in 0..width {
            image.put_pixel(i, height - j - 1, Rgba(ray_img.get_pixel(i, j)));
        }
    }

    image.save_with_format("./output-draft.png", image::ImageFormat::Png);
}
