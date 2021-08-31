use super::Open;
use super::PubInt;
use std::io::{Read, Write};
use std::ops::{Add, Mul};

pub(crate) struct Shamir(pub(crate) i32);

impl Open for Shamir {
    type Public = PubInt;

    fn open<U: Read + Write>(self, channel: &mut U) -> Self::Public {
        todo!()
    }
}

impl Add for Shamir {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul for Shamir {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
