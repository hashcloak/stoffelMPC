/// A Processor is component of the MPC VM that handles the core processing within the MPC VM
use crate::state::{Memory, Register, StackRegister};

pub mod arithmetic;
pub mod boolean;

pub trait Processor {
    fn clear_registers();
    fn get_program_counter() -> usize;
    fn get_program_size() -> usize;
    fn execute();
    fn jump();
    fn relative_jump();
    fn increment_program_counter();
    fn read_tape();
    fn receive_tape();
    fn receive_private_input(to_store_in_memory: bool);
    fn give_private_output(to_store_in_memory: bool);
}
