use super::Open;
use super::PubInt;
use ark_bls12_381::Fr;
use ark_ff::BigInteger256;
use ark_ff::{Field, PrimeField};
use std::io::{Read, Write};
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) struct ShamirSecret<T: PrimeField>(T);

impl ShamirSecret<Fr> {
    pub fn new(element: BigInteger256) -> ShamirSecret<Fr> {
        let field_element = PrimeField::from_repr(element).unwrap();
        ShamirSecret(field_element)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shamir_new() {
        let shamir_1 = ShamirSecret::<Fr>::new(42.into());
        let shamir_2 = ShamirSecret::<Fr>::new(2.into());
        println!("First: {:?}\n Second: {:?}", shamir_1, shamir_2);

        assert_eq!(shamir_1 + shamir_2, ShamirSecret::<Fr>::new(44.into()));
        assert_eq!(shamir_1 * shamir_2, ShamirSecret::<Fr>::new(84.into()));
    }
}
