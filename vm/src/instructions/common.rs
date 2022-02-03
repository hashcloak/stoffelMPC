// VM<T, U, M, N> Instructions that are common to both GC and Arithmetic circuits

use types::numbers::Number;

use crate::stoffel_vm::StoffelVM;

// Output a single byte
pub fn printchr<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Output four bytes
pub fn printstr<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Set number of digits after decimal point for PRINTFLOATPLAIN
// @dev: Do we need this?
pub fn printfloatprec<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Store immediate value in clear integer register
pub fn ldint<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Clear integer register addition
pub fn addint<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Clera integer register subtraction
pub fn subint<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Clear integer register multiplication
pub fn mulint<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Clear integer register division with floor rounding
pub fn divint<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn jmp<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Conditional relative jump in the bytecode
// NZ = Not Zero
pub fn jmpnz<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Conditional relative jump in the bytecode
pub fn jmpeqz<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Clear integer zero test
pub fn eqzc<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Clear integer less than zero test
pub fn ltzc<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Clear integer less than comparison
pub fn ltc<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Clear integer greater than comparison
pub fn gtc<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Clear integer equality test
pub fn eqc<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn jmpi<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immediate address
pub fn ldmint<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immeiate address
pub fn stmint<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by register address
pub fn ldminti<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Assign clear integer to clear integer memory value(s) by register address
pub fn stminti<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Pushes clear integer register to the thread-local stack
pub fn pushint<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Pops from the thread-local stack to clear integer
pub fn popint<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Copy clear integer register
pub fn movint<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Clear integer bit decomposition
pub fn bitdecint<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Store the argument passed to the current thread in clear integer register
pub fn ldarg<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Copy clear integer register to the thread argument
pub fn starg<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Output time since start of computation
pub fn time<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Start timer
pub fn start<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Stop timer
pub fn stop<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

pub fn gldms<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Assign clear memory value(s) to clear register by immediate address
pub fn ldmc<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Assign secret memory value(s) to secret register by immediate address
pub fn ldms<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Output clear integer register
// @dev: Should we keep this?
pub fn printint<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

pub fn gldmc<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Start tape/bytecode file in another thread
pub fn run_tape<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Join thread
pub fn join_tape<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Crash runtime if the register's value is > 0
pub fn crash<T: Number, U: Number, const M: usize, const N: usize>(vm: &mut StoffelVM<T, U, M, N>) {
    todo!();
}

// Offline data usage
pub fn use_off<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Input usage
pub fn use_inp<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Store number of players in clear integer register
pub fn nplayers<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Store maximal number of corrupt players in clear integer register
pub fn threshold<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}

// Store current player number in clear integer register
pub fn playerid<T: Number, U: Number, const M: usize, const N: usize>(
    vm: &mut StoffelVM<T, U, M, N>,
) {
    todo!();
}
