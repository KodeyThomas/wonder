use rand::{rngs::ThreadRng, Rng};

pub struct Seed {
    seed: [u8; 64],
}

impl Seed {
    /// Generates a random 64-byte seed.
    ///
    /// Do NOT use this function except for testing, as it is not cryptographically secure.
    pub fn from_random_bytes() -> Seed {
        let mut seed: Vec<u8> = vec![0u8; 64];
        let mut rng: ThreadRng = rand::thread_rng();

        rng.try_fill(seed.as_mut_slice()).unwrap();
        Seed {
            seed: seed.try_into().unwrap(),
        }
    }

    /// Returns a reference to the seed
    pub fn as_ref(&self) -> &[u8; 64] {
        &self.seed
    }
}
