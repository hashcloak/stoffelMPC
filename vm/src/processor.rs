use crate::state::{Memory, Register, StackRegister};
use mpc::protocols::{hbmpc::HoneyBadgerMPC, MPCProtocol};
use std::sync::{Arc, Mutex};
use types::numbers::{gf2::SecGf2, int::SecInt, Number};

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
pub struct Core<T: MPCProtocol<U>, U: Number, V: Number> {
    secret_stack: StackRegister<U>,
    public_stack: StackRegister<T::Public<V>>,
    secret_register: Register<U>,
    public_register: Register<T::Public<V>>,
    secret_memory: Arc<Mutex<Memory<U>>>,
    public_memory: Arc<Mutex<Memory<T::Public<V>>>>,
}

impl<T: MPCProtocol<U>, U: Number, V: Number> Processor for Core<T, U, V> {
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

impl<T: MPCProtocol<U>, U: Number, V: Number> Core<T, U, V> {
    // opcodes applicable to all protocols and all types
}

impl<T: MPCProtocol<SecGf2<U>>, U: Number, V: Number> Core<T, SecGf2<U>, V> {
    // opcodes applicable to all protocols which use SecGf2 as their secret type
}

impl<U: Number, V: Number> Core<HoneyBadgerMPC, U, V>
where
    HoneyBadgerMPC: MPCProtocol<U>,
{
    // opcodes applicable for all number types for which HoneybadgerMPC is defined
}

impl<T: MPCProtocol<SecInt<U>>, U: Number, V: Number> Core<T, SecInt<U>, V> {}
