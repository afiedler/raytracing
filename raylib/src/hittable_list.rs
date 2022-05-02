use crate::hittable::DidHit;

use super::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> DidHit {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            match obj.hit(r, t_min, closest_so_far) {
                DidHit::Hit(rec) => {
                    hit_anything = true;
                    temp_rec = rec;
                    closest_so_far = temp_rec.t;
                }
                _ => {}
            }
        }

        if hit_anything {
            DidHit::Hit(temp_rec)
        } else {
            DidHit::Miss
        }
    }
}
