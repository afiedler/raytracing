use crate::{
    rand::Rand,
    ray::Ray,
    util::degrees_to_radians,
    vec3::{cross, unit_vector, Point3},
    Vec3,
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        look_from: &Point3,
        look_at: &Point3,
        up: &Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperature: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(look_from - look_at));
        let u = unit_vector(&cross(&up, &w));
        let v = cross(&w, &u);

        let origin = *look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperature / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rand: &mut Rand) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rand);
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
