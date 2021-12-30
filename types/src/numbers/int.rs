use super::secret_sharing::{shamir::Shamir, SecretSharing};
use super::{MPCType};
use ark_ff::PrimeField;
use num_bigint::BigUint;
use std::ops::{Add, Mul};
use thiserror::Error;

/// Public integer
///
/// This type is used for providing public integers of arbitrary size.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
pub struct PubInt([u64; 4]);

impl MPCType for PubInt {

    fn square(self) -> Self {
        todo!();
    }

    fn pow(self, exp: usize) -> Self {
        todo!();
    }

    fn min(self, a: Self) -> Self {
        todo!();
    }

    fn max(self, a: Self) -> Self {
        todo!();
    }

    fn if_else(self, a: Self, b: Self) -> Self {
        todo!();
    }

    fn cond_swap(self, a: Self, b: Self) -> (Self, Self) {
        todo!();
    }
}

impl Add for PubInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl Mul for PubInt {
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
pub struct SecInt<T: SecretSharing>(T);

impl<T: SecretSharing + Add<Output = T> + Mul<Output = T>> MPCType for SecInt<T> {
    fn square(self) -> Self {
        todo!();
    }

    fn pow(self, exp: usize) -> Self {
        todo!();
    }

    fn min(self, a: Self) -> Self {
        todo!();
    }

    fn max(self, a: Self) -> Self {
        todo!();
    }

    fn if_else(self, a: Self, b: Self) -> Self {
        todo!();
    }

    fn cond_swap(self, a: Self, b: Self) -> (Self, Self) {
        todo!();
    }
}

impl<T: SecretSharing + Add<Output = T>> Add for SecInt<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        SecInt(self.0 + rhs.0)
    }
}

impl<T: SecretSharing + Mul<Output = T>> Mul for SecInt<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        SecInt(self.0 * rhs.0)
    }
}

impl<T: PrimeField> SecInt<Shamir<T>> {
    /// Creates a new secret integer using a Shamir secret
    ///
    /// This will return a Shamir secret wrapped in a `SecInt`
    pub fn new(integer: impl Into<BigUint>) -> Result<Self, IntegerError> {
        let integer: BigUint = integer.into();
        Ok(SecInt(
            Shamir::<T>::new(integer).map_err(|_| IntegerError::Init)?,
        ))
    }
}

impl<T: SecretSharing + std::fmt::Display> std::fmt::Display for SecInt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.0.to_string();
        write!(f, "{}", out.trim_start_matches('0'))
    }
}

#[derive(Debug, Error)]
pub enum IntegerError {
    #[error("Unable to initialize SecInt")]
    Init,
}

#[cfg(test)]
mod tests {
    use ark_bls12_381::{Fr, FrParameters};
    use ark_ff::Fp256;

    use super::*;

    #[test]
    fn sec_int_shamir_new() {
        let _secret_int = SecInt::<Shamir<Fp256<FrParameters>>>::new(42_u64).unwrap();
    }

    #[test]
    fn sec_int_shamir_add() {
        let secret_int_1 = SecInt::<Shamir<Fr>>::new(42_u64).unwrap();
        let secret_int_2 = SecInt::<Shamir<Fr>>::new(2_u64).unwrap();

        assert_eq!(
            secret_int_1 + secret_int_2,
            SecInt::<Shamir<Fr>>::new(44_u64).unwrap()
        );
    }

    #[test]
    fn sec_int_shamir_multiply() {
        let secret_int_1 = SecInt::<Shamir<Fr>>::new(42_u64).unwrap();
        let secret_int_2 = SecInt::<Shamir<Fr>>::new(2_u64).unwrap();

        assert_eq!(
            secret_int_1 * secret_int_2,
            SecInt::<Shamir<Fr>>::new(84_u64).unwrap()
        );
    }

    #[test]
    fn sec_int_shamir_display() {
        let secret_int = SecInt::<Shamir<Fr>>::new(42_u64).unwrap();

        assert_eq!(secret_int.to_string(), "2A");
    }
}
