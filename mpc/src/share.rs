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
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Field> Add for Share<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl<T: Field> Mul for Share<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}

impl<T: Field> MpcType for Share<T> {}
