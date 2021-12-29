/// A Processor is component of the MPC VM that handles the core processing within the MPC VM
use super::state::{Register, Memory, StackRegister};


trait Processor {
    fn clear_registers();
    fn get_program_counter() -> usize;
    fn get_program_size() -> usize;
    fn execute();
    fn jump();
    fn relative_jump();
    fn increment_program_counter();   
}