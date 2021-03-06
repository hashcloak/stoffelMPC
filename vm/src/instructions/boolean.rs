use crate::processor::boolean::BooleanCore;
use mpc::protocols::MPCProtocol;

// Copy private input to secret bit register vectors
pub fn inputb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy private input secret bit registers bit by bit
pub fn inputbvec<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

pub fn ldmsd<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

pub fn stmsd<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

pub fn ldmsdi<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

pub fn stmsdi<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

pub fn stmsdci<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Bitwise XOR of secret bit register
pub fn xors<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Bitwise XOR of two single clear bit registers
pub fn xorcb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Bitwise XOR of single clear bit register and immediate
pub fn xorcbi<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Bitwise NOT of secret register vector
pub fn nots<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Bitwise NOT of secret register vector
pub fn notcb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Constant-vector AND of secret bit registeers
pub fn andrs<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Bitwise AND of secret bit register vector
pub fn ands<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Integer addition two single clear bit registers
pub fn addcb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Integer addition single clear bit register and immediate
pub fn addcbi<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Integer multiplication single clear bit register and immediate
pub fn mulcbi<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Secret bit register decomposition
pub fn bitdecs<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Secret bit register decomposition
pub fn bitcoms<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Secret bit register decomposition
pub fn bitdecc<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Right shift of clear bit register by immediate
pub fn shrcbi<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Left shift of clear bit register by immediate
pub fn shlcbi<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Store immediate in secret bit register
pub fn ldbits<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy secret bit memory cell with compile-time address to secret bit register
pub fn ldmsb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy secret bit register to secret bit memory cell with compile-time address
pub fn stmsb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy secret bit memory cell with run-time address to secret bit register
pub fn ldmsbi<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy secret bit register to secret bit memory cell with run-time address
pub fn stmsbi<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy clear bit memory cell with compile-time address to clear bit register
pub fn ldmcb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy clear bit register to cleare bit memory cell with compile-time address
pub fn stmcb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

pub fn movsb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Secret bit register vector transpose
pub fn trans<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy fresh secret random bit to secret bit register
pub fn bitb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Reveal secret bit register vectors and copy result to clear bit register vectors
pub fn reveal<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Signed output of clear bit register
pub fn printregsigned<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Debug output of clear bit register
pub fn printregb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Output clear bit register
pub fn printregplainb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Output floating-number from clear bit registers
pub fn printfloatplainb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Conditionally output four bytes
pub fn condprintstrb<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Bitwise AND of single secret and clear bit registers
pub fn andm<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy clear integer register to secret bit register
pub fn convsint<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy clear integer register to clear bit register
pub fn convcint<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy clear register vector by bit to clear bit register vectors
pub fn convcintvec<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy clear bit register to clear integer register
pub fn convcbit<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

pub fn convcbitvec<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Copy clear bit register vector to secret bit register vector
pub fn convcbit2s<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Store fresh random daBits in secret register
// @dev: Do we need to keep this?
pub fn dabit<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Store frehs random loose edaBits in secret register
// @dev: Do we need to keep this?
pub fn edabit<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Store fresh random strict edaBits in secret register
pub fn sedabit<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}

// Local share conversion
pub fn split<T: MPCProtocol>(processor: &mut BooleanCore<T>) {
    todo!();
}
