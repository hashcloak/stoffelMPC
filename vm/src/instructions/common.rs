// VM Instructions that are common to both GC and Arithmetic circuits

use crate::processors::processor::{Processor};

// Output a single byte
pub fn PRINTCHR<P: Processor>(p: &mut P) {
    todo!();
}

// Output four bytes
pub fn PRINTSTR<P: Processor>(p: &mut P) {
    todo!();
}

// Set number of digits after decimal point for PRINTFLOATPLAIN
// @dev: Do we need this?
pub fn PRINTFLOATPREC<P: Processor>(p: &mut P) {
    todo!();
}

// Store immediate value in clear integer register
pub fn LDINT<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer register addition
pub fn ADDINT<P: Processor>(p: &mut P) {
    todo!();
}

// Clera integer register subtraction
pub fn SUBINT<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer register multiplication
pub fn MULINT<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer register division with floor rounding
pub fn DIVINT<P: Processor>(p: &mut P) {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn JMP<P: Processor>(p: &mut P) {
    todo!();
}

// Conditional relative jump in the bytecode
// NZ = Not Zero
pub fn JMPNZ<P: Processor>(p: &mut P) {
    todo!();
}

// Conditional relative jump in the bytecode
pub fn JMPEQZ<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer zero test
pub fn EQZC<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer less than zero test
pub fn LTZC<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer less than comparison
pub fn LTC<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer greater than comparison
pub fn GTC<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer equality test
pub fn EQC<P: Processor>(p: &mut P) {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn JMPI<P: Processor>(p: &mut P) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immediate address
pub fn LDMINT<P: Processor>(p: &mut P) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immeiate address
pub fn STMINT<P: Processor>(p: &mut P) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by register address
pub fn LDMINTI<P: Processor>(p: &mut P) {
    todo!();
}

// Assign clear integer to clear integer memory value(s) by register address
pub fn STMINTI<P: Processor>(p: &mut P) {
    todo!();
}

// Pushes clear integer register to the thread-local stack
pub fn PUSHINT<P: Processor>(p: &mut P) {
    todo!();
}

// Pops from the thread-local stack to clear integer
pub fn POPINT<P: Processor>(p: &mut P) {
    todo!();
}

// Copy clear integer register
pub fn MOVINT<P: Processor>(p: &mut P) {
    todo!();
}

// Clear integer bit decomposition
pub fn BITDECINT<P: Processor>(p: &mut P) {
    todo!();
}

// Store the argument passed to the current thread in clear integer register
pub fn LDARG<P: Processor>(p: &mut P) {
    todo!();
}

// Copy clear integer register to the thread argument
pub fn STARG<P: Processor>(p: &mut P) {
    todo!();
}

// Output time since start of computation
pub fn TIME<P: Processor>(p: &mut P) {
    todo!();
}

// Start timer
pub fn START<P: Processor>(p: &mut P) {
    todo!();
}

// Stop timer
pub fn STOP<P: Processor>(p: &mut P) {
    todo!();
}

pub fn GLDMS<P: Processor>(p: &mut P) {
    todo!();
}

// Assign clear memory value(s) to clear register by immediate address
pub fn LDMC<P: Processor>(p: &mut P) {
    todo!();
}

// Assign secret memory value(s) to secret register by immediate address
pub fn LDMS<P: Processor>(p: &mut P) {
    todo!();
}

// Output clear integer register
// @dev: Should we keep this?
pub fn PRINTINT<P: Processor>(p: &mut P) {
    todo!();
}

pub fn GLDMC<P: Processor>(p: &mut P) {
    todo!();
}

// Start tape/bytecode file in another thread
pub fn RUN_TAPE<P: Processor>(p: &mut P) {
    todo!();
}

// Join thread
pub fn JOIN_TAPE<P: Processor>(p: &mut P) {
    todo!();
}

// Crash runtime if the register's value is > 0
pub fn CRASH<P: Processor>(p: &mut P) {
    todo!();
}

// Offline data usage
pub fn USE<P: Processor>(p: &mut P) {
    todo!();
}

// Input usage
pub fn USE_INP<P: Processor>(p: &mut P) {
    todo!();
}

// Store number of players in clear integer register
pub fn NPLAYERS<P: Processor>(p: &mut P) {
    todo!();
}

// Store maximal number of corrupt players in clear integer register
pub fn THRESHOLD<P: Processor>(p: &mut P) {
    todo!();
}

// Store current player number in clear integer register
pub fn PLAYERID<P: Processor>(p: &mut P) {
    todo!();
}