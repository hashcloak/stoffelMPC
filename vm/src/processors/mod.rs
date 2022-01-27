/// A Processor is component of the MPC VM that handles the core processing within the MPC VM
pub mod arithmetic;
pub mod boolean;

pub trait Processor {
    type Memory;

    fn clear_registers(&mut self);
    fn get_program_counter(&self) -> usize;
    fn get_program_size(&self) -> usize;
    fn execute(&mut self);
    fn jump(&mut self);
    fn relative_jump(&mut self);
    fn increment_program_counter(&mut self);
    fn read_tape(&self);
    fn receive_tape(&self);
    fn receive_private_input(&self, to_store_in_memory: bool);
    fn give_private_output(&self, to_store_in_memory: bool);
    fn memory(&self) -> Self::Memory;
}
