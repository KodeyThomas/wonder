use num_bigint::BigInt;

use super::{curve::Curve, hex::Hex};

pub trait CurveDomainProperties {
    /// Coefficient of the curve
    fn a(&self) -> BigInt;
    /// Coefficient of the curve
    fn b(&self) -> BigInt;
    /// Prime of the curve
    fn p(&self) -> BigInt;
    /// Order of the curve
    fn n(&self) -> BigInt;
    /// Cofactor of the curve
    fn h(&self) -> BigInt;
    /// Generator point x coordinate
    fn gx(&self) -> BigInt;
    /// Generator point y coordinate
    fn gy(&self) -> BigInt;
}

impl CurveDomainProperties for Curve {
    fn a(&self) -> BigInt {
        match self {
            Curve::Secp256k1 => Hex::to_bigint(&"0"),
        }
    }

    fn b(&self) -> BigInt {
        match self {
            Curve::Secp256k1 => Hex::to_bigint(&"7"),
        }
    }

    fn p(&self) -> BigInt {
        match self {
            Curve::Secp256k1 => {
                Hex::to_bigint(&"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
            }
        }
    }

    fn n(&self) -> BigInt {
        match self {
            Curve::Secp256k1 => {
                Hex::to_bigint(&"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141")
            }
        }
    }

    fn h(&self) -> BigInt {
        match self {
            Curve::Secp256k1 => Hex::to_bigint(&"1"),
        }
    }

    fn gx(&self) -> BigInt {
        match self {
            Curve::Secp256k1 => {
                Hex::to_bigint(&"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798")
            }
        }
    }

    fn gy(&self) -> BigInt {
        match self {
            Curve::Secp256k1 => {
                Hex::to_bigint(&"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8")
            }
        }
    }
}
