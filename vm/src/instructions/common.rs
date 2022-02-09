//Instructions that are common to both GC and Arithmetic circuits

use mpc::protocols::MPCProtocol;
use types::numbers::Number;

use crate::processor::Core;

// Output a single byte
pub fn printchr<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Output four bytes
pub fn printstr<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Set number of digits after decimal point for PRINTFLOATPLAIN
// @dev: Do we need this?
pub fn printfloatprec<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Store immediate value in clear integer register
pub fn ldint<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Clear integer register addition
pub fn addint<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Clera integer register subtraction
pub fn subint<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Clear integer register multiplication
pub fn mulint<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Clear integer register division with floor rounding
pub fn divint<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn jmp<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Conditional relative jump in the bytecode
// NZ = Not Zero
pub fn jmpnz<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Conditional relative jump in the bytecode
pub fn jmpeqz<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Clear integer zero test
pub fn eqzc<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Clear integer less than zero test
pub fn ltzc<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Clear integer less than comparison
pub fn ltc<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Clear integer greater than comparison
pub fn gtc<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Clear integer equality test
pub fn eqc<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn jmpi<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immediate address
pub fn ldmint<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immeiate address
pub fn stmint<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by register address
pub fn ldminti<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Assign clear integer to clear integer memory value(s) by register address
pub fn stminti<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Pushes clear integer register to the thread-local stack
pub fn pushint<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Pops from the thread-local stack to clear integer
pub fn popint<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Copy clear integer register
pub fn movint<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Clear integer bit decomposition
pub fn bitdecint<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Store the argument passed to the current thread in clear integer register
pub fn ldarg<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Copy clear integer register to the thread argument
pub fn starg<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Output time since start of computation
pub fn time<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Start timer
pub fn start<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Stop timer
pub fn stop<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

pub fn gldms<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Assign clear memory value(s) to clear register by immediate address
pub fn ldmc<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Assign secret memory value(s) to secret register by immediate address
pub fn ldms<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Output clear integer register
// @dev: Should we keep this?
pub fn printint<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

pub fn gldmc<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Start tape/bytecode file in another thread
pub fn run_tape<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Join thread
pub fn join_tape<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Crash runtime if the register's value is > 0
pub fn crash<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Offline data usage
pub fn use_off<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Input usage
pub fn use_inp<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Store number of players in clear integer register
pub fn nplayers<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Store maximal number of corrupt players in clear integer register
pub fn threshold<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}

// Store current player number in clear integer register
pub fn playerid<T: MPCProtocol>(processor: &mut Core<T>) {
    todo!();
}
