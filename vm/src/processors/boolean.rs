use super::Processor;
use crate::state::{Register, StackRegister};
use types::numbers::MPCType;

pub struct BooleanProcessor<T: MPCType> {
    // Registers
    reg_cgf2: Register<T>,
    reg_sgf2: Register<T>,
    reg_cgf2n: Register<T>,
    reg_sgf2n: Register<T>,

    // Stack Registers
    stack_cgf2: StackRegister<T>,
    stack_sgf2: StackRegister<T>,
    stack_cgf2n: StackRegister<T>,
    stack_sgf2n: StackRegister<T>,
}

impl<T: MPCType> Processor for BooleanProcessor<T> {
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

impl<T: MPCType> BooleanProcessor<T> {}
