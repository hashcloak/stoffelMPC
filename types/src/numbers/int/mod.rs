use super::{Number, SecretSharing};
use ark_ff::PrimeField;
use num_bigint::BigUint;
use std::ops::{Add, Mul};

mod ark_integer;
use ark_integer::ArkInt;

/// Public integer
///
/// This type is used for providing public integers of arbitrary size.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
pub struct PubInt<T = ArkInt>(T)
where
    T: Number;

impl<T: Number> Number for PubInt<T> {}

impl<T: Number> Add for PubInt<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number> Mul for PubInt<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

/// Secret integer type
///
/// This type wraps different implementation for secret integers
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SecInt<T: Number>(T);

impl<T: Number> Number for SecInt<T> {}

impl<T: Number> SecretSharing for SecInt<T> {
    type Public = PubInt;
}

impl<T: Number> Add for SecInt<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        SecInt(self.0 + rhs.0)
    }
}

impl<T: Number> Mul for SecInt<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        SecInt(self.0 * rhs.0)
    }
}

impl<T: Number + PrimeField> SecInt<T> {
    /// Creates a new secret integer using an ark prime field
    ///
    /// This will return a `SecInt` using an ark prime field
    /// as the underlying representation
    pub fn new(integer: impl Into<BigUint>) -> Self {
        let integer: BigUint = integer.into();
        SecInt(T::from(integer))
    }
}

impl<T: Number + std::fmt::Display> std::fmt::Display for SecInt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.0.to_string();
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::numbers::Number;
    use ark_bls12_381::Fr;

    impl Number for Fr {}

    #[test]
    fn sec_int_new() {
        let _secret_int = SecInt::<Fr>::new(42_u64);
    }

    #[test]
    fn sec_int_add() {
        let secret_int_1 = SecInt::<Fr>::new(42_u64);
        let secret_int_2 = SecInt::<Fr>::new(2_u64);

        assert_eq!(secret_int_1 + secret_int_2, SecInt::<Fr>::new(44_u64));
    }

    #[test]
    fn sec_int_multiply() {
        let secret_int_1 = SecInt::<Fr>::new(42_u64);
        let secret_int_2 = SecInt::<Fr>::new(2_u64);

        assert_eq!(secret_int_1 * secret_int_2, SecInt::<Fr>::new(84_u64));
    }

    #[test]
    fn sec_int_display() {
        let secret_int = SecInt::<Fr>::new(42_u64);

        assert_eq!(
            secret_int.to_string(),
            "Fp256 \"(000000000000000000000000000000000000000000000000000000000000002A)\""
        );
    }
}
