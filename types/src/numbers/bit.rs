use super::secret_sharing::SecretSharing;
use super::{MPCType};
use std::ops::{Add, Mul};

/// Public bit type
///
/// This type is used for providing arithmetic for bits
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PubBit(bool);

impl MPCType for PubBit { 
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

impl Add for PubBit {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl Mul for PubBit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

/// Secret bit type
///
/// This type wraps different implementations for secret bit types
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecBit<T: SecretSharing>(T);

impl<T: SecretSharing> MPCType for SecBit<T> {
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

impl<T: SecretSharing> Add for SecBit<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: SecretSharing> Mul for SecBit<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}