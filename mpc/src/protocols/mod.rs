use std::fmt::Debug;
use types::numbers::Number;
use types::vm::Integer;

pub mod honey_badger;

pub trait MPCProtocol: Debug {
    type Public: Number;
    type Secret: Number;
    type VmType: Integer + Debug;

    fn compute();

    fn setup();

    fn into_vm_type(number: impl Number) -> Self::VmType;
}
