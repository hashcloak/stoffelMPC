use crate::processor::boolean::BooleanProcessor;
use types::numbers::MPCType;

// Copy private input to secret bit register vectors
pub fn INPUTB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy private input secret bit registers bit by bit
pub fn INPUTBVEC<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn LDMSD<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn STMSD<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn LDMSDI<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn STMSDI<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn STMSDCI<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise XOR of secret bit register
pub fn XORS<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise XOR of two single clear bit registers
pub fn XORCB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise XOR of single clear bit register and immediate
pub fn XORCBI<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise NOT of secret register vector
pub fn NOTS<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise NOT of secret register vector
pub fn NOTCB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Constant-vector AND of secret bit registeers
pub fn ANDRS<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise AND of secret bit register vector
pub fn ANDS<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Integer addition two single clear bit registers
pub fn ADDCB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Integer addition single clear bit register and immediate
pub fn ADDCBI<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Integer multiplication single clear bit register and immediate
pub fn MULCBI<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Secret bit register decomposition
pub fn BITDECS<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Secret bit register decomposition
pub fn BITCOMS<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Secret bit register decomposition
pub fn BITDECC<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Right shift of clear bit register by immediate
pub fn SHRCBI<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Left shift of clear bit register by immediate
pub fn SHLCBI<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Store immediate in secret bit register
pub fn LDBITS<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy secret bit memory cell with compile-time address to secret bit register
pub fn LDMSB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy secret bit register to secret bit memory cell with compile-time address
pub fn STMSB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy secret bit memory cell with run-time address to secret bit register
pub fn LDMSBI<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy secret bit register to secret bit memory cell with run-time address
pub fn STMSBI<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear bit memory cell with compile-time address to clear bit register
pub fn LDMCB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear bit register to cleare bit memory cell with compile-time address
pub fn STMCB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn MOVSB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Secret bit register vector transpose
pub fn TRANS<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy fresh secret random bit to secret bit register
pub fn BITB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Reveal secret bit register vectors and copy result to clear bit register vectors
pub fn REVEAL<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Signed output of clear bit register
pub fn PRINTREGSIGNED<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Debug output of clear bit register
pub fn PRINTREGB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Output clear bit register
pub fn PRINTREGPLAINB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Output floating-number from clear bit registers
pub fn PRINTFLOATPLAINB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Conditionally output four bytes
pub fn CONDPRINTSTRB<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise AND of single secret and clear bit registers
pub fn ANDM<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear integer register to secret bit register
pub fn CONVSINT<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear integer register to clear bit register
pub fn CONVCINT<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear register vector by bit to clear bit register vectors
pub fn CONVCINTVEC<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear bit register to clear integer register
pub fn CONVCBIT<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn CONVCBITVEC<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear bit register vector to secret bit register vector
pub fn CONVCBIT2S<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Store fresh random daBits in secret register
// @dev: Do we need to keep this?
pub fn DABIT<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Store frehs random loose edaBits in secret register
// @dev: Do we need to keep this?
pub fn EDABIT<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Store fresh random strict edaBits in secret register
pub fn SEDABIT<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Local share conversion
pub fn SPLIT<T: MPCType>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

