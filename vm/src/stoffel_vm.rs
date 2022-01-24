#![feature(associated_type_bounds)]
use types::numbers::secret_sharing::SecretSharing;
use types::numbers::{
    fixed::{PubFixed, SecFixed},
    gf2::{PubGf2, SecGf2},
    int::{PubInt, SecInt},
    Number,
};

use super::processors::{arithmetic::ArithmeticProcessor, boolean::BooleanProcessor, Processor};
use super::state::{GlobalMemory, Memory, Register, StackRegister};

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
    Optimized,
}

pub struct ArithmeticStoffelVM<
    T: Number,
    Fr: PrimeField + SquareRootField,
    const N: usize,
    const M: usize,
> {
    processors: Vec<ArithmeticProcessor<T>>,
    program_counter: usize,
    global_memory: GlobalMemory<Fr, N, M>,
    mode: VMMode,
}

pub struct BooleanStoffelVM<
    T: Number,
    Fr: PrimeField + SquareRootField,
    const N: usize,
    const M: usize,
> {
    processors: Vec<BooleanProcessor<T>>,
    program_counter: usize,
    global_memory: GlobalMemory<Fr, N, M>,
    mode: VMMode,
}

impl<T: Number, const N: usize, const M: usize, Fr: PrimeField + SquareRootField> StoffelVM
    for ArithmeticStoffelVM<T, Fr, N, M>
{
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

impl<T: Number, const N: usize, const M: usize, Fr: PrimeField + SquareRootField> StoffelVM
    for BooleanStoffelVM<T, Fr, N, M>
{
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
