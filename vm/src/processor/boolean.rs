use super::Processor;
use crate::state::{Memory, Register, StackRegister};
use mpc::protocols::MPCProtocol;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct BooleanCore<T: MPCProtocol> {
    secret_stack: StackRegister<T::VmType>,
    public_stack: StackRegister<T::VmType>,
    secret_register: Register<T::VmType>,
    public_register: Register<T::VmType>,
    secret_memory: Arc<Mutex<Memory<T::VmType>>>,
    public_memory: Arc<Mutex<Memory<T::VmType>>>,
}

impl<T: MPCProtocol> BooleanCore<T> {}

impl<T: MPCProtocol> Processor for BooleanCore<T> {}
