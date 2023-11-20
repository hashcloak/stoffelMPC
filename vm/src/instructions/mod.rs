pub mod arithmetic;
pub mod boolean;
pub mod common;
pub mod opcodes;

use super::processor::Processor;
use opcodes::{InstructionOpcode, Opcode};
use types::vm::{ImmediateValue, RegisterAddr};

#[derive(Debug)]
pub struct Instruction<P: Processor> {
    opcode: Opcode,
    opcode_name: InstructionOpcode,
    processor: P,
    vector_size: usize,
    instruction_register1: RegisterAddr,
    instruction_register2: RegisterAddr,
    instruction_register3: RegisterAddr,
    instruction_register4: RegisterAddr,
    possible_immediate_value: ImmediateValue,
    // TODO: Change u64 to appropriate mpc type
    variable_params: Vec<u64>,
}

impl<P: Processor> Instruction<P> {
    pub fn new() -> Self {
        todo!();
    }

    pub fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

    pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }
}
