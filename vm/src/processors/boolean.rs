use super::{Processor, Register, StackRegister};
use types::numbers::{MPCType};

pub struct BooleanProcessor {
    // Registers
    reg_cgf2: Register,
    reg_sgf2: Register,
    reg_cgf2n: Register,
    reg_sgf2n: Register,

    // Stack Registers
    stack_cgf2: StackRegister,
    stack_sgf2: StackRegister,
    stack_cgf2n: StackRegister,
    stack_sgf2n: StackRegister
}

impl Processor for BooleanProcessor {
    fn clear_registers() {
        todo!();
    }

    fn get_program_counter() -> uint {
        todo!();
    }

    fn get_program_size() -> uint {
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

impl BooleanProcessor {

}