use super::Processor;
use crate::state::{Register, StackRegister};
use crate::{program, Program};
use mpc::protocols::MPCProtocol;
use mpc::share::Share;

#[derive(Debug, Clone, Default)]
pub struct ArithmeticCore<T: MPCProtocol> {
    /// Stack for the secret values.
    pub secret_stack: StackRegister<Share<T::Domain>>,
    /// Stack for the clear values.
    pub clear_stack: StackRegister<T::Domain>,

    /// Register for secret values.
    pub secret_register: Register<Share<T::Domain>>,
    /// Register for clear values.
    pub clear_register: Register<T::Domain>,

    /// Program counter for the core.
    pub program_counter: usize,
}

impl<T: MPCProtocol> ArithmeticCore<T> {}

impl<T: MPCProtocol> Processor for ArithmeticCore<T> {
    /// Cleans both the clear and secret registers.
    fn clear_registers(&mut self) {
        self.clear_register.clear();
        self.secret_register.clear();
    }

    /// Returns the program size.
    fn get_program_size(&self) -> usize {
        todo!()
    }

    /// Executes a given program
    fn execute(&mut self, program: Program) {
        todo!()
    }

    fn jump(&mut self, new_program_counter: usize) {
        self.program_counter = new_program_counter;
    }

    fn relative_jump(&mut self) {
        todo!()
    }

    /// Increments program counter
    fn increment_program_counter(&mut self) {
        self.program_counter += 1
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
