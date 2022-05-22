mod camera;
mod hittable;
mod hittable_list;
mod image;
mod material;
mod rand;
mod ray;
mod raytracer;
mod scene;
mod sphere;
mod util;
mod vec3;

pub use image::Image;
pub use rand::Rand;
pub use raytracer::{random_scene, Raytracer, RaytracerOptions};
pub use vec3::Vec3;

pub fn hello_raylib() {
    log::info!("hello from raylib");
}
