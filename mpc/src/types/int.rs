use super::shamir::{Shamir, ShamirError};
use ark_bls12_381::Fr;
use ark_ff::BigInteger256;
use num_bigint::BigUint;
use std::convert::TryInto;
use std::ops::{Add, Mul};
use thiserror::Error;

/// A public integer
///
/// This type is used for providing public integers of arbitrary size.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PubInt(pub BigUint);

impl From<BigInteger256> for PubInt {
    fn from(integer: BigInteger256) -> Self {
        // This unwrap() should be safe because the operation is infallible
        PubInt(integer.try_into().unwrap())
    }
}

/// The secret integer type
///
/// This type wraps different implementation for secret integers
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecInt<T>(T);

impl<T: Add<Output = T>> Add for SecInt<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        SecInt(self.0 + rhs.0)
    }
}

impl SecInt<Shamir<Fr>> {
    /// Creates a new secret integer using a Shamir secret
    ///
    /// This will return a Shamir secret wrapped in a `SecInt`
    pub fn new(integer: impl Into<BigUint>) -> Result<Self, IntegerError> {
        // The unwrap() should be safe because the operation is infallible
        let integer = integer.into().try_into().unwrap();
        Ok(SecInt(
            Shamir::new(integer).map_err(IntegerError::InitShamir)?,
        ))
    }

    /// Adds a `PubInt`
    ///
    /// This allows adding a public integer to a secret integer returning a new
    /// secret integer.
    pub fn add_public(&mut self, pub_int: PubInt) -> Result<Self, IntegerError> {
        let new_secret = SecInt::new(pub_int.0)?;
        *self = *self + new_secret;
        Ok(*self)
    }
}

impl<T: Mul<Output = T>> Mul for SecInt<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        SecInt(self.0 * rhs.0)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for SecInt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.0.to_string();
        write!(f, "{}", out.trim_start_matches('0'))
    }
}

#[derive(Debug, Error)]
pub enum IntegerError {
    #[error("Unable to initialize SecInt with ShamirSecret {0}")]
    InitShamir(#[source] ShamirError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sec_int_shamir_new() {
        let _secret_int = SecInt::<Shamir<Fr>>::new(42_u64).unwrap();
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
