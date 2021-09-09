use ark_bls12_381::Fr;
use ark_ff::{BigInteger256, Field, PrimeField};
use std::ops::{Add, Mul};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Shamir<T: Field>(T);

impl Shamir<Fr> {
    pub fn new(integer: BigInteger256) -> Result<Shamir<Fr>, ShamirError> {
        let field_element = PrimeField::from_repr(integer).ok_or(ShamirError::Init(integer))?;
        Ok(Shamir(field_element))
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

#[derive(Debug, Error)]
pub enum ShamirError {
    #[error("Unable to create ShamirSecret from element {0}")]
    Init(BigInteger256),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shamir_new() {
        let _shamir_1 = Shamir::<Fr>::new(BigInteger256::from(0_u64)).unwrap();
        let _shamir_2 = Shamir::<Fr>::new(BigInteger256::from(2310498322_u64)).unwrap();
    }

    #[test]
    fn shamir_add() {
        let shamir_1 = Shamir::<Fr>::new(BigInteger256::from(5_u64)).unwrap();
        let shamir_2 = Shamir::<Fr>::new(BigInteger256::from(3_u64)).unwrap();

        assert_eq!(
            shamir_1 + shamir_2,
            Shamir::<Fr>::new(BigInteger256::from(8_u64)).unwrap()
        );
    }

    #[test]
    fn shamir_multiply() {
        let shamir_1 = Shamir::<Fr>::new(BigInteger256::from(42_u64)).unwrap();
        let shamir_2 = Shamir::<Fr>::new(BigInteger256::from(5_u64)).unwrap();

        assert_eq!(
            shamir_1 * shamir_2,
            Shamir::<Fr>::new(BigInteger256::from(210_u64)).unwrap()
        );
    }

    #[test]
    fn shamir_display() {
        let shamir_1 = Shamir::<Fr>::new(BigInteger256::from(42_u64)).unwrap();
        assert_eq!(
            shamir_1.to_string(),
            "000000000000000000000000000000000000000000000000000000000000002A"
        );
    }
}
