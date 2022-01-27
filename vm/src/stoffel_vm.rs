use std::sync::{Arc, Mutex};

use types::numbers::{Number, SecretSharing};

use super::processors::Processor;
use super::state::GlobalMemory;

enum VMMode {
    Normal,
    Debug,
    Optimized,
}

pub struct StoffelVM<T: SecretSharing + Number, U: Number> {
    processors: Vec<Box<dyn Processor<Memory = GlobalMemory<T, U>>>>,
    program_counter: usize,
    global_memory: Arc<Mutex<GlobalMemory<T, U>>>,
    mode: VMMode,
}

impl<T: SecretSharing + Number, U: Number> StoffelVM<T, U> {
    fn load_program() {
        todo!();
    }

    fn execute() {
        todo!();
    }

    fn load_schedule() {
        todo!();
    }

    fn set_mode() {
        todo!();
    }

    fn start_timer() {
        todo!();
    }

    fn stop_timer() {
        todo!();
    }

    fn get_current_program_counter() {
        todo!();
    }
}
