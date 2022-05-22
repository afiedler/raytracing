use std::ops::{self, Range};

use overload::overload;

use crate::{
    rand::Rand,
    util::{clamp, random_double_in_range},
};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }
    pub fn random(rand: &mut Rand) -> Self {
        Self::new(
            rand.random_double(),
            rand.random_double(),
            rand.random_double(),
        )
    }
    pub fn random_in_range(range: Range<f64>, rand: &mut Rand) -> Self {
        Self::new(
            random_double_in_range(&range, rand),
            random_double_in_range(&range, rand),
            random_double_in_range(&range, rand),
        )
    }
    pub fn random_in_unit_sphere(rand: &mut Rand) -> Self {
        let mut p = Self::random_in_range(-1.0..1.0, rand);
        while p.length_squared() >= 1.0 {
            p = Self::random_in_range(-1.0..1.0, rand);
        }
        p
    }
    pub fn random_in_unit_disk(rand: &mut Rand) -> Self {
        let mut p = Vec3::new(
            random_double_in_range(&(-1.0..1.0), rand),
            random_double_in_range(&(-1.0..1.0), rand),
            0.0,
        );
        while p.length_squared() >= 1.0 {
            p = Vec3::new(
                random_double_in_range(&(-1.0..1.0), rand),
                random_double_in_range(&(-1.0..1.0), rand),
                0.0,
            );
        }
        p
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        );
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs);
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs);
    }
}

// Unary negate
overload!(- (a: ?Vec3) -> Vec3 { Vec3 { e: [-a.e[0], -a.e[1], -a.e[2]] } });

// Add
overload!((a: ?Vec3) + (b: ?Vec3) -> Vec3 {
    Vec3 {
        e: [
            a.e[0] + b.e[0],
            a.e[1] + b.e[1],
            a.e[2] + b.e[2]
        ]
    }
});

// Subtract
overload!((a: ?Vec3) - (b: ?Vec3) -> Vec3 {
    Vec3 {
        e: [
            a.e[0] - b.e[0],
            a.e[1] - b.e[1],
            a.e[2] - b.e[2]
        ]
    }
});

// Multiply
overload!((a: ?Vec3) * (b: ?Vec3) -> Vec3 {
    Vec3 {
        e: [
            a.e[0] * b.e[0],
            a.e[1] * b.e[1],
            a.e[2] * b.e[2]
        ]
    }
});
overload!((a: f64) * (b: ?Vec3) -> Vec3 {
    Vec3 {
        e: [
            a * b.e[0],
            a * b.e[1],
            a * b.e[2]
        ]
    }
});
overload!((a: ?Vec3) * (b: f64) -> Vec3 {
    Vec3 {
        e: [
            a.e[0] * b,
            a.e[1] * b,
            a.e[2] * b
        ]
    }
});

// Divide
overload!((a: ?Vec3) / (b: f64) -> Vec3 {
    Vec3 {
        e: [
            a.e[0] / b,
            a.e[1] / b,
            a.e[2] / b
        ]
    }
});

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn random_unit_vector(rand: &mut Rand) -> Vec3 {
    unit_vector(&Vec3::random_in_unit_sphere(rand))
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn rgba(color: &Color) -> (u8, u8, u8, u8) {
    (
        (255.999 * color.x()) as u8,
        (255.999 * color.y()) as u8,
        (255.999 * color.z()) as u8,
        255,
    )
}

pub fn rgba_multisampled(color: &Color, samples_per_pixel: u32) -> (u8, u8, u8, u8) {
    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (color.x() * scale).sqrt();
    let g = (color.y() * scale).sqrt();
    let b = (color.z() * scale).sqrt();

    (
        (256.0 * clamp(r, 0.0..0.999)) as u8,
        (256.0 * clamp(g, 0.0..0.999)) as u8,
        (256.0 * clamp(b, 0.0..0.999)) as u8,
        255,
    )
}
