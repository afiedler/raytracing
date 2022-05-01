use crate::{
    camera::Camera,
    util::random_double,
    vec3::{random_unit_vector, rgba_multisampled},
};

use super::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
    sphere::Sphere,
    vec3::{dot, rgba, unit_vector, Color, Point3},
    Vec3,
};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + random_unit_vector();
        0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1)
    } else {
        let unit_direction = unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = dot(&oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

pub fn raytracer() -> Vec<u8> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let max_depth = 50;
    let samples_per_pixel = 100;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let cam = Camera::default();

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //let mut image: Vec<u8> = Vec::with_capacity((image_width * image_height * 4) as usize);
    let mut image: Vec<u8> = vec![0; (image_width * image_height * 4) as usize];

    let (image_width_f, image_height_f) = (image_width as f64, image_height as f64);

    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            let (i_f, j_f) = (i as f64, j as f64);
            for s in 0..samples_per_pixel {
                let u = (i_f + random_double()) / (image_width_f - 1.0);
                let v = (j_f + random_double()) / (image_height_f - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            set_rgba(
                &mut image,
                image_width - i - 1,
                image_height - j - 1,
                image_width,
                rgba_multisampled(&pixel_color, samples_per_pixel),
            )
        }
    }
    image
}

fn set_rgba(image: &mut Vec<u8>, x: u32, y: u32, width: u32, rgba: (u8, u8, u8, u8)) {
    let i = (y * width + x) as usize;
    image[4 * i] = rgba.0;
    image[4 * i + 1] = rgba.1;
    image[4 * i + 2] = rgba.2;
    image[4 * i + 3] = rgba.3;
}
