use crate::{crypto::hmac512, ecc::Curve, seed::Seed};

pub struct ExtendedPrivateKey {
    pub private_key: [u8; 32],
    pub chain_code: [u8; 32],
    pub curve: Curve,
}

impl ExtendedPrivateKey {
    pub fn from_seed(seed: Seed, curve: Curve) -> ExtendedPrivateKey {
        let hmac = hmac512(seed.as_ref(), b"Bitcoin seed");
        drop(seed);

        let (private_key, chain_code) = hmac.split_at(hmac.len() / 2);

        ExtendedPrivateKey {
            private_key: private_key.try_into().unwrap(),
            chain_code: chain_code.try_into().unwrap(),
            curve,
        }
    }

    pub fn derive_child(&self, index: u32) -> ExtendedPrivateKey {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_seed_private_key_and_chain_code_should_be_32_bytes_each() {
        let seed: Seed = Seed::from_random_bytes();
        let extended_private_key: ExtendedPrivateKey =
            ExtendedPrivateKey::from_seed(seed, Curve::Secp256k1);

        assert_eq!(extended_private_key.private_key.len(), 32);
        assert_eq!(extended_private_key.chain_code.len(), 32);
    }
}
