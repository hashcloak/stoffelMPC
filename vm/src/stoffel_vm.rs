use types::numbers::secret_sharing::SecretSharing;
use types::numbers::{
    MPCType,
    fixed::{PubFixed, SecFixed},
    gf2::{PubGf2, SecGf2},
    int::{PubInt, SecInt},
};

use super::processors::{Processor, ArithmeticProcessor, BooleanProcessor};
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

pub struct StoffelVM<P: Processor> {
    processors: Vec<P>,
    program_counter: usize,
    mode: VMMode
}

pub struct ArithmeticStoffelVM {
    processors: Vec<ArithmeticProcessor>,
    program_counter: usize,
    mode: VMMode,
}

pub struct BooleanStoffelVM {
    processors: Vec<BooleanProcessor>,
    program_counter: usize,
    mode: VMMode,
}

impl StoffelVM<P: Processor> for StoffelVM<P> {
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

impl StoffelVM for ArithmeticStoffelVM {
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

impl StoffelVM for BooleanStoffelVM {
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