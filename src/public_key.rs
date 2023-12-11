use crate::{
    ecc::{Curve, CurveDomainProperties},
    private_key::ExtendedPrivateKey,
};

pub struct ExtendedPublicKey {
    pub public_key: [u8; 32],
    pub chain_code: [u8; 32],
    pub curve: Curve,
}

impl ExtendedPublicKey {
    pub fn from_extended_private(extended_private_key: &ExtendedPrivateKey) -> ExtendedPublicKey {
        todo!()
    }

    fn calculate_public_key(private_key: &[u8; 32], curve: Curve) -> [u8; 64] {
        todo!()
    }
}
