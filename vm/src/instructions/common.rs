//Instructions that are common to both GC and Arithmetic circuits

use mpc::protocols::MPCProtocol;
use types::numbers::Number;

use crate::processors::NewProcessor;

// Output a single byte
pub fn printchr<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Output four bytes
pub fn printstr<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Set number of digits after decimal point for PRINTFLOATPLAIN
// @dev: Do we need this?
pub fn printfloatprec<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Store immediate value in clear integer register
pub fn ldint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear integer register addition
pub fn addint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clera integer register subtraction
pub fn subint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear integer register multiplication
pub fn mulint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear integer register division with floor rounding
pub fn divint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn jmp<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Conditional relative jump in the bytecode
// NZ = Not Zero
pub fn jmpnz<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Conditional relative jump in the bytecode
pub fn jmpeqz<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear integer zero test
pub fn eqzc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear integer less than zero test
pub fn ltzc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear integer less than comparison
pub fn ltc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear integer greater than comparison
pub fn gtc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear integer equality test
pub fn eqc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn jmpi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immediate address
pub fn ldmint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immeiate address
pub fn stmint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by register address
pub fn ldminti<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign clear integer to clear integer memory value(s) by register address
pub fn stminti<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Pushes clear integer register to the thread-local stack
pub fn pushint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Pops from the thread-local stack to clear integer
pub fn popint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear integer register
pub fn movint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear integer bit decomposition
pub fn bitdecint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store the argument passed to the current thread in clear integer register
pub fn ldarg<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear integer register to the thread argument
pub fn starg<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Output time since start of computation
pub fn time<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Start timer
pub fn start<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Stop timer
pub fn stop<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gldms<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign clear memory value(s) to clear register by immediate address
pub fn ldmc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign secret memory value(s) to secret register by immediate address
pub fn ldms<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Output clear integer register
// @dev: Should we keep this?
pub fn printint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gldmc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Start tape/bytecode file in another thread
pub fn run_tape<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Join thread
pub fn join_tape<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Crash runtime if the register's value is > 0
pub fn crash<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Offline data usage
pub fn use_off<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Input usage
pub fn use_inp<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store number of players in clear integer register
pub fn nplayers<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store maximal number of corrupt players in clear integer register
pub fn threshold<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store current player number in clear integer register
pub fn playerid<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}
