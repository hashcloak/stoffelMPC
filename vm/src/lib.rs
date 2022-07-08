#![feature(derive_default_enum)]

/// A module to collect all the different instruction sets
mod instructions;
/// A processor is component of the stoffelVM that handles the core processing within the MPC VM
pub mod processor;
/// A module for handling StoffelMPC programs
pub mod program;

pub use program::Program;
/// Collects structs to manage state like registers and so on
mod state;
/// The vm implementation
pub mod stoffel_vm;

pub use stoffel_vm::StoffelVM;
