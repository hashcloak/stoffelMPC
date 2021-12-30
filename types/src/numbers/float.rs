use super::secret_sharing::SecretSharing;
use super::{MPCType};
use std::ops::{Add, Mul};

/// Public floating point type
///
/// This type is used for providing arithmetic for floating point numbers
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Default)]
pub struct PubFloat(f32);

impl MPCType for PubFloat {
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

impl Add for PubFloat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl Mul for PubFloat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

/// Secret floating point type
///
/// This type wraps different implementations for secret floating point types
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SecFloat<T: SecretSharing>(T);


impl<T: SecretSharing> MPCType for SecFloat<T> {
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

impl<T: SecretSharing> Add for SecFloat<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: SecretSharing> Mul for SecFloat<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}