use crate::processors::boolean::BooleanProcessor;
use types::numbers::{Number, SecretSharing};

// Copy private input to secret bit register vectors
pub fn inputb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy private input secret bit registers bit by bit
pub fn inputbvec<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

pub fn ldmsd<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

pub fn stmsd<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

pub fn ldmsdi<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

pub fn stmsdi<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

pub fn stmsdci<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Bitwise XOR of secret bit register
pub fn xors<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Bitwise XOR of two single clear bit registers
pub fn xorcb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Bitwise XOR of single clear bit register and immediate
pub fn xorcbi<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Bitwise NOT of secret register vector
pub fn nots<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Bitwise NOT of secret register vector
pub fn notcb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Constant-vector AND of secret bit registeers
pub fn andrs<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Bitwise AND of secret bit register vector
pub fn ands<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Integer addition two single clear bit registers
pub fn addcb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Integer addition single clear bit register and immediate
pub fn addcbi<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Integer multiplication single clear bit register and immediate
pub fn mulcbi<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Secret bit register decomposition
pub fn bitdecs<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Secret bit register decomposition
pub fn bitcoms<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Secret bit register decomposition
pub fn bitdecc<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Right shift of clear bit register by immediate
pub fn shrcbi<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Left shift of clear bit register by immediate
pub fn shlcbi<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Store immediate in secret bit register
pub fn ldbits<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy secret bit memory cell with compile-time address to secret bit register
pub fn ldmsb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy secret bit register to secret bit memory cell with compile-time address
pub fn stmsb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy secret bit memory cell with run-time address to secret bit register
pub fn ldmsbi<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy secret bit register to secret bit memory cell with run-time address
pub fn stmsbi<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy clear bit memory cell with compile-time address to clear bit register
pub fn ldmcb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy clear bit register to cleare bit memory cell with compile-time address
pub fn stmcb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

pub fn movsb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Secret bit register vector transpose
pub fn trans<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy fresh secret random bit to secret bit register
pub fn bitb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Reveal secret bit register vectors and copy result to clear bit register vectors
pub fn reveal<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Signed output of clear bit register
pub fn printregsigned<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Debug output of clear bit register
pub fn printregb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Output clear bit register
pub fn printregplainb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Output floating-number from clear bit registers
pub fn printfloatplainb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Conditionally output four bytes
pub fn condprintstrb<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Bitwise AND of single secret and clear bit registers
pub fn andm<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy clear integer register to secret bit register
pub fn convsint<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy clear integer register to clear bit register
pub fn convcint<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy clear register vector by bit to clear bit register vectors
pub fn convcintvec<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy clear bit register to clear integer register
pub fn convcbit<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

pub fn convcbitvec<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Copy clear bit register vector to secret bit register vector
pub fn convcbit2s<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Store fresh random daBits in secret register
// @dev: Do we need to keep this?
pub fn dabit<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Store frehs random loose edaBits in secret register
// @dev: Do we need to keep this?
pub fn edabit<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Store fresh random strict edaBits in secret register
pub fn sedabit<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}

// Local share conversion
pub fn split<T: Number + SecretSharing, U: Number, const N: usize>(
    bp: &mut BooleanProcessor<T, U, N>,
) {
    todo!();
}
