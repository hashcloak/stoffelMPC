pub mod bit;
pub mod fixed;
pub mod float;
pub mod gf2;
pub mod int;
use std::ops::{Add, Mul};

/// This trait is a catch-all for a type that is used
/// within MPC protocols.
pub trait Number:
    Add<Output = Self> + Mul<Output = Self> + Copy + Default + std::fmt::Debug + 'static
{
    /// Returns the square of an MPCType
    fn square(self) -> Self {
        todo!();
    }

    /// Returns the power of itself to an exponent
    fn pow(self, exp: usize) -> Self {
        todo!();
    }

    /// Returns the minimum between itself and another MPCType
    fn min(self, a: Self) -> Self {
        todo!();
    }

    /// Returns the maximum between itself and another MPCType
    fn max(self, b: Self) -> Self {
        todo!();
    }

    /// Returns a if self == 1 and b if self == 0
    fn if_else(self, a: Self, b: Self) -> Self {
        todo!();
    }

    /// Returns (a, b) if self == 0 and (b, a) if self == 1
    fn cond_swap(self, a: Self, b: Self) -> (Self, Self) {
        todo!();
    }
}
<<<<<<< HEAD

/// An abstraction for communication
/// in MPC protocols
pub trait SecretSharing {
    type Public;

    fn share(&self) {
        todo!();
    }

    fn open(&self) -> Self::Public {
        todo!();
    }
}
=======
>>>>>>> main
