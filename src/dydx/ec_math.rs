/*
 * Math needed for the signatures
*/

use primitive_types::U512;

use crate::dydx::constants::*;

pub struct ECMath;

impl ECMath {

    pub(crate) fn ec_add(p1: (U512, U512), p2: (U512, U512)) -> (U512, U512) {
        let modulus = *FIELD_PRIME;

        let inverse = Self::modinv(Self::mod_sub(p1.0, p2.0, modulus), modulus);
        let m = (Self::mod_sub(p1.1, p2.1, modulus) * inverse) % modulus;
        let x = Self::mod_sub(Self::mod_sub(m * m, p1.0, modulus), p2.0, modulus) % modulus;
        let y = Self::mod_sub(m * Self::mod_sub(p1.0, x, modulus), p1.1, modulus) % modulus;
        (x, y)
    }

    fn mod_sub(first: U512, second: U512, modulus: U512) -> U512 {
        if first > second {
            first - second
        }
        else {
            modulus - (second - first)
        }
    }

    pub(crate) fn modinv(integer: U512, modulus: U512) -> U512 {
        if integer == U512::one() { return U512::one(); }
        let (mut a, mut m, mut x, mut inv) = (integer, modulus, U512::zero(), U512::one());

        while a > U512::one() {
            let div = a / m;
            let rem = a % m;
            if (div * x) > inv {
                inv = modulus - (((div * x) % modulus) - inv);
            }
            else {
                inv = (inv - (div * x)) % modulus;
            }
            a = rem;
            std::mem::swap(&mut a, &mut m);
            std::mem::swap(&mut x, &mut inv);
        }
        inv
    }

    pub(crate) fn ec_mult(scalar: U512, point: (U512, U512), alpha: U512) -> (U512, U512) {
        if scalar == U512::one() { return point; }
        if scalar % 2 == U512::zero() { return Self::ec_mult(scalar/U512::from(2), Self::ec_double(point, alpha), alpha); }
        Self::ec_add(Self::ec_mult(scalar - U512::one(), point, alpha), point)
    }

    pub(crate) fn ec_double(point: (U512, U512), alpha: U512) -> (U512, U512) {
        let modulus = *FIELD_PRIME;
        assert!(point.1 % modulus != U512::zero());
        let inverse = Self::modinv((U512::from(2) * point.1) % modulus, modulus) % modulus;
        let m = (((((((U512::from(3) * point.0) % modulus) * point.0) % modulus) + alpha) % modulus) * inverse) % modulus;
        let x = Self::mod_sub((m * m) % modulus, (U512::from(2) * point.0) % modulus, modulus) % modulus;
        let y = Self::mod_sub((m * Self::mod_sub(point.0, x, modulus)) % modulus, point.1, modulus) % modulus;
        (x, y)
    }
}
