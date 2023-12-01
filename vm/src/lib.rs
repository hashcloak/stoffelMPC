/// A module to collect all the different instruction sets
pub mod instructions;
/// A processor is component of the stoffelVM that handles the core processing within the MPC VM
pub mod processor;
/// A module for handling StoffelMPC programs
pub mod program;
/// A scheduler that specifies which take (program) is being executed
pub mod schedule;

pub use program::Program;
/// Collects structs to manage state like registers and so on
pub mod state;
/// The vm implementation
pub mod stoffel_vm;

pub use stoffel_vm::StoffelVM;
