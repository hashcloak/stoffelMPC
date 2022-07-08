pub mod arithmetic;
pub mod boolean;
pub mod common;
pub mod opcodes;

use opcodes::{Opcode, InstructionOpcode};
use super::processor::Processor;

#[derive(Debug)]
pub struct Instruction<P: Processor> {
    opcode: Opcode,
    opcode_name: InstructionOpcode,
    processor: P,
}

impl<P: Processor> Instruction<P> {
    pub fn new() -> Self {
        todo!();
    }

    pub fn parse(&mut self) -> Result<(),Box<dyn std::error::Error>>{
        todo!();
    }
}