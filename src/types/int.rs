use super::Open;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PubInt(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecInt<T: Open>(T);

impl<T: Add<Output = T> + Open> Add for SecInt<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        SecInt(self.0 + rhs.0)
    }
}

impl<T: Mul<Output = T> + Open> Mul for SecInt<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        SecInt(self.0 * rhs.0)
    }
}

impl<T: Open> Open for SecInt<T> {
    type Public = PubInt;

    fn open<U>(self, channel: &mut U) -> PubInt {
        todo!()
    }
}
