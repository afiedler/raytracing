use std::{ops::Range, sync::Mutex};

pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees / PI * 180.0
}

pub fn random_double_in_range(range: &Range<f64>) -> f64 {
    range.start + (range.end - range.start) * random_double()
}

pub fn clamp(x: f64, range: Range<f64>) -> f64 {
    if x < range.start {
        range.start
    } else if x > range.end {
        range.end
    } else {
        x
    }
}

// Faster random numbers in WebAssembly
// Thank you https://clayto.com/2021/07/shaking-off-the-rust-2-ray-tracing-in-webassembly/
lazy_static::lazy_static! {
    static ref RNG: Mutex<u64> = Mutex::new(0xda942042e4dd58b5);
}

pub fn random_double() -> f64 {
    let mut num: u64;
    {
        // scope controls when the RNG mutex is released
        let mut rng = RNG.lock().unwrap();

        *rng = rng.wrapping_mul(0xda942042e4dd58b5u64);
        num = *rng;
    }

    num >>= 32;

    let num = (num as f32) / 2f32.powi(32);

    f64::from(num)
}
