use crate::{
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
}

impl Camera {
    pub fn default() -> Camera {
        Self::new(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(0.0, 0.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
            75.0,
            16.0 / 9.0,
        )
    }

    pub fn new(
        look_from: &Point3,
        look_at: &Point3,
        up: &Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(look_from - look_at));
        let u = unit_vector(&cross(&up, &w));
        let v = cross(&w, &u);

        let origin = *look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
