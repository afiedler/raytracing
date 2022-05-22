use crate::{
    hittable::{DidHit, HitRecord, Hittable},
    hittable_list::HittableList,
    material::Material,
    ray::Ray,
};

pub struct Scene {
    materials: Vec<Box<dyn Material + Sync + Send>>,
    objects: Vec<Box<dyn Hittable + Sync + Send>>,
}
pub type MaterialId = i32;

impl Scene {
    pub fn new() -> Self {
        Scene {
            materials: vec![],
            objects: vec![],
        }
    }

    pub fn add_material(&mut self, material: Box<dyn Material + Sync + Send>) -> MaterialId {
        self.materials.push(material);
        TryInto::<i32>::try_into(self.materials.len()).unwrap() - 1
    }

    pub fn add_object(&mut self, object: Box<dyn Hittable + Sync + Send>) {
        self.objects.push(object)
    }

    pub fn get_material<'a>(
        &'a self,
        material_id: MaterialId,
    ) -> &'a Box<dyn Material + Send + Sync> {
        let material_id = TryInto::<usize>::try_into(material_id).unwrap();
        self.materials.get(material_id).unwrap()
    }
}

impl Hittable for Scene {
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
