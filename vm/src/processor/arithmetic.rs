use super::Processor;
use crate::state::{Register, StackRegister};
use mpc::protocols::MPCProtocol;
use mpc::share::Share;
use types::vm::MpcType;

#[derive(Debug, Clone, Default)]
pub struct ArithmeticCore<T: MPCProtocol> {
    pub secret_stack: StackRegister<Share<T::Domain>>,
    pub public_stack: StackRegister<T::Domain>,
    pub secret_register: Register<Share<T::Domain>>,
    pub public_register: Register<T::Domain>,
}

impl<T: MPCProtocol> ArithmeticCore<T> where T::Domain: MpcType {}

impl<T: MPCProtocol> Processor for ArithmeticCore<T> {
    fn clear_registers(&mut self) {
        todo!()
    }

    fn get_program_counter(&self) -> usize {
        todo!()
    }

    fn get_program_size(&self) -> usize {
        todo!()
    }

    fn execute(&mut self) {
        todo!()
    }

    fn jump(&mut self) {
        todo!()
    }

    fn relative_jump(&mut self) {
        todo!()
    }

    fn increment_program_counter(&mut self) {
        todo!()
    }

    fn read_tape(&self) {
        todo!()
    }

    fn receive_tape(&self) {
        todo!()
    }

    fn receive_private_input(&self, to_store_in_memory: bool) {
        todo!()
    }

    fn give_private_output(&self, to_store_in_memory: bool) {
        todo!()
    }
}
