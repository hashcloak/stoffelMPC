use ark_ff::fields::Field;
use std::fmt::Debug;
use types::vm::MpcType;

pub mod honey_badger;

pub trait MPCProtocol: Debug {
    /// Represents the underlying domain of computation used by the protocol.
    type Domain: Field + MpcType;

    fn compute();

    fn setup();

    fn send(value: &Self::Domain, recipient: Box<dyn std::io::Write>);
}
