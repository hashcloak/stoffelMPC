pub mod arithmetic;
pub mod boolean;
pub mod common;
pub mod opcodes;

use crate::state::Memory;

use super::processor::Processor;
use mpc::protocols::MPCProtocol;
use opcodes::{InstructionOpcode, Opcode};
use types::vm::{ImmediateValue, MpcType, RegisterAddr};

#[derive(Debug)]
pub struct Instruction {
    /// Opcode of the instruction
    opcode: Opcode,
    /// Name of the opcode
    opcode_name: InstructionOpcode,
    vector_size: usize,
    instruction_register1: RegisterAddr,
    instruction_register2: RegisterAddr,
    instruction_register3: RegisterAddr,
    instruction_register4: RegisterAddr,
    possible_immediate_value: ImmediateValue,
    // TODO: Change u64 to appropriate mpc type
    variable_params: Vec<u64>,
}

impl Instruction {
    /// Creates a new instruction
    pub fn new() -> Self {
        todo!();
    }

    /// Parses the instruction from a bytecode.
    pub fn parse(&mut self, bytes: impl AsRef<[u8]>) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

    /// Executes an instruction using a processor and a given memory.
    pub fn execute<P, T>(
        processor: &mut P,
        memory: &mut Memory<T>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        P: Processor,
        T: MPCProtocol,
    {
        todo!();
    }
}
