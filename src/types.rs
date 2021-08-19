use std::ops::{Add, Mul};
pub trait Open {
    fn open(self) -> PubInt;
}

#[derive(PrimeField)]
#[PrimeFieldModulus = "31"]
#[PrimeFieldGenerator = "3"]
#[PrimeFieldReprEndianness = "little"]
pub struct PubInt([u64; 1]);

impl Add for PubInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul for PubInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

pub struct SecInt<T: Open>(T);

struct Shamir(i32);

impl Open for Shamir {
    fn open(self) -> PubInt {
        todo!()
    }
}

struct Masked(i32);

impl Open for Masked {
    fn open(self) -> PubInt {
        todo!()
    }
}
