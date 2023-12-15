use super::Processor;
use crate::state::{Register, StackRegister};
use crate::Program;
use mpc::protocols::MPCProtocol;
use mpc::share::Share;
use types::vm::RegisterAddr;

#[derive(Debug, Clone, Default)]
pub struct ArithmeticCore<T: MPCProtocol> {
    /// Stack for the secret values.
    secret_stack: StackRegister<Share<T::Domain>>,
    /// Stack for the clear values.
    clear_stack: StackRegister<T::Domain>,
    /// Stack for register addresses.
    reg_addr_stack: StackRegister<RegisterAddr>,

    /// Register for secret values.
    secret_register: Register<Share<T::Domain>>,
    /// Register for clear values.
    clear_register: Register<T::Domain>,
    /// Register for register addresses.
    reg_addr_register: Register<RegisterAddr>,

    /// Program counter for the core.
    program_counter: usize,
}

impl<T: MPCProtocol> ArithmeticCore<T> {
    /// Creates an arithmetic processor with program counter in zero.
    pub fn new() -> ArithmeticCore<T> {
        ArithmeticCore {
            // Construction of stacks
            secret_stack: StackRegister::new(),
            clear_stack: StackRegister::new(),
            reg_addr_stack: StackRegister::new(),

            /// Construction of registers
            secret_register: Register::new(),
            clear_register: Register::new(),
            reg_addr_register: Register::new(),

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

    pub fn reg_addr_register(&self) -> &Register<RegisterAddr> {
        &self.reg_addr_register
    }

    pub fn reg_addr_register_mut(&mut self) -> &mut Register<RegisterAddr> {
        &mut self.reg_addr_register
    }

    pub fn secret_stack(&self) -> &StackRegister<Share<T::Domain>> {
        &self.secret_stack
    }

    pub fn clear_stack(&self) -> &StackRegister<T::Domain> {
        &self.clear_stack
    }

    pub fn reg_addr_stack(&self) -> &StackRegister<RegisterAddr> {
        &self.reg_addr_stack
    }
}

impl<T: MPCProtocol> Processor for ArithmeticCore<T> {
    /// Cleans both the clear and secret registers.
    fn clean_registers(&mut self) {
        self.clear_register.clean();
        self.secret_register.clean();
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

    fn relative_jump(&mut self, n_positions: usize) {
        self.program_counter += n_positions;
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
