use super::instructions::opcodes::Opcode;
use super::instructions::{arithmetic, boolean, common};
use super::processor::Processor;
use super::program::Program;

#[derive(Debug, Clone, Copy, Default)]
pub enum VMMode {
    #[default]
    Normal,
    Debug,
    Optimized,
}

#[derive(Debug)]
pub struct StoffelVM<T: Processor> {
    processors: Vec<T>,
    program_counter: usize,
    mode: VMMode,
    code: Program<T>,
}

impl<T: Processor> StoffelVM<T> {
    pub fn new() -> Self {
        Self {
            processors: vec![],
            program_counter: 0,
            mode: VMMode::default(),
            code: Program::new(),
        }
    }

    pub fn load_byte_code(&mut self, bytes: impl AsRef<[u8]>) {
        self.code.parse_bytes(bytes.as_ref())
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>>{
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
        let _vm = StoffelVM::<ArithmeticCore<HoneyBadgerMPC>>::new();
    }
}
