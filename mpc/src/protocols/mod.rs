pub mod hbmpc;

use ark_ff::{PrimeField, ToBytes};
use types::numbers::{Number};
use super::utils::Ring;

pub trait MPCProtocol<N: Number> {
    type Share;

    type Input;

    type Output;

    type Error;

    type Parameters;

    fn compute() -> Result<Self::Output, Self::Error>;

    fn setup() -> Result<Self::Parameters, Self::Error>;
}