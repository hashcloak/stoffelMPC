use super::instructions::opcodes::Opcodes;

#[derive(Debug)]
pub struct Program(Vec<Opcodes>);

impl Program {
    pub fn new() -> Self {
        Program(vec![])
    }

    pub fn parse_bytes(&mut self, bytes: impl AsRef<[u8]>) {
        todo!();
    }

    fn execute() {
        todo!();
    }
}
