use super::Number;
use std::ops::{Add, Mul};

/// Public Gf2 type
///
/// This type is used for providing finite field arithmetic over two elements
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct PubGf2<T: Number + Default, const N: usize>([T; N]);

impl<T: Number, const N: usize> Number for PubGf2<T, N> {}

impl<T: Number, const N: usize> Add for PubGf2<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number, const N: usize> Mul for PubGf2<T, N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number, const N: usize> Default for PubGf2<T, N> {
    fn default() -> Self {
        PubGf2([T::default(); N])
    }
}

/// Secret Gf2 type
///
/// This type wraps different implementation for secret Gf2 types
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SecGf2<T: Number>(T);

impl<T: Number> Number for SecGf2<T> {}

impl<T: Number> Add for SecGf2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number> Mul for SecGf2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number + std::fmt::Display> std::fmt::Display for SecGf2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.0.to_string();
        write!(f, "{}", out.trim_start_matches('0'))
    }
}
