use ark_ff::PrimeField;
use std::fmt::Debug;
use types::vm::MpcType;

pub mod honey_badger;

pub trait MPCProtocol: Debug {
    /// Represents the underlying domain of computation used by the protocol.
    type Domain: PrimeField + MpcType;

    fn compute();

    fn setup();

    fn send(value: &Self::Domain, recipient: Box<dyn std::io::Write>);
}
