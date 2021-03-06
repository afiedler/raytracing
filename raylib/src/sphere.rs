use std::sync::Arc;

use crate::{hittable::DidHit, material::Material, scene::MaterialId};

use super::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{dot, Point3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material_id: MaterialId,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material_id: MaterialId) -> Self {
        Sphere {
            center,
            radius,
            material_id,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> DidHit {
        let oc = r.origin() - &self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            DidHit::Miss
        } else {
            let sqrtd = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range.
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return DidHit::Miss;
                }
            }
            let mut rec = HitRecord::default();
            rec.t = root;
            rec.p = r.at(rec.t);
            let outward_normal = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, &outward_normal);
            rec.set_material_id(self.material_id);

            return DidHit::Hit(rec);
        }
    }
}
