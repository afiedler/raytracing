use crate::{
    hittable::HitRecord,
    rand::Rand,
    ray::Ray,
    util::clamp,
    vec3::{dot, random_unit_vector, reflect, refract, unit_vector, Color},
    Vec3,
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, rand: &mut Rand) -> (bool, Color, Ray);
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rand: &mut Rand) -> (bool, Color, Ray) {
        let mut scatter_direction = rec.normal + random_unit_vector(rand);

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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: clamp(fuzz, f64::NEG_INFINITY..1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rand: &mut Rand) -> (bool, Color, Ray) {
        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(rand),
        );
        let attenuation = self.albedo;
        (
            dot(scattered.direction(), &rec.normal) > 0.0,
            attenuation,
            scattered,
        )
    }
}

pub struct Dielectric {
    /// Index of refraction
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rand: &mut Rand) -> (bool, Color, Ray) {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand.random_double() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        let scattered = Ray::new(rec.p, direction);

        (true, attenuation, scattered)
    }
}
