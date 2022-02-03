use super::Number;
use std::ops::{Add, Mul};

/// Public floating point type
///
/// This type is used for providing arithmetic for floating point numbers
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Default)]
pub struct PubFloat<T: Number>(T);

impl<T: Number> Number for PubFloat<T> {}

impl<T: Number> Add for PubFloat<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number> Mul for PubFloat<T> {
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
pub struct SecFloat<T: Number>(T);

impl<T: Number> Number for SecFloat<T> {}

impl<T: Number> Add for SecFloat<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number> Mul for SecFloat<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: Number + std::fmt::Display> std::fmt::Display for SecFloat<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.0.to_string();
        write!(f, "{}", out.trim_start_matches('0'))
    }
}
