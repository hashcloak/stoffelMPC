use crate::processors::boolean::BooleanProcessor;
use types::numbers::Number;

// Copy private input to secret bit register vectors
pub fn inputb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy private input secret bit registers bit by bit
pub fn inputbvec<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn ldmsd<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn stmsd<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn ldmsdi<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn stmsdi<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn stmsdci<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise XOR of secret bit register
pub fn xors<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise XOR of two single clear bit registers
pub fn xorcb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise XOR of single clear bit register and immediate
pub fn xorcbi<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise NOT of secret register vector
pub fn nots<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise NOT of secret register vector
pub fn notcb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Constant-vector AND of secret bit registeers
pub fn andrs<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise AND of secret bit register vector
pub fn ands<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Integer addition two single clear bit registers
pub fn addcb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Integer addition single clear bit register and immediate
pub fn addcbi<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Integer multiplication single clear bit register and immediate
pub fn mulcbi<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Secret bit register decomposition
pub fn bitdecs<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Secret bit register decomposition
pub fn bitcoms<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Secret bit register decomposition
pub fn bitdecc<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Right shift of clear bit register by immediate
pub fn shrcbi<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Left shift of clear bit register by immediate
pub fn shlcbi<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Store immediate in secret bit register
pub fn ldbits<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy secret bit memory cell with compile-time address to secret bit register
pub fn ldmsb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy secret bit register to secret bit memory cell with compile-time address
pub fn stmsb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy secret bit memory cell with run-time address to secret bit register
pub fn ldmsbi<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy secret bit register to secret bit memory cell with run-time address
pub fn stmsbi<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear bit memory cell with compile-time address to clear bit register
pub fn ldmcb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear bit register to cleare bit memory cell with compile-time address
pub fn stmcb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn movsb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Secret bit register vector transpose
pub fn trans<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy fresh secret random bit to secret bit register
pub fn bitb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Reveal secret bit register vectors and copy result to clear bit register vectors
pub fn reveal<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Signed output of clear bit register
pub fn printregsigned<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Debug output of clear bit register
pub fn printregb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Output clear bit register
pub fn printregplainb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Output floating-number from clear bit registers
pub fn printfloatplainb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Conditionally output four bytes
pub fn condprintstrb<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Bitwise AND of single secret and clear bit registers
pub fn andm<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear integer register to secret bit register
pub fn convsint<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear integer register to clear bit register
pub fn convcint<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear register vector by bit to clear bit register vectors
pub fn convcintvec<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear bit register to clear integer register
pub fn convcbit<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

pub fn convcbitvec<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Copy clear bit register vector to secret bit register vector
pub fn convcbit2s<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Store fresh random daBits in secret register
// @dev: Do we need to keep this?
pub fn dabit<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Store frehs random loose edaBits in secret register
// @dev: Do we need to keep this?
pub fn edabit<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Store fresh random strict edaBits in secret register
pub fn sedabit<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}

// Local share conversion
pub fn split<T: Number>(bp: &mut BooleanProcessor<T>) {
    todo!();
}
