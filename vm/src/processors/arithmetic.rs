use types::numbers::{Number, SecretSharing};

use super::Processor;
use crate::state::{GlobalMemory, Registers, StackRegisters};

pub struct ArithmeticProcessor<T: Number + SecretSharing, U: Number, const N: usize> {
    pub stack: StackRegisters<T, U>,
    pub registers: Registers<T, U, N>,
}

impl<T: Number + SecretSharing, U: Number, const N: usize> Processor
    for ArithmeticProcessor<T, U, N>
{
    type Memory = GlobalMemory<T, U>;

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

    fn memory(&self) -> Self::Memory {
        todo!()
    }
}

impl<T: Number + SecretSharing, U: Number, const N: usize> ArithmeticProcessor<T, U, N> {}
