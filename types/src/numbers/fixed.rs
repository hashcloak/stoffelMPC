use super::{Number, SecretSharing};
use std::ops::{Add, Mul};

/// Public fixed point type
///
/// This type is used for providing arithmetic for fixed point numbers
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
pub struct PubFixed<T: Number>(T);

impl<T: Number> Number for PubFixed<T> {}

impl<T: Number> Add for PubFixed<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number> Mul for PubFixed<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

/// Secret fixed point type
///
/// This type wraps different implementations for secret fixed point types
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SecFixed<T: Number>(T);

impl<T: Number> Number for SecFixed<T> {}

impl<T: Number> SecretSharing for SecFixed<T> {
    type Public = PubFixed<T>;
}

impl<T: Number> Add for SecFixed<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number> Mul for SecFixed<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number + std::fmt::Display> std::fmt::Display for SecFixed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.0.to_string();
        write!(f, "{}", out.trim_start_matches('0'))
    }
}
