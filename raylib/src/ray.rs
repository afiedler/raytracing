use super::{vec3::Point3, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
}
