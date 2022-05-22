use crate::{
    camera::Camera,
    hittable::DidHit,
    material::{Dielectric, Lambertian, Metal},
    rand::Rand,
    scene::Scene,
    util::random_double_in_range,
    vec3::rgba_multisampled,
    Vec3,
};

use super::{
    hittable::Hittable,
    ray::Ray,
    sphere::Sphere,
    vec3::{dot, unit_vector, Color, Point3},
};

fn ray_color(r: &Ray, scene: &Scene, depth: i32, rand: &mut Rand) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match scene.hit(r, 0.001, f64::INFINITY) {
        DidHit::Hit(rec) => {
            let material = scene.get_material(rec.material_id());
            let (hit, attenuation, scattered) = material.scatter(r, &rec, rand);
            if hit {
                attenuation * ray_color(&scattered, scene, depth - 1, rand)
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

#[derive(Debug, Clone, Copy)]
pub struct RaytracerOptions {
    pub image_width: u32,
    pub aspect_ratio: f64,
    pub max_depth: u8,
    pub samples_per_pixel: u32,
}

impl Default for RaytracerOptions {
    fn default() -> Self {
        RaytracerOptions {
            image_width: 1200,
            aspect_ratio: 3.0 / 2.0,
            max_depth: 50,
            samples_per_pixel: 500,
        }
    }
}

pub struct Raytracer {
    scene: Scene,
    camera: Camera,
    options: RaytracerOptions,
    image_height: u32,
}

impl Raytracer {
    pub fn new<'a>(scene: Scene, options: &RaytracerOptions) -> Raytracer {
        let aspect_ratio = options.aspect_ratio;
        let image_width = options.image_width;
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        // World

        let look_from = Point3::new(13.0, 2.0, 3.0);
        let look_at = Point3::new(0.0, 0.0, 0.0);

        let camera = Camera::new(
            &look_from,
            &look_at,
            &Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.1,
            10.0,
        );

        Raytracer {
            scene,
            camera,
            options: options.clone(),
            image_height,
        }
    }

    pub fn trace_line(&self, y: u32, rand: &mut Rand) -> Vec<u8> {
        let (image_width_f, image_height_f) =
            (self.options.image_width as f64, self.image_height as f64);

        let mut line = vec![0; (self.options.image_width as usize * 4)];
        let camera = &self.camera;

        for i in 0..(self.options.image_width as usize) {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            let (i_f, j_f) = (i as f64, y as f64);
            for _s in 0..self.options.samples_per_pixel {
                let u = (i_f + rand.random_double()) / (image_width_f - 1.0);
                let v = (j_f + rand.random_double()) / (image_height_f - 1.0);
                let r = camera.get_ray(u, v, rand);

                pixel_color += ray_color(&r, &self.scene, self.options.max_depth as i32, rand);
            }

            let rgba = rgba_multisampled(&pixel_color, self.options.samples_per_pixel);
            line[4 * i] = rgba.0;
            line[4 * i + 1] = rgba.1;
            line[4 * i + 2] = rgba.2;
            line[4 * i + 3] = rgba.3;
        }
        println!("Finished line {}", y);
        line
    }
}

pub fn random_scene(rand: &mut Rand) -> Scene {
    let mut scene = Scene::new();

    let ground_material_id =
        scene.add_material(Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))));

    scene.add_object(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material_id,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand.random_double();
            let center = Point3::new(
                a as f64 + 0.9 * rand.random_double(),
                0.2,
                b as f64 + 0.9 * rand.random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(rand) * Color::random(rand);
                    let sphere_material_id = scene.add_material(Box::new(Lambertian::new(albedo)));
                    scene.add_object(Box::new(Sphere::new(center, 0.2, sphere_material_id)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_in_range(0.5..1.0, rand);
                    let fuzz = random_double_in_range(&(0.0..0.5), rand);
                    let sphere_material_id = scene.add_material(Box::new(Metal::new(albedo, fuzz)));
                    scene.add_object(Box::new(Sphere::new(center, 0.2, sphere_material_id)));
                } else {
                    // glass
                    let sphere_material_id = scene.add_material(Box::new(Dielectric::new(1.5)));
                    scene.add_object(Box::new(Sphere::new(center, 0.2, sphere_material_id)));
                }
            }
        }

        let material1 = scene.add_material(Box::new(Dielectric::new(1.5)));
        scene.add_object(Box::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            material1,
        )));

        let material2 = scene.add_material(Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))));
        scene.add_object(Box::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            material2,
        )));

        let material3 = scene.add_material(Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));
        scene.add_object(Box::new(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            material3,
        )));
    }

    scene
}
