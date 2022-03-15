use super::Number;
use std::ops::{Add, Mul};

/// Public bit type
///
/// This type is used for providing arithmetic for bits
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
pub struct PubBit<T: Number>(T);

impl<T: Number> Number for PubBit<T> {}

impl<T: Number> Add for PubBit<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number> Mul for PubBit<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

/// Secret bit type
///
/// This type wraps different implementations for secret bit types
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SecBit<T: Number>(T);

impl<T: Number> Number for SecBit<T> {}

impl<T: Number> Add for SecBit<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number> Mul for SecBit<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number + std::fmt::Display> std::fmt::Display for SecBit<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.0.to_string();
        write!(f, "{}", out.trim_start_matches('0'))
    }
}
