use ark_ff::Field;
use std::ops::{Add, Mul, Neg, Sub};
use types::vm::MpcType;

/// Representation of a share in a Shamir secret-sharing scheme.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Share<T: Field> {
    /// Share held by a party.
    value: T,
}

impl<T: Field> Share<T> {
    /// Creates a new share from a given value.
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

// ============================================================
// Implementation of operation between shares and clear values.
// ============================================================

impl<T: Field> Add<T> for Share<T> {
    type Output = Self;

    /// Computes the share of the addition between a secret value with a public
    /// value.
    fn add(self, rhs: T) -> Self::Output {
        Self {
            value: self.value + rhs,
        }
    }
}

impl<T: Field> Sub<T> for Share<T> {
    type Output = Self;

    /// Computes the share of the subtraction between a secret value with a
    /// public value.
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            value: self.value - rhs,
        }
    }
}

impl<T: Field> Mul<T> for Share<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            value: self.value * rhs,
        }
    }
}

// ===========================================
// Implementation of operation between shares.
// ===========================================

impl<T: Field> Add for Share<T> {
    type Output = Self;

    /// Adds two shares locally to obtain the share of the sum of the underlying
    /// secrets.
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl<T: Field> Mul for Share<T> {
    type Output = Self;

    /// Multiplies two shares. If one share has degree $d_1$ and the other share
    /// has degree $d_2$, then the resulting share will be of degree $d_1 + d_2$.
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}

impl<T: Field> Sub for Share<T> {
    type Output = Self;

    /// Subtracts two shares locally to obtain a share of the subtraction of the
    /// underlying secrets.
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl<T: Field> Neg for Share<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.value)
    }
}

impl<T: Field> MpcType for Share<T> {}
