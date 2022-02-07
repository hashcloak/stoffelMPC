use super::processors::{ArithmeticProcessor, BooleanProcessor, Processor};
use super::program::Program;
use super::state::GlobalMemory;
use mpc::protocols::hbmpc::HoneyBadgerMPC;
use mpc::protocols::MPCProtocol;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use types::numbers::int::SecInt;
use types::numbers::Number;

#[derive(Debug, Clone, Copy, Default)]
pub enum VMMode {
    #[default]
    Normal,
    Debug,
    Optimized,
}

#[derive(Debug)]
pub struct StoffelVM<T: Number, U: Number, const M: usize, const N: usize> {
    processors: Vec<Box<dyn Processor<Memory = GlobalMemory<T, U, N>>>>,
    program_counter: usize,
    global_memory: Arc<Mutex<GlobalMemory<T, U, N>>>,
    mode: VMMode,
    code: Program,
}

impl<T: Number, U: Number, const M: usize, const N: usize> StoffelVM<T, U, M, N> {
    pub fn new() -> Self {
        Self {
            processors: vec![
                Box::new(ArithmeticProcessor::<T, U, M, N>::default()),
                Box::new(BooleanProcessor::<T, U, M, N>::default()),
            ],
            program_counter: 0,
            global_memory: Arc::new(Mutex::new(GlobalMemory::<T, U, N>::default())),
            mode: VMMode::default(),
            code: Program::new(),
        }
    }

    pub fn load_byte_code(&mut self, bytes: impl AsRef<[u8]>) {
        self.code.parse_bytes(bytes.as_ref())
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

struct NewStoffelVM<T: MPCProtocol<U>, U: Number, V: Number> {
    t: PhantomData<T>,
    u: PhantomData<U>,
    v: PhantomData<V>,
}

// Implement methods for ALL protocols over ALL possible number traits
impl<T: MPCProtocol<U>, U: Number, V: Number> NewStoffelVM<T, U, V> {}

// Implement methods for a single protocol and for all all numbers the protocol supports
impl<U: Number, V: Number> NewStoffelVM<HoneyBadgerMPC, U, V> where HoneyBadgerMPC: MPCProtocol<U> {}

// Implement methods for all protcols using a single secret number
impl<T: MPCProtocol<SecInt<U>>, U: Number, V: Number> NewStoffelVM<T, SecInt<U>, V> {}

// Implement methods for a single number type of a single protocol
impl<T: Number, V: Number> NewStoffelVM<HoneyBadgerMPC, SecInt<T>, V> {}
