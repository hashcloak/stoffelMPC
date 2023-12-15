use mpc::{protocols::MPCProtocol, share::Share};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};
use types::vm::{MpcType, RegisterAddr};

use crate::error::VmError;

/// Register of the virtual machine that is inside of the processors.
#[derive(Clone, Debug)]
pub struct Register<T: MpcType, const N: usize = 8>([T; N]);

impl<T: MpcType, const N: usize> Default for Register<T, N> {
    fn default() -> Self {
        Self([T::default(); N])
    }
}

impl<T: MpcType, const N: usize> Register<T, N> {
    /// Creates a new register with default values.
    pub fn new() -> Register<T, N> {
        Register([T::default(); N])
    }

    /// Read a given position of the register
    pub fn read(&self, i: RegisterAddr) -> Result<T, VmError> {
        if i < self.0.len() {
            Ok(self.0[i])
        } else {
            Err(VmError::IndexOutOfBound(i, self.0.len()))
        }
    }

    /// Writes a given content in the given address.
    pub fn write(&mut self, i: RegisterAddr, element: T) -> Result<(), VmError> {
        if i < self.0.len() {
            self.0[i] = element;
            Ok(())
        } else {
            Err(VmError::IndexOutOfBound(i, self.0.len()))
        }
    }

    /// Cleans the register.
    pub fn clean(&mut self) {
        self.0 = [T::default(); N]
    }
}

/// Stack for a given MpcType that is inside of the processor.
#[derive(Clone, Debug, Default)]
pub struct StackRegister<T: MpcType>(Vec<T>);

impl<T: MpcType> StackRegister<T> {
    /// Creates a new stack register
    pub fn new() -> StackRegister<T> {
        StackRegister(Vec::new())
    }

    /// Pushes an element in the stack.
    pub fn push(&mut self, element: T) {
        self.0.push(element);
    }

    /// Pops an element from the stack.
    pub fn pop(&mut self) -> T {
        self.0.pop().unwrap()
    }

    /// Peeks an element from the stack at a given location.
    pub fn peek(&self, location: usize) -> Option<&T> {
        self.0.get(location)
    }

    /// Poke an element from the stack.
    pub fn poke(&mut self, location: usize, element: T) -> Result<(), VmError> {
        if location > self.0.len() {
            Err(VmError::IndexOutOfBound(location, self.0.len()))
        } else {
            self.0[location] = element;
            Ok(())
        }
    }

    /// Clears the stack.
    pub fn clear_stack(&mut self) {
        self.0.clear();
    }
}

/// Array that is inside the global memory for each type.
///
/// Recall that the memory stores data for each possible data type. This struct
/// represents each array for each data type in the memory.
#[derive(Clone, Debug, Default)]
pub struct MemoryArray<T: MpcType>(Vec<T>);

impl<T: MpcType> MemoryArray<T> {
    /// Creates an empty memory.
    pub fn new(memory_size: usize) -> Self {
        Self(vec![T::default(); memory_size])
    }

    /// Returns the value in the memory stored at a given index.
    pub fn read(&self, i: usize) -> Result<T, VmError> {
        if i < self.0.len() {
            Ok(self.0[i])
        } else {
            Err(VmError::IndexOutOfBound(i, self.0.len()))
        }
    }

    /// Writes a given value in a given index of the memory.
    pub fn write(&mut self, i: usize, value: T) -> Result<(), VmError> {
        if i < self.0.len() {
            self.0[i] = value;
            Ok(())
        } else {
            Err(VmError::IndexOutOfBound(i, self.0.len()))
        }
    }

    pub fn allocate() {
        todo!();
    }

    pub fn deallocate() {
        todo!();
    }

    pub fn resize() {
        todo!();
    }
}

/// Global memory of the VM.
#[derive(Debug)]
pub struct Memory<P: MPCProtocol> {
    /// Global memory for secret arithmetic values.
    sec_memory: Arc<Mutex<MemoryArray<Share<P::Domain>>>>,
    /// Global memory for public arithmetic values.
    pub_memory: Arc<Mutex<MemoryArray<P::Domain>>>,
    /// Global memory for public register integers.
    reg_memory: Arc<Mutex<MemoryArray<RegisterAddr>>>,
}

impl<P: MPCProtocol> Memory<P> {
    pub fn new(array_size: usize) -> Self {
        Self {
            pub_memory: Arc::new(Mutex::new(MemoryArray::new(array_size))),
            sec_memory: Arc::new(Mutex::new(MemoryArray::new(array_size))),
            reg_memory: Arc::new(Mutex::new(MemoryArray::new(array_size))),
        }
    }

    pub fn sec_memory(&self) -> &Arc<Mutex<MemoryArray<Share<P::Domain>>>> {
        &self.sec_memory
    }

    pub fn pub_memory(&self) -> &Arc<Mutex<MemoryArray<P::Domain>>> {
        &self.pub_memory
    }

    pub fn pub_memory_mut(&mut self) -> &mut Arc<Mutex<MemoryArray<P::Domain>>> {
        &mut self.pub_memory
    }

    pub fn reg_memory(&self) -> &Arc<Mutex<MemoryArray<RegisterAddr>>> {
        &self.reg_memory
    }
}
