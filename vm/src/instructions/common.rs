// VM Instructions that are common to both GC and Arithmetic circuits

use crate::processors::Processor;

// Output a single byte
pub fn printchr<P: Processor>(p: &mut P) {
    todo!();
}

// Output four bytes
pub fn printstr<P: Processor>(p: &mut P) {
    todo!();
}

// Set number of digits after decimal point for PRINTFLOATPLAIN
// @dev: Do we need this?
pub fn printfloatprec<P: Processor>(p: &mut P) {
    todo!();
}

// Store immediate value in clear integer register
pub fn ldint<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer register addition
pub fn addint<P: Processor>(p: &mut P) {
    todo!();
}

// Clera integer register subtraction
pub fn subint<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer register multiplication
pub fn mulint<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer register division with floor rounding
pub fn divint<P: Processor>(p: &mut P) {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn jmp<P: Processor>(p: &mut P) {
    todo!();
}

// Conditional relative jump in the bytecode
// NZ = Not Zero
pub fn jmpnz<P: Processor>(p: &mut P) {
    todo!();
}

// Conditional relative jump in the bytecode
pub fn jmpeqz<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer zero test
pub fn eqzc<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer less than zero test
pub fn ltzc<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer less than comparison
pub fn ltc<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer greater than comparison
pub fn gtc<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer equality test
pub fn eqc<P: Processor>(p: &mut P) {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn jmpi<P: Processor>(p: &mut P) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immediate address
pub fn ldmint<P: Processor>(p: &mut P) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immeiate address
pub fn stmint<P: Processor>(p: &mut P) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by register address
pub fn ldminti<P: Processor>(p: &mut P) {
    todo!();
}

// Assign clear integer to clear integer memory value(s) by register address
pub fn stminti<P: Processor>(p: &mut P) {
    todo!();
}

// Pushes clear integer register to the thread-local stack
pub fn pushint<P: Processor>(p: &mut P) {
    todo!();
}

// Pops from the thread-local stack to clear integer
pub fn popint<P: Processor>(p: &mut P) {
    todo!();
}

// Copy clear integer register
pub fn movint<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer bit decomposition
pub fn bitdecint<P: Processor>(p: &mut P) {
    todo!();
}

// Store the argument passed to the current thread in clear integer register
pub fn ldarg<P: Processor>(p: &mut P) {
    todo!();
}

// Copy clear integer register to the thread argument
pub fn starg<P: Processor>(p: &mut P) {
    todo!();
}

// Output time since start of computation
pub fn time<P: Processor>(p: &mut P) {
    todo!();
}

// Start timer
pub fn start<P: Processor>(p: &mut P) {
    todo!();
}

// Stop timer
pub fn stop<P: Processor>(p: &mut P) {
    todo!();
}

pub fn gldms<P: Processor>(p: &mut P) {
    todo!();
}

// Assign clear memory value(s) to clear register by immediate address
pub fn ldmc<P: Processor>(p: &mut P) {
    todo!();
}

// Assign secret memory value(s) to secret register by immediate address
pub fn ldms<P: Processor>(p: &mut P) {
    todo!();
}

// Output clear integer register
// @dev: Should we keep this?
pub fn printint<P: Processor>(p: &mut P) {
    todo!();
}

pub fn gldmc<P: Processor>(p: &mut P) {
    todo!();
}

// Start tape/bytecode file in another thread
pub fn run_tape<P: Processor>(p: &mut P) {
    todo!();
}

// Join thread
pub fn join_tape<P: Processor>(p: &mut P) {
    todo!();
}

// Crash runtime if the register's value is > 0
pub fn crash<P: Processor>(p: &mut P) {
    todo!();
}

// Offline data usage
pub fn use_off<P: Processor>(p: &mut P) {
    todo!();
}

// Input usage
pub fn use_inp<P: Processor>(p: &mut P) {
    todo!();
}

// Store number of players in clear integer register
pub fn nplayers<P: Processor>(p: &mut P) {
    todo!();
}

// Store maximal number of corrupt players in clear integer register
pub fn threshold<P: Processor>(p: &mut P) {
    todo!();
}

// Store current player number in clear integer register
pub fn playerid<P: Processor>(p: &mut P) {
    todo!();
}
