use ark_ff::Field;
use std::ops::{Add, Mul};
use types::vm::MpcType;

/// Representation of a share in a Shamir secret-sharing scheme.
#[derive(Debug, Default, Clone, Copy)]
pub struct Share<T: Field> {
    /// Share held by a party.
    value: T,
}

impl<T: Field> Share<T> {
    /// Creates a new share from a given value.
    fn new(value: T) -> Self {
        Self { value }
    }
}

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

impl<T: Field> MpcType for Share<T> {}
