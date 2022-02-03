use types::numbers::Number;

use super::Processor;
use crate::state::{GlobalMemory, Registers, StackRegisters};

#[derive(Debug, Clone, Default)]
pub struct ArithmeticProcessor<T: Number, U: Number, const M: usize, const N: usize> {
    pub stack: StackRegisters<T, U, N>,
    pub registers: Registers<T, U, M, N>,
}

impl<T: Number, U: Number, const M: usize, const N: usize> Processor
    for ArithmeticProcessor<T, U, M, N>
{
    type Memory = GlobalMemory<T, U, N>;

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

impl<T: Number, U: Number, const M: usize, const N: usize> ArithmeticProcessor<T, U, M, N> {}
