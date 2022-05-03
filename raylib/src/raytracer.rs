use std::rc::Rc;

use crate::{
    camera::Camera,
    hittable::DidHit,
    material::{Dielectric, Lambertian, Metal},
    util::{random_double, random_double_in_range},
    vec3::rgba_multisampled,
    Vec3,
};

use super::{
    hittable::Hittable,
    hittable_list::HittableList,
    ray::Ray,
    sphere::Sphere,
    vec3::{dot, unit_vector, Color, Point3},
};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(r, 0.001, f64::INFINITY) {
        DidHit::Hit(rec) => {
            let (hit, attenuation, scattered) = rec.material.scatter(r, &rec);
            if hit {
                attenuation * ray_color(&scattered, world, depth - 1)
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
            // let target = rec.p + rec.normal + random_unit_vector();
            // 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1)
        }
        DidHit::Miss => {
            let unit_direction = unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
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

pub fn raytracer() -> (u32, u32, Vec<u8>) {
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let max_depth = 50;
    let samples_per_pixel = 500;

    // World
    let world = random_scene();

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);

    let cam = Camera::new(
        &look_from,
        &look_at,
        &Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    let mut image: Vec<u8> = vec![0; (image_width * image_height * 4) as usize];

    let (image_width_f, image_height_f) = (image_width as f64, image_height as f64);

    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            let (i_f, j_f) = (i as f64, j as f64);
            for _s in 0..samples_per_pixel {
                let u = (i_f + random_double()) / (image_width_f - 1.0);
                let v = (j_f + random_double()) / (image_height_f - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            set_rgba(
                &mut image,
                i,
                image_height - j - 1,
                image_width,
                rgba_multisampled(&pixel_color, samples_per_pixel),
            )
        }
        println!("Finished line {}", j);
    }
    (image_width, image_height, image)
}

fn set_rgba(image: &mut Vec<u8>, x: u32, y: u32, width: u32, rgba: (u8, u8, u8, u8)) {
    let i = (y * width + x) as usize;
    image[4 * i] = rgba.0;
    image[4 * i + 1] = rgba.1;
    image[4 * i + 2] = rgba.2;
    image[4 * i + 3] = rgba.3;
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_in_range(0.5..1.0);
                    let fuzz = random_double_in_range(&(0.0..0.5));
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }

        let material1 = Rc::new(Dielectric::new(1.5));
        world.add(Box::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            material1,
        )));

        let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
        world.add(Box::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            material2,
        )));

        let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
        world.add(Box::new(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            material3,
        )));
    }

    world
}
