#![feature(associated_type_bounds)]
use types::numbers::secret_sharing::SecretSharing;
use types::numbers::{
    MPCType,
    fixed::{PubFixed, SecFixed},
    gf2::{PubGf2, SecGf2},
    int::{PubInt, SecInt},
};

use super::processors::{Processor, arithmetic::ArithmeticProcessor, boolean::BooleanProcessor};
use super::state::{Memory, Register, StackRegister, GlobalMemory};


pub trait StoffelVM {
    fn load_program();

    fn execute();

    fn load_schedule();

    fn set_mode();

    fn start_timer();

    fn stop_timer();

    fn get_current_program_counter();
}

enum VMMode {
    Normal,
    Debug,
    Optimized
}

pub struct ArithmeticStoffelVM<T: MPCType, N: const usize> {
    processors: Vec<ArithmeticProcessor<T>>,
    program_counter: usize,
    global_memory: GlobalMemory<N>
    mode: VMMode,
}

pub struct BooleanStoffelVM<T: MPCType, N: const usize> {
    processors: Vec<BooleanProcessor<T>>,
    program_counter: usize,
    global_memory: GlobalMemory<N>
    mode: VMMode,
}

impl<T: MPCType, N: const usize> StoffelVM for ArithmeticStoffelVM<T, N> {
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

impl<T: MPCType, N: const usize> StoffelVM for BooleanStoffelVM<T, N> {
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