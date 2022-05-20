mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod raytracer;
mod sphere;
mod util;
mod vec3;

pub use raytracer::{random_scene, raytracer};
pub use vec3::Vec3;

pub fn hello_raylib() {
    log::info!("hello from raylib");
}
