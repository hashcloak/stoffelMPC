use std::fmt::Debug;
use types::compiler::SkeletonType;
use types::vm::Number;

pub mod honey_badger;

pub trait MPCProtocol: Debug {
    type Public: SkeletonType;
    type Secret: SkeletonType;
    type VmType: Number + Debug;

    fn compute();

    fn setup();

    fn into_vm_type(number: impl Number) -> Self::VmType;

    fn send(value: &Self::VmType, recipient: Box<dyn std::io::Write>);
}
