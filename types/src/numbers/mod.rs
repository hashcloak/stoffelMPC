pub mod bit;
pub mod fixed;
pub mod float;
pub mod gf2;
pub mod int;
use std::ops::{Add, Mul};

pub mod secret_sharing;

/// This trait is a catch-all for a type that is used 
/// within MPC protocols. 

pub trait MPCType: Sized + Add<Output=Self> + Mul<Output=Self> + Copy + Default{

    /// Returns the square of an MPCType
    fn square(self) -> Self;

    /// Returns the power of itself to an exponent
    fn pow(self, exp: usize) -> Self;

    /// Returns the minimum between itself and another MPCType
    fn min(self, a: Self) -> Self;

    /// Returns the maximum between itself and another MPCType
    fn max(self, b: Self) -> Self;

    /// Returns a if self == 1 and b if self == 0
    fn if_else(self, a: Self, b: Self) -> Self;

    /// Returns (a, b) if self == 0 and (b, a) if self == 1
    fn cond_swap(self, a: Self, b: Self) -> (Self, Self);
}