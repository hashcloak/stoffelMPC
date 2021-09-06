use super::Open;
use super::PubInt;
use ark_bls12_381::Fr;
use ark_ff::BigInteger256;
use ark_ff::PrimeField;
use std::io::{Read, Write};
use std::ops::{Add, Mul};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) struct ShamirSecret<T: PrimeField>(T);

impl ShamirSecret<Fr> {
    pub fn new(element: BigInteger256) -> Result<ShamirSecret<Fr>, ShamirError> {
        let field_element =
            PrimeField::from_repr(element).ok_or(ShamirError::InitError(element))?;
        Ok(ShamirSecret(field_element))
    }
}

impl<T: PrimeField> Open for ShamirSecret<T> {
    type Public = PubInt;

    fn open<U: Read + Write>(self, channel: &mut U) -> Self::Public {
        todo!()
    }
}

impl<T: PrimeField> Add for ShamirSecret<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ShamirSecret(self.0 + rhs.0)
    }
}

impl<T: PrimeField> Mul for ShamirSecret<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ShamirSecret(self.0 * rhs.0)
    }
}

impl std::fmt::Display for ShamirSecret<Fr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Error)]
pub enum ShamirError {
    #[error("Unable to create ShamirSecret from element {0}")]
    InitError(BigInteger256),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shamir_new() {
        let _shamir_1 = ShamirSecret::<Fr>::new(0.into()).unwrap();
        let _shamir_2 = ShamirSecret::<Fr>::new(2310498322.into()).unwrap();
    }

    #[test]
    fn shamir_add() {
        let shamir_1 = ShamirSecret::<Fr>::new(5.into()).unwrap();
        let shamir_2 = ShamirSecret::<Fr>::new(3.into()).unwrap();

        assert_eq!(
            shamir_1 + shamir_2,
            ShamirSecret::<Fr>::new(8.into()).unwrap()
        );
    }

    #[test]
    fn shamir_multiply() {
        let shamir_1 = ShamirSecret::<Fr>::new(42.into()).unwrap();
        let shamir_2 = ShamirSecret::<Fr>::new(5.into()).unwrap();

        assert_eq!(
            shamir_1 * shamir_2,
            ShamirSecret::<Fr>::new(210.into()).unwrap()
        );
    }
}
