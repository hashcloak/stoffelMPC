/// Errors that can occur in the virtual machine.
#[derive(Debug, PartialEq, Clone)]
pub enum VmError {
    /// Error for index out of bounds in data structures. It contains the index
    /// That is trying to access and the length of the data structure.
    IndexOutOfBound(usize, usize),

    /// Error for division by zero in the field.
    DivisionByZero,

    /// Error for invalid instruction. It contains the opcode that is being
    /// accessed wrongly.
    InvalidInstruction(usize),
}

impl std::error::Error for VmError {}

impl std::fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
