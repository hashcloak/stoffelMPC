use super::processors::{ArithmeticProcessor, BooleanProcessor, Processor};
use super::state::GlobalMemory;
use std::sync::{Arc, Mutex};
use types::numbers::{Number, SecretSharing};

#[derive(Debug, Clone, Copy, Default)]
pub enum VMMode {
    #[default]
    Normal,
    Debug,
    Optimized,
}

#[derive(Debug)]
pub struct StoffelVM<T: SecretSharing + Number, U: Number, const M: usize, const N: usize> {
    processors: Vec<Box<dyn Processor<Memory = GlobalMemory<T, U, N>>>>,
    program_counter: usize,
    global_memory: Arc<Mutex<GlobalMemory<T, U, N>>>,
    mode: VMMode,
    byte_code: Vec<u8>,
}

impl<T: SecretSharing + Number, U: Number, const M: usize, const N: usize> StoffelVM<T, U, M, N> {
    pub fn new_debug() -> Self {
        Self {
            processors: vec![
                Box::new(ArithmeticProcessor::<T, U, M, N>::default()),
                Box::new(BooleanProcessor::<T, U, M, N>::default()),
            ],
            program_counter: 0,
            global_memory: Arc::new(Mutex::new(GlobalMemory::<T, U, N>::default())),
            mode: VMMode::default(),
            byte_code: vec![],
        }
    }

    pub fn load_byte_code(&mut self, bytes: impl AsRef<[u8]>) {
        self.byte_code.extend_from_slice(bytes.as_ref())
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
