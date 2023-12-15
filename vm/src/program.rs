use super::instructions::Instruction;
use super::processor::Processor;
use crate::state::Memory;
use mpc::protocols::MPCProtocol;

/// Represents a program as a vector of instructions.
#[derive(Debug, Default)]
pub struct Program(Vec<Instruction>);

impl Program {
    /// Creates a new empty program.
    pub fn new() -> Self {
        Program(vec![])
    }

    /// Creates a new program from bytes.
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Self {
        let mut new_program = Self::new();
        new_program.parse_bytes(bytes);
        new_program
    }

    /// Creates a program from a stream of bytes.
    pub fn parse_bytes(&mut self, bytes: impl AsRef<[u8]>) {
        todo!();
    }

    /// Executes the program in a given processor and with access to a given memory.
    fn execute<P, T>(processor: &mut P, memory: &mut Memory<T>)
    where
        P: Processor,
        T: MPCProtocol,
    {
        todo!();
    }

    /// Returns the instructions in the program.
    fn instructions(&self) -> &Vec<Instruction> {
        &self.0
    }
}
