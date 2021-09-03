use std::io::{Read, Write};
use std::ops::{Add, Mul};

mod shamir;

pub trait Open {
    type Public;
    fn open<U: Read + Write>(self, channel: &mut U) -> Self::Public;
}

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

// We need to consider that it is probably also possible to add/mulitply this way:
// secret +/* public = secret
impl<T: Add<Output = T> + Open> Add<PubInt> for SecInt<T> {
    type Output = Self;

    fn add(self, rhs: PubInt) -> Self {
        todo!()
    }
}

impl<T: Mul<Output = T> + Open> Mul for SecInt<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        SecInt(self.0 * rhs.0)
    }
}

impl<T: Open> Open for SecInt<T> {
    type Public = T::Public;

    fn open<U: Read + Write>(self, channel: &mut U) -> Self::Public {
        self.0.open::<U>(channel)
    }
}
