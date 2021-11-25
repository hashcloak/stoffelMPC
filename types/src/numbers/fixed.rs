use super::secret_sharing::SecretSharing;
use super::{MPCType};
use std::ops::{Add, Mul};

/// Public fixed point type
///
/// This type is used for providing arithmetic for fixed point numbers
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PubFixed();

impl MPCType for PubFixed {
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

impl Add for PubFixed {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl Mul for PubFixed {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

/// Secret fixed point type
///
/// This type wraps different implementations for secret fixed point types
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecFixed<T: SecretSharing>(T);

impl<T: SecretSharing> MPCType for SecFixed<T> {
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

impl<T: SecretSharing> Add for SecFixed<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: SecretSharing> Mul for SecFixed<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}