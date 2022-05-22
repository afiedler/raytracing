pub struct Rand {
    rng: oorandom::Rand64,
}

impl Rand {
    pub fn new() -> Self {
        Rand {
            rng: oorandom::Rand64::new(0xda942042e4dd58b5),
        }
    }
    pub fn new_with_seed(seed: u128) -> Self {
        Rand {
            rng: oorandom::Rand64::new(seed),
        }
    }

    pub fn random_double(&mut self) -> f64 {
        self.rng.rand_float()
    }
}
