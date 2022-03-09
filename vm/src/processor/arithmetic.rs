use super::Processor;
use crate::state::{Memory, Register, StackRegister};
use mpc::protocols::MPCProtocol;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct ArithmeticCore<T: MPCProtocol> {
    secret_stack: StackRegister<T::Secret>,
    public_stack: StackRegister<T::Public>,
    secret_register: Register<T::Secret>,
    public_register: Register<T::Public>,
    secret_memory: Arc<Mutex<Memory<T::Secret>>>,
    public_memory: Arc<Mutex<Memory<T::Public>>>,
}

impl<T: MPCProtocol> ArithmeticCore<T> {}

impl<T: MPCProtocol> Processor for ArithmeticCore<T> {}
