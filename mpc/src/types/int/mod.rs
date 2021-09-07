use ark_ff::Field;
use std::ops::{Add, Mul};

mod shamir;
pub use shamir::{ShamirError, ShamirSecret};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PubInt(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecInt<T>(T);

impl<T: Add<Output = T>> Add for SecInt<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        SecInt(self.0 + rhs.0)
    }
}

impl<T: Field> SecInt<ShamirSecret<T>> {
    pub fn new(integer: i32) -> SecInt<ShamirSecret<T>> {
        todo!()
    }
}

// We need to consider that it is probably also possible to add/mulitply this way:
// secret +/* public = secret
impl<T: Add<Output = T>> Add<PubInt> for SecInt<T> {
    type Output = Self;

    fn add(self, rhs: PubInt) -> Self {
        todo!()
    }
}

impl<T: Mul<Output = T>> Mul for SecInt<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        SecInt(self.0 * rhs.0)
    }
}
