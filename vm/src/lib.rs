#![feature(derive_default_enum)]

mod instructions;
mod processors;
mod program;
mod state;
pub mod stoffel_vm;

pub use stoffel_vm::StoffelVM;
