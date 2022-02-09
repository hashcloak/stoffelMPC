use crate::state::{Memory, Register, StackRegister};
use mpc::protocols::{honey_badger::HoneyBadgerMPC, MPCProtocol};
use std::sync::{Arc, Mutex};
use types::numbers::Number;

pub trait Processor: std::fmt::Debug {
    fn clear_registers(&mut self);
    fn get_program_counter(&self) -> usize;
    fn get_program_size(&self) -> usize;
    fn execute(&mut self);
    fn jump(&mut self);
    fn relative_jump(&mut self);
    fn increment_program_counter(&mut self);
    fn read_tape(&self);
    fn receive_tape(&self);
    fn receive_private_input(&self, to_store_in_memory: bool);
    fn give_private_output(&self, to_store_in_memory: bool);
}

#[derive(Debug, Clone, Default)]
pub struct Core<T: MPCProtocol> {
    secret_stack: StackRegister<T::Secret>,
    public_stack: StackRegister<T::Public>,
    secret_register: Register<T::Secret>,
    public_register: Register<T::Public>,
    secret_memory: Arc<Mutex<Memory<T::Secret>>>,
    public_memory: Arc<Mutex<Memory<T::Public>>>,
}

impl<T: MPCProtocol> Processor for Core<T> {
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
}

impl<T: MPCProtocol> Core<T> {
    // opcodes applicable to all protocols and all types
}

impl<T: Number, U: Number> Core<HoneyBadgerMPC<T, U>> {
    // opcodes applicable to Honeybadger protocol
}
