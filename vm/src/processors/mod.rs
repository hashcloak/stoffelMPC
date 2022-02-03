/// A Processor is component of the MPC VM that handles the core processing within the MPC VM
pub mod arithmetic;
pub mod boolean;

pub use arithmetic::ArithmeticProcessor;
pub use boolean::BooleanProcessor;
use mpc::protocols::{hbmpc::HoneyBadgerMPC, MPCProtocol};
use types::numbers::Number;

use crate::state::StackRegister;

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

pub struct Core<T: MPCProtocol<U>, U: Number, V: Number = U> {
    secret_stack: StackRegister<U>,
    public_stack: StackRegister<T::Public<V>>,
}

impl<T: MPCProtocol<U>, U: Number, V: Number> Core<T, U, V> {
    // opcodes applicable to all protocols and all types
}

impl<U: Number, V: Number> Core<HoneyBadgerMPC, U, V>
where
    HoneyBadgerMPC: MPCProtocol<U>,
{
    // opcodes applicable for all number types for which HoneybadgerMPC is defined
}
