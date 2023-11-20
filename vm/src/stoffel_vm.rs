use std::sync::Arc;

use futures::lock::Mutex;
use mpc::protocols::MPCProtocol;

use super::processor::Processor;
use super::program::Program;
use super::state::Memory;
use mpc::share::Share;

#[derive(Debug, Clone, Copy, Default)]
pub enum VMMode {
    #[default]
    Normal,
    Debug,
    Optimized,
}

#[derive(Debug)]
pub struct StoffelVM<T: Processor, U: MPCProtocol> {
    /// Processors in the virtual machine.
    processors: Vec<T>,
    /// Program counter of the virtual machine.
    program_counter: usize,
    /// Mode of execution of the virtual machine.
    mode: VMMode,
    /// Program that will be executed by the virtual machine.
    code: Program<T>,
    /// Global memory for secret arithmetic values.
    arith_sec_memory: Arc<Mutex<Memory<Share<U::Domain>>>>,
    /// Global memory for public arithmetic values.
    arith_pub_memory: Arc<Mutex<Memory<U::Domain>>>,
    /// Global memory for public register integers.
    arith_reg_memory: Arc<Mutex<Memory<U::Domain>>>,
}

impl<T: Processor, U: MPCProtocol> StoffelVM<T, U> {
    pub fn new() -> Self {
        Self {
            processors: vec![],
            program_counter: 0,
            mode: VMMode::default(),
            code: Program::new(),
            arith_pub_memory: Arc::new(Mutex::new(Memory::new())),
            arith_sec_memory: Arc::new(Mutex::new(Memory::new())),
            arith_reg_memory: Arc::new(Mutex::new(Memory::new())),
        }
    }

    pub fn load_byte_code(&mut self, bytes: impl AsRef<[u8]>) {
        self.code.parse_bytes(bytes.as_ref())
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

    pub fn execute(&mut self) -> i32 {
        todo!();
    }

    pub fn load_schedule() {
        todo!();
    }

    pub fn set_mode(&mut self, mode: VMMode) {
        todo!();
    }

    pub fn start_timer() {
        todo!();
    }

    pub fn stop_timer() {
        todo!();
    }

    pub fn get_current_program_counter(&self) -> usize {
        self.program_counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::arithmetic::ArithmeticCore;
    use mpc::protocols::honey_badger::HoneyBadgerMPC;

    #[test]
    fn test_vm_new() {
        let _vm = StoffelVM::<ArithmeticCore<HoneyBadgerMPC>, HoneyBadgerMPC>::new();
    }
}
