use mpc::protocols::MPCProtocol;

use crate::{state::Register, Program};

pub mod arithmetic;
pub mod boolean;

pub trait Processor: std::fmt::Debug {
    fn clean_registers(&mut self) {
        todo!()
    }

    fn get_program_counter(&self) -> usize {
        todo!()
    }

    fn get_program_size(&self) -> usize {
        todo!()
    }

    fn execute(&mut self, program: Program) {
        todo!()
    }

    fn jump(&mut self, new_program_counter: usize);

    fn relative_jump(&mut self, n_positions: usize);

    fn increment_program_counter(&mut self);

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
