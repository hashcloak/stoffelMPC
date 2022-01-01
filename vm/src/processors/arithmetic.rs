use super::processor::Processor;
use crate::state::{Register, StackRegister};
use types::numbers::{MPCType};

pub struct ArithmeticProcessor<T: MPCType> {
    // Registers
    reg_cint_mod_p: Register<T>,
    reg_register_int: Register<T>,
    reg_sint_mod_p: Register<T>,
    reg_sregister_int: Register<T>,

    // Stacks
    stack_cint_mod_p: StackRegister<T>,
    stack_register_int: StackRegister<T>,
    stack_sint_mod_p: StackRegister<T>,
    stack_sregister_int: StackRegister<T>

}

impl<T: MPCType> Processor for ArithmeticProcessor<T> {
    fn clear_registers() {
        todo!();
    }

    fn get_program_counter() -> usize {
        todo!();
    }

    fn get_program_size() -> usize {
        todo!();
    }

    fn execute() {
        todo!();
    }
    fn jump() {
        todo!();
    }

    fn relative_jump() {
        todo!();
    }

    fn increment_program_counter() {
        todo!();
    }

    fn read_tape() {
        todo!();
    }
    fn receive_tape() {
        todo!();
    }
    fn receive_private_input(to_store_in_memory: bool) {
        todo!();
    }
    fn give_private_output(to_store_in_memory: bool) {
        todo!()
    }

}

impl<T: MPCType> ArithmeticProcessor<T> {

}