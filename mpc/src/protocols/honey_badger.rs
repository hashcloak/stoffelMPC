use types::compiler::Integer;

use super::MPCProtocol;

#[derive(Debug)]
pub struct HoneyBadgerMPC;

impl MPCProtocol for HoneyBadgerMPC {
    type Public = Integer;
    type Secret = Integer;
    type VmType = u32;

    fn compute() {
        todo!()
    }

    fn setup() {
        todo!()
    }

    fn into_vm_type(
        number: impl types::compiler::CompilerType + Into<Self::VmType>,
    ) -> Self::VmType {
        todo!()
    }

    fn send(value: &Self::VmType, recipient: Box<dyn std::io::Write>) {
        todo!()
    }
}
