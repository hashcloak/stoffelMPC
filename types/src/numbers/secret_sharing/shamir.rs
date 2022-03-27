use super::{SecretSharing, SecretSharingError};
use ark_ff::{Field, PrimeField};
use num_bigint::BigUint;
use std::ops::{Add, Mul};
use thiserror::Error;

/// A Shamir secret
///
/// This type is used to represent a secret-shared value using polynomials.
/// It can be used as an inner representation of some integer value.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Shamir<T: Field>(T);

impl<T: PrimeField> Shamir<T> {
    /// Create a new Shamir secret
    ///
    /// This implementation uses the BLS12-381 prime field.
    pub fn new(integer: BigUint) -> Result<Shamir<T>, ShamirError> {
        let field_element = T::from(integer);
        Ok(Shamir(field_element))
    }
}

impl<T: Field> From<T> for Shamir<T> {
    fn from(element: T) -> Self {
        Shamir(element)
    }
}

impl<T: Field> Add for Shamir<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Shamir(self.0 + rhs.0)
    }
}

impl<T: Field> Mul for Shamir<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Shamir(self.0 * rhs.0)
    }
}

impl<T: PrimeField> std::fmt::Display for Shamir<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0.into_repr())
    }
}

impl<T: PrimeField> SecretSharing for Shamir<T> {
    fn share(&mut self) -> Result<(), SecretSharingError> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum ShamirError {
    #[error("Unable to convert element into prime field")]
    PrimeFieldConvert,
    #[error("Unable to create sharing")]
    CreateSharing,
    #[error("Not enough evaluation points to reconstruct secret")]
    Underdetermined,
}

#[cfg(test)]
mod tests {
    use ark_bls12_381::Fr;

    use super::*;

    #[test]
    fn shamir_new() {
        let _shamir_1 = Shamir::<Fr>::new(0_u64.into()).unwrap();
        let _shamir_2 = Shamir::<Fr>::new(2310498322_u64.into()).unwrap();
    }

    #[test]
    fn shamir_add() {
        let shamir_1 = Shamir::<Fr>::new(5_u64.into()).unwrap();
        let shamir_2 = Shamir::<Fr>::new(3_u64.into()).unwrap();

        assert_eq!(
            shamir_1 + shamir_2,
            Shamir::<Fr>::new(8_u64.into()).unwrap()
        );
    }

    #[test]
    fn shamir_multiply() {
        let shamir_1 = Shamir::<Fr>::new(42_u64.into()).unwrap();
        let shamir_2 = Shamir::<Fr>::new(5_u64.into()).unwrap();

        assert_eq!(
            shamir_1 * shamir_2,
            Shamir::<Fr>::new(210_u64.into()).unwrap()
        );
    }

    #[test]
    fn shamir_display() {
        let shamir_1 = Shamir::<Fr>::new(42_u64.into()).unwrap();
        assert_eq!(
            shamir_1.to_string(),
            "000000000000000000000000000000000000000000000000000000000000002A"
        );
    }
}