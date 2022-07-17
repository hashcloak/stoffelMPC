use super::instructions::Instruction;
use super::processor::Processor;

#[derive(Debug, Default)]
pub struct Program<P: Processor>(pub Vec<Instruction<P>>);

impl<P: Processor> Program<P> {
    pub fn new() -> Self {
        Program(vec![])
    }

    pub fn parse_bytes(&mut self, bytes: impl AsRef<[u8]>) {
        todo!();
    }

    fn execute(processor: P, instr_index: usize) {
        todo!();
    }
}
