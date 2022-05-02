use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{dot, random_unit_vector, reflect, unit_vector, Color},
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> (bool, Color, Ray);
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        (true, attenuation, scattered)
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        (
            dot(scattered.direction(), &rec.normal) > 0.0,
            attenuation,
            scattered,
        )
    }
}
