use std::sync::Arc;

use crate::{
    material::{Lambertian, Material},
    scene::MaterialId,
    vec3::Color,
};

use super::{
    ray::Ray,
    vec3::{dot, Point3},
    Vec3,
};

pub enum DidHit {
    Hit(HitRecord),
    Miss,
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material_id: MaterialId,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }

    pub fn set_material_id(&mut self, material_id: MaterialId) {
        self.material_id = material_id
    }

    pub fn material_id(&self) -> MaterialId {
        self.material_id
    }

    pub fn default() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material_id: 0,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> DidHit;
}
