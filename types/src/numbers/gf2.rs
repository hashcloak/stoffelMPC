use super::secret_sharing::SecretSharing;
use super::MPCType;
use std::ops::{Add, Mul};

/// Public Gf2 type
///
/// This type is used for providing finite field arithmetic over two elements
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct PubGf2<const N: usize>([bool; N]);

impl<const N: usize> MPCType for PubGf2<N> {
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

impl<const N: usize> Add for PubGf2<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<const N: usize> Mul for PubGf2<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<const N: usize> Default for PubGf2<N> {
    fn default() -> Self {
        PubGf2([false; N])
    }
}

/// Secret Gf2 type
///
/// This type wraps different implementation for secret Gf2 types
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SecGf2<T: SecretSharing>(T);

impl<T: SecretSharing> MPCType for SecGf2<T> {
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

impl<T: SecretSharing> Add for SecGf2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: SecretSharing> Mul for SecGf2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

