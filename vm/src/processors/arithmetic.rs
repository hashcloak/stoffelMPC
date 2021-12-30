use super::{Processor, Register, StackRegister};
use types::numbers::{MPCType};

pub struct ArithmeticProcessor {
    // Registers
    reg_cint_mod_p: Register,
    reg_register_int: Register,
    reg_sint_mod_p: Register,
    reg_sregister_int: Register,

    // Stacks
    stack_cint_mod_p: StackRegister,
    stack_register_int: StackRegister,
    reg_sint_mod_p: StackRegister,
    reg_sregister_int: StackRegister

}

impl Processor for ArithmeticProcessor {
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

impl ArithmeticProcessor {

}