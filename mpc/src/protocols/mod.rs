use std::fmt::Debug;
use types::compiler::CompilerType;
use types::vm::Number;

pub mod honey_badger;

pub trait MPCProtocol: Debug {
    type Public: CompilerType;
    type Secret: CompilerType;
    type VmType: Number + Debug;

    fn compute();

    fn setup();

    fn into_vm_type(number: impl Number) -> Self::VmType;

    fn send(value: &Self::VmType, recipient: Box<dyn std::io::Write>);
}
