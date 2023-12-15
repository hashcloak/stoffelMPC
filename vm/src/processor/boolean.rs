use super::Processor;
use crate::state::{Register, StackRegister};
use mpc::protocols::MPCProtocol;
use mpc::share::Share;

#[derive(Debug, Clone, Default)]
pub struct BooleanCore<T: MPCProtocol> {
    secret_stack: StackRegister<Share<T::Domain>>,
    public_stack: StackRegister<T::Domain>,
    secret_register: Register<Share<T::Domain>>,
    public_register: Register<T::Domain>,
}

impl<T: MPCProtocol> BooleanCore<T> {}

impl<T: MPCProtocol> Processor for BooleanCore<T> {
    fn jump(&mut self, new_program_counter: usize) {
        todo!()
    }

    fn relative_jump(&mut self, n_positions: usize) {
        todo!()
    }

    fn increment_program_counter(&mut self) {
        todo!()
    }
}
