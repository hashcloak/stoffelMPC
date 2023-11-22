use mpc::protocols::MPCProtocol;

use super::processor::Processor;
use super::schedule::Schedule;
use super::state::Memory;

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
    /// Schedule for current execution. It contains the programs that will be 
    /// executed during the session.
    scheduler: Schedule,
    /// Global memory of the VM.
    memory: Memory<U>,
}

impl<T: Processor, U: MPCProtocol> StoffelVM<T, U> {
    /// Creates a new virual machine.
    pub fn new() -> Self {
        Self {
            processors: vec![],
            program_counter: 0,
            mode: VMMode::default(),
            scheduler: Schedule::new(),
            memory: Memory::new(),
        }
    }

    /// Loads and parses bytecode.
    pub fn load_byte_code(&mut self, bytes: impl AsRef<[u8]>) {
        todo!();
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
