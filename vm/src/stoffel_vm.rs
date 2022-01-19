#![feature(associated_type_bounds)]
use types::numbers::secret_sharing::SecretSharing;
use types::numbers::{
    MPCType,
    fixed::{PubFixed, SecFixed},
    gf2::{PubGf2, SecGf2},
    int::{PubInt, SecInt},
};

use super::processors::{Processor, arithmetic::ArithmeticProcessor, boolean::BooleanProcessor};
use super::state::{Memory, Register, StackRegister};


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

pub struct ArithmeticStoffelVM<T: MPCType> {
    processors: Vec<ArithmeticProcessor<T>>,
    program_counter: usize,
    mode: VMMode,
}

pub struct BooleanStoffelVM<T: MPCType> {
    processors: Vec<BooleanProcessor<T>>,
    program_counter: usize,
    mode: VMMode,
}

impl<T: MPCType> StoffelVM for ArithmeticStoffelVM<T> {
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

impl<T: MPCType> StoffelVM for BooleanStoffelVM<T> {
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