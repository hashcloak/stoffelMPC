use crate::processors::NewProcessor;
use mpc::protocols::MPCProtocol;
use types::numbers::Number;

// Copy private input to secret bit register vectors
pub fn inputb<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy private input secret bit registers bit by bit
pub fn inputbvec<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn ldmsd<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn stmsd<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn ldmsdi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn stmsdi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn stmsdci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Bitwise XOR of secret bit register
pub fn xors<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Bitwise XOR of two single clear bit registers
pub fn xorcb<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Bitwise XOR of single clear bit register and immediate
pub fn xorcbi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Bitwise NOT of secret register vector
pub fn nots<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Bitwise NOT of secret register vector
pub fn notcb<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Constant-vector AND of secret bit registeers
pub fn andrs<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Bitwise AND of secret bit register vector
pub fn ands<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Integer addition two single clear bit registers
pub fn addcb<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Integer addition single clear bit register and immediate
pub fn addcbi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Integer multiplication single clear bit register and immediate
pub fn mulcbi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Secret bit register decomposition
pub fn bitdecs<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Secret bit register decomposition
pub fn bitcoms<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Secret bit register decomposition
pub fn bitdecc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Right shift of clear bit register by immediate
pub fn shrcbi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Left shift of clear bit register by immediate
pub fn shlcbi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store immediate in secret bit register
pub fn ldbits<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy secret bit memory cell with compile-time address to secret bit register
pub fn ldmsb<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy secret bit register to secret bit memory cell with compile-time address
pub fn stmsb<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy secret bit memory cell with run-time address to secret bit register
pub fn ldmsbi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy secret bit register to secret bit memory cell with run-time address
pub fn stmsbi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear bit memory cell with compile-time address to clear bit register
pub fn ldmcb<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear bit register to cleare bit memory cell with compile-time address
pub fn stmcb<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn movsb<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Secret bit register vector transpose
pub fn trans<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy fresh secret random bit to secret bit register
pub fn bitb<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Reveal secret bit register vectors and copy result to clear bit register vectors
pub fn reveal<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Signed output of clear bit register
pub fn printregsigned<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Debug output of clear bit register
pub fn printregb<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Output clear bit register
pub fn printregplainb<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Output floating-number from clear bit registers
pub fn printfloatplainb<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Conditionally output four bytes
pub fn condprintstrb<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Bitwise AND of single secret and clear bit registers
pub fn andm<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear integer register to secret bit register
pub fn convsint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear integer register to clear bit register
pub fn convcint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear register vector by bit to clear bit register vectors
pub fn convcintvec<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear bit register to clear integer register
pub fn convcbit<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn convcbitvec<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear bit register vector to secret bit register vector
pub fn convcbit2s<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store fresh random daBits in secret register
// @dev: Do we need to keep this?
pub fn dabit<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store frehs random loose edaBits in secret register
// @dev: Do we need to keep this?
pub fn edabit<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store fresh random strict edaBits in secret register
pub fn sedabit<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Local share conversion
pub fn split<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}
