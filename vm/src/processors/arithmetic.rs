use super::Processor;
use crate::state::{Register, StackRegister};
use types::numbers::{
    int::{PubInt, SecInt},
    secret_sharing::SecretSharing,
    Number,
};

pub struct ArithmeticProcessor<T: Number + SecretSharing> {
    // Registers
    reg_pub_int: Register<PubInt>,
    reg_sec_int: Register<SecInt<T>>,

    // Stacks
    stack_pub_int: StackRegister<PubInt>,
    stack_sec_int: StackRegister<SecInt<T>>,
}

impl<T: Number + SecretSharing> Processor for ArithmeticProcessor<T> {
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

impl<T: Number + SecretSharing> ArithmeticProcessor<T> {}
