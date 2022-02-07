/// A Processor is component of the MPC VM that handles the core processing within the MPC VM
pub mod arithmetic;
pub mod boolean;

use std::sync::{Arc, Mutex};

pub use arithmetic::ArithmeticProcessor;
pub use boolean::BooleanProcessor;
use mpc::protocols::{hbmpc::HoneyBadgerMPC, MPCProtocol};
use types::numbers::{gf2::SecGf2, int::SecInt, Number};

use crate::state::{Memory, Register, StackRegister};

pub trait Processor: std::fmt::Debug {
    type Memory;

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
    fn memory(&self) -> Self::Memory;
}

pub struct NewProcessor<T: MPCProtocol<U>, U: Number, V: Number> {
    secret_stack: StackRegister<U>,
    public_stack: StackRegister<T::Public<V>>,
    secret_register: Register<U>,
    public_register: Register<T::Public<V>>,
    secret_memory: Arc<Mutex<Memory<U>>>,
    public_memory: Arc<Mutex<Memory<T::Public<V>>>>,
}

impl<T: MPCProtocol<U>, U: Number, V: Number> NewProcessor<T, U, V> {
    fn clear_registers(&mut self) {
        todo!();
    }

    fn get_program_counter(&self) -> usize {
        todo!();
    }

    fn get_program_size(&self) -> usize {
        todo!();
    }
    fn execute(&mut self) {
        todo!();
    }
    fn jump(&mut self) {
        todo!();
    }
    fn relative_jump(&mut self) {
        todo!();
    }
    fn increment_program_counter(&mut self) {
        todo!();
    }
    fn read_tape(&self) {
        todo!();
    }
    fn receive_tape(&self) {
        todo!();
    }
    fn receive_private_input(&self, to_store_in_memory: bool) {
        todo!();
    }
    fn give_private_output(&self, to_store_in_memory: bool) {
        todo!();
    }

    // opcodes applicable to all protocols and all types
}

impl<T: MPCProtocol<SecGf2<U>>, U: Number, V: Number> NewProcessor<T, SecGf2<U>, V> {
    // opcodes applicable to all protocols which use SecGf2 as their secret type
}

impl<U: Number, V: Number> NewProcessor<HoneyBadgerMPC, U, V>
where
    HoneyBadgerMPC: MPCProtocol<U>,
{
    // opcodes applicable for all number types for which HoneybadgerMPC is defined
}

impl<T: MPCProtocol<SecInt<U>>, U: Number, V: Number> NewProcessor<T, SecInt<U>, V> {}
