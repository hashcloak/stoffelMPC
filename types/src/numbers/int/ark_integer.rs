use crate::numbers::Number;
use ark_ff::BigInteger256;
use num_bigint::BigUint;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
pub struct ArkInt(BigInteger256);

impl ArkInt {
    pub fn new(int: BigUint) -> Result<Self, String> {
        Ok(ArkInt(BigInteger256::try_from(int)?))
    }
}

impl Number for ArkInt {}

impl Add for ArkInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!();
    }
}

impl Mul for ArkInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!();
    }
}
