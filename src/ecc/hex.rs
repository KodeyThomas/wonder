use num_bigint::BigInt;

pub struct Hex {
    pub bytes: Vec<u8>,
}

impl Hex {
    pub fn to_bigint(hex_string: &str) -> BigInt {
        BigInt::parse_bytes(hex_string.as_bytes(), 16).unwrap()
    }
}
