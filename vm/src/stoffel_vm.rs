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

use ark_ff::fields::{PrimeField, SquareRootField};


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

pub struct ArithmeticStoffelVM<T: MPCType, const N: usize, const M: usize, Fr: PrimeField + SquareRootField> {
    processors: Vec<ArithmeticProcessor<T>>,
    program_counter: usize,
    global_memory: GlobalMemory<N, M, Fr>,
    mode: VMMode,
}

pub struct BooleanStoffelVM<T: MPCType, const N: usize, const M: usize, Fr: PrimeField + SquareRootField> {
    processors: Vec<BooleanProcessor<T>>,
    program_counter: usize,
    global_memory: GlobalMemory<N, M, Fr>,
    mode: VMMode,
}

impl<T: MPCType, const N: usize, const M: usize, Fr: PrimeField + SquareRootField> StoffelVM for ArithmeticStoffelVM<T, N, M, Fr> {
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

impl<T: MPCType, const N: usize, const M: usize, Fr: PrimeField + SquareRootField> StoffelVM for BooleanStoffelVM<T, N, M, Fr> {
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