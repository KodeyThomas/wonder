use super::{Curve, CurveDomainProperties};
use num_bigint::BigInt;
use num_traits::ops::euclid::Euclid;

#[derive(Clone, Debug)]
pub struct Point {
    /// The x coordinate of the EC point
    pub x: BigInt,
    /// The y coordinate of the EC point
    pub y: BigInt,
    /// The curve that the point is on
    pub curve: Curve,
}

impl Point {
    pub fn identity(curve: Curve) -> Point {
        Point {
            x: BigInt::from(0),
            y: BigInt::from(0),
            curve,
        }
    }

    /// Multiplies a point on the curve by a scalar value using the double-and-add algorithm
    pub fn multiply_scalar(&self, scalar: BigInt) -> Point {
        // The identity element of the group, also known as the point at infinity
        let mut result: Point = Point::identity(self.curve.clone());

        // The point to be added to the result, initialized to the point to be multiplied
        let mut addend: Point = self.clone();

        // The scalar value to be multiplied
        let mut n: BigInt = scalar;

        // Whilst the scalar value is greater than 0
        while n > BigInt::from(0) {
            // If the scalar value is odd
            if &n % BigInt::from(2) == BigInt::from(1) {
                // Add the addend to the result
                result = result.add(addend.clone());
            }

            // Double the addend
            addend = addend.double();

            // Halve the scalar value
            n = n >> 1;
        }

        result
    }

    /// Addition of two points on an elliptic curve, assumes that the points are on the same curve
    pub fn add(&self, other: Point) -> Point {
        // x and y coordinates of the first point
        let x1: &BigInt = &self.x;
        let y1: &BigInt = &self.y;

        // x and y coordinates of the second point
        let x2: &BigInt = &other.x;
        let y2: &BigInt = &other.y;

        // The identity element of the group, also known as the point at infinity
        let identity: Point = Point::identity(self.curve.clone());

        // If either point is the identity element, return the second point
        if x1.eq(&identity.x) || y1.eq(&identity.y) {
            return other;
        }

        // If either point is the identity element, return the first point
        if x2.eq(&identity.x) || y2.eq(&identity.y) {
            return Point {
                x: x1.clone(),
                y: y1.clone(),
                curve: self.curve.clone(),
            };
        }

        // If the points are the same, double the point
        if x1.eq(x2) && y1.eq(y2) {
            return self.double();
        }

        // If the points are inverses, return the identity element
        if x1.eq(x2) && y1.eq(&-y2) {
            return identity;
        }

        // Calculate the lambda value (the slope of the line between the two points)
        let lambda: BigInt = {
            Point::modulo(
                (y2 - y1) * Point::invert(x2 - x1, self.curve.p()),
                self.curve.p(),
            )
        };

        // Calculate the x and y coordinates of the third point
        let x3: BigInt = Point::modulo(&lambda * &lambda - x1 - x2, self.curve.p());
        let y3: BigInt = Point::modulo(&lambda * (x1 - &x3) - y1, self.curve.p());

        Point {
            x: x3,
            y: y3,
            curve: self.curve.clone(),
        }
    }

    pub fn double(&self) -> Point {
        let lambda: BigInt = {
            Point::modulo(
                BigInt::from(3)
                    * self.x.pow(2)
                    * Point::invert(BigInt::from(2) * self.y.clone(), self.curve.p()),
                self.curve.p(),
            )
        };

        let x_r: BigInt = Point::modulo(
            &lambda * &lambda - BigInt::from(2) * &self.x,
            self.curve.p(),
        );

        let y_r: BigInt = Point::modulo(&lambda * (&self.x - &x_r) - &self.y, self.curve.p());

        Point {
            x: x_r,
            y: y_r,
            curve: self.curve,
        }
    }

    /// Calculates the multiplicative inverse
    fn invert(number: BigInt, modulo: BigInt) -> BigInt {
        if number.eq(&BigInt::from(0)) || modulo.le(&BigInt::from(0)) {
            panic!(
                "invert: expected positive integers, got n={} mod={}",
                number, modulo
            );
        }

        let mut a: BigInt = Point::modulo(number, modulo.clone());
        let mut b: BigInt = modulo.clone();
        let mut x: BigInt = BigInt::from(0);
        let mut y: BigInt = BigInt::from(1);
        let mut u: BigInt = BigInt::from(1);
        let mut v: BigInt = BigInt::from(0);

        // Extended Euclidean algorithm, adapted from pseudocode on Wikipedia
        while a.eq(&BigInt::from(0)) == false {
            let q: BigInt = &b / &a;
            let r: BigInt = &b % &a;
            let m: BigInt = &x - &u * &q;
            let n: BigInt = &y - &v * &q;

            b = a;
            a = r;
            x = u;
            y = v;
            u = m;
            v = n;
        }

        let gcd: BigInt = b;

        if gcd.ne(&BigInt::from(1)) {
            // TODO: really need to return a result here.
            panic!("unable to invert");
        }

        Point::modulo(x, modulo)
    }

    /// Performs a modulo operation that always returns a positive result
    fn modulo(a: BigInt, b: BigInt) -> BigInt {
        // Calculates the least nonnegative remainder of a (mod b)
        let result: BigInt = a.rem_euclid(&b);

        // If the result is positive, return it
        // Otherwise, return the positive result of adding b to the result
        if result >= BigInt::from(0) {
            result
        } else {
            b + result
        }
    }
}

#[cfg(test)]
mod test {
    use num_bigint::BigInt;

    use crate::ecc::{hex::Hex, Curve, CurveDomainProperties};

    use super::Point;

    #[test]
    fn test_vector_1() {
        let curve: Curve = Curve::Secp256k1;
        let generator: Point = Point {
            x: curve.gx(),
            y: curve.gy(),
            curve,
        };

        let private_key: BigInt =
            Hex::to_bigint("fab9fca923e226fa3e6d383a4b22e84c43babafae9a2c8fc332feb1a3327b69e");

        let public_key: Point = generator.multiply_scalar(private_key);

        assert_eq!(
            public_key.x.to_str_radix(16),
            "acf3f82a52e4f09f7d6334e46239125957ccfa7404370948031ac999c4d8e057"
        );

        assert_eq!(
            public_key.y.to_str_radix(16),
            "24447e7fd300f4b33154abc24d7ee473b20fe67334f28cefdcbd8c633c436586"
        )
    }

    #[test]
    fn test_vector_2() {
        let curve: Curve = Curve::Secp256k1;
        let generator: Point = Point {
            x: curve.gx(),
            y: curve.gy(),
            curve,
        };

        let private_key: BigInt =
            Hex::to_bigint("302593b9ef13aa4db375014910dac09a5027cec32061f95f7c5f6314d891841b");

        let public_key: Point = generator.multiply_scalar(private_key);

        assert_eq!(
            public_key.x.to_str_radix(16),
            "46f6dce90f1aaafe82284c17ca493b231c62e57074132da3010a277eb2a628d3"
        );
        assert_eq!(
            public_key.y.to_str_radix(16),
            "5e94a236576fd576badf01da8c5102851e323ff99cdde953681358f8d56e322e"
        );
    }
}
