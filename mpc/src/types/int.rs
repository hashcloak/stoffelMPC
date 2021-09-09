use super::shamir::{Shamir, ShamirError};
use ark_bls12_381::Fr;
use num_bigint::BigUint;
use std::convert::TryInto;
use std::ops::{Add, Mul};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PubInt(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecInt<T>(T);

impl<T: Add<Output = T>> Add for SecInt<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        SecInt(self.0 + rhs.0)
    }
}

impl SecInt<Shamir<Fr>> {
    pub fn new(integer: impl Into<BigUint>) -> Result<SecInt<Shamir<Fr>>, SecIntError> {
        let integer = integer
            .into()
            .try_into()
            .map_err(|_| SecIntError::Infallible)?;
        Ok(SecInt(
            Shamir::new(integer).map_err(SecIntError::InitShamir)?,
        ))
    }
}

// We need to consider that it is probably also possible to add/mulitply this way:
// secret +/* public = secret
impl<T: Add<Output = T>> Add<PubInt> for SecInt<T> {
    type Output = Self;

    fn add(self, rhs: PubInt) -> Self {
        todo!()
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
pub enum SecIntError {
    #[error("Unable to initialize SecInt with ShamirSecret {0}")]
    InitShamir(#[source] ShamirError),
    #[error("Impossible Error happened")]
    Infallible,
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
