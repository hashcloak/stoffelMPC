use super::Processor;
use crate::state::{Register, StackRegister};
use crate::{program, Program};
use mpc::protocols::MPCProtocol;
use mpc::share::Share;

#[derive(Debug, Clone, Default)]
pub struct ArithmeticCore<T: MPCProtocol> {
    /// Stack for the secret values.
    secret_stack: StackRegister<Share<T::Domain>>,
    /// Stack for the clear values.
    clear_stack: StackRegister<T::Domain>,

    /// Register for secret values.
    secret_register: Register<Share<T::Domain>>,
    /// Register for clear values.
    clear_register: Register<T::Domain>,

    /// Program counter for the core.
    program_counter: usize,
}

impl<T: MPCProtocol> ArithmeticCore<T> {
    /// Creates an arithmetic processor with program counter in zero.
    pub fn new() -> ArithmeticCore<T> {
        ArithmeticCore {
            secret_stack: StackRegister::new(),
            clear_stack: StackRegister::new(),
            secret_register: Register::new(),
            clear_register: Register::new(),
            program_counter: 0,
        }
    }

    pub fn clear_register_mut(&mut self) -> &mut Register<T::Domain> {
        &mut self.clear_register
    }

    pub fn clear_register(&self) -> &Register<T::Domain> {
        &self.clear_register
    }

    pub fn secret_register_mut(&mut self) -> &mut Register<Share<T::Domain>> {
        &mut self.secret_register
    }

    pub fn secret_register(&self) -> &Register<Share<T::Domain>> {
        &self.secret_register
    }
}

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
