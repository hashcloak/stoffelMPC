//Instructions that are common to both GC and Arithmetic circuits
use crate::processor::Processor;

// Output a single byte
pub fn printchr(processor: &mut impl Processor) {
    todo!();
}

// Output four bytes
pub fn printstr(processor: &mut impl Processor) {
    todo!();
}

// Set number of digits after decimal point for PRINTFLOATPLAIN
// @dev: Do we need this?
pub fn printfloatprec(processor: &mut impl Processor) {
    todo!();
}

// Store immediate value in clear integer register
pub fn ldint(processor: &mut impl Processor) {
    todo!();
}

// Clear integer register addition
pub fn addint(processor: &mut impl Processor) {
    todo!();
}

// Clera integer register subtraction
pub fn subint(processor: &mut impl Processor) {
    todo!();
}

// Clear integer register multiplication
pub fn mulint(processor: &mut impl Processor) {
    todo!();
}

// Clear integer register division with floor rounding
pub fn divint(processor: &mut impl Processor) {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn jmp(processor: &mut impl Processor) {
    todo!();
}

// Conditional relative jump in the bytecode
// NZ = Not Zero
pub fn jmpnz(processor: &mut impl Processor) {
    todo!();
}

// Conditional relative jump in the bytecode
pub fn jmpeqz(processor: &mut impl Processor) {
    todo!();
}

// Clear integer zero test
pub fn eqzc(processor: &mut impl Processor) {
    todo!();
}

// Clear integer less than zero test
pub fn ltzc(processor: &mut impl Processor) {
    todo!();
}

// Clear integer less than comparison
pub fn ltc(processor: &mut impl Processor) {
    todo!();
}

// Clear integer greater than comparison
pub fn gtc(processor: &mut impl Processor) {
    todo!();
}

// Clear integer equality test
pub fn eqc(processor: &mut impl Processor) {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn jmpi(processor: &mut impl Processor) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immediate address
pub fn ldmint(processor: &mut impl Processor) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immeiate address
pub fn stmint(processor: &mut impl Processor) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by register address
pub fn ldminti(processor: &mut impl Processor) {
    todo!();
}

// Assign clear integer to clear integer memory value(s) by register address
pub fn stminti(processor: &mut impl Processor) {
    todo!();
}

// Pushes clear integer register to the thread-local stack
pub fn pushint(processor: &mut impl Processor) {
    todo!();
}

// Pops from the thread-local stack to clear integer
pub fn popint(processor: &mut impl Processor) {
    todo!();
}

// Copy clear integer register
pub fn movint(processor: &mut impl Processor) {
    todo!();
}

// Clear integer bit decomposition
pub fn bitdecint(processor: &mut impl Processor) {
    todo!();
}

// Store the argument passed to the current thread in clear integer register
pub fn ldarg(processor: &mut impl Processor) {
    todo!();
}

// Copy clear integer register to the thread argument
pub fn starg(processor: &mut impl Processor) {
    todo!();
}

// Output time since start of computation
pub fn time(processor: &mut impl Processor) {
    todo!();
}

// Start timer
pub fn start(processor: &mut impl Processor) {
    todo!();
}

// Stop timer
pub fn stop(processor: &mut impl Processor) {
    todo!();
}

pub fn gldms(processor: &mut impl Processor) {
    todo!();
}

// Assign clear memory value(s) to clear register by immediate address
pub fn ldmc(processor: &mut impl Processor) {
    todo!();
}

// Assign secret memory value(s) to secret register by immediate address
pub fn ldms(processor: &mut impl Processor) {
    todo!();
}

// Output clear integer register
// @dev: Should we keep this?
pub fn printint(processor: &mut impl Processor) {
    todo!();
}

pub fn gldmc(processor: &mut impl Processor) {
    todo!();
}

// Start tape/bytecode file in another thread
pub fn run_tape(processor: &mut impl Processor) {
    todo!();
}

// Join thread
pub fn join_tape(processor: &mut impl Processor) {
    todo!();
}

// Crash runtime if the register's value is > 0
pub fn crash(processor: &mut impl Processor) {
    todo!();
}

// Offline data usage
pub fn use_off(processor: &mut impl Processor) {
    todo!();
}

// Input usage
pub fn use_inp(processor: &mut impl Processor) {
    todo!();
}

// Store number of players in clear integer register
pub fn nplayers(processor: &mut impl Processor) {
    todo!();
}

// Store maximal number of corrupt players in clear integer register
pub fn threshold(processor: &mut impl Processor) {
    todo!();
}

// Store current player number in clear integer register
pub fn playerid(processor: &mut impl Processor) {
    todo!();
}
