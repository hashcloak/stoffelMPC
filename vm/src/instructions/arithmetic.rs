use ark_ff::{BigInteger, Field, LegendreSymbol, PrimeField};

use std::sync::Arc;

use crate::{processor::arithmetic::ArithmeticCore, state::MemoryArray};
use mpc::protocols::MPCProtocol;
use mpc::share::Share;
use std::sync::Mutex;
use types::vm::{MemoryAddr, RegisterAddr};

use num_bigint::BigUint;

fn from_domain_to_bigint<T: MPCProtocol>(value: T::Domain) -> BigUint {
    let value_bytes = value.into_bigint().to_bytes_be();
    BigUint::from_bytes_be(&value_bytes)
}

fn from_bigint_to_domain<T: MPCProtocol>(value: BigUint) -> T::Domain {
    let value_bytes = value.to_bytes_be();
    T::Domain::from_be_bytes_mod_order(&value_bytes)
}

/// Assign immediate value to clear register
pub fn ldi<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    register_pos: RegisterAddr,
    immediate_value: T::Domain,
) {
    // TODO: Add error handling and a return type for successful execution
    processor
        .clear_register_mut()
        .write(register_pos, immediate_value);
}

/// Assign immediate value to secret register
pub fn ldsi<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    register_pos: RegisterAddr,
    immediate_value: Share<T::Domain>,
) {
    processor
        .secret_register_mut()
        .write(register_pos, immediate_value);
}

/// Assign clear register to clear memory value(s) by immediate address
pub fn stmc<T: MPCProtocol>(
    processor: &ArithmeticCore<T>,
    clear_memory: Arc<Mutex<MemoryArray<T::Domain>>>,
    reg_addr: RegisterAddr,
    mem_addr: MemoryAddr,
) {
    let value = processor.clear_register().read(reg_addr);
    clear_memory.lock().unwrap().write(mem_addr, value);
}

/// Assign secret register to secret memory value(s) by immediate address
pub fn stms<T: MPCProtocol>(
    processor: &ArithmeticCore<T>,
    secret_memory: Arc<Mutex<MemoryArray<Share<T::Domain>>>>,
    reg_addr: RegisterAddr,
    mem_addr: MemoryAddr,
) {
    let value = processor.secret_register().read(reg_addr);
    secret_memory.lock().unwrap().write(mem_addr, value);
}

/// Assign clear memory value(s) to clear register by register address
pub fn ldmci<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    clear_memory: Arc<Mutex<MemoryArray<T::Domain>>>,
    reg_addr: RegisterAddr,
    mem_addr: MemoryAddr,
) {
    let value = clear_memory.lock().unwrap().read(mem_addr);
    processor.clear_register_mut().write(reg_addr, value);
}

/// Assign secret memory value(s) to secret register by register address
pub fn ldmsi<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    secret_memory: Arc<Mutex<MemoryArray<Share<T::Domain>>>>,
    reg_addr: RegisterAddr,
    mem_addr: MemoryAddr,
) {
    let value = secret_memory.lock().unwrap().read(mem_addr);
    processor.secret_register_mut().write(reg_addr, value);
}

// Assign clear register to clear memory value(s) by register address
pub fn stmci<T: MPCProtocol>(
    processor: &ArithmeticCore<T>,
    clear_memory: Arc<Mutex<MemoryArray<T::Domain>>>,
    reg_addr_origin: RegisterAddr,
    reg_addr_destiny: RegisterAddr,
) {
    let moved_value = processor.clear_register().read(reg_addr_origin);
    let index = processor.reg_addr_register().read(reg_addr_destiny);
    clear_memory.lock().unwrap().write(index, moved_value);
}

// Assign secret register to secret memory value(s) by register address
pub fn stmsi<T: MPCProtocol>(
    processor: &ArithmeticCore<T>,
    secret_memory: Arc<Mutex<MemoryArray<Share<T::Domain>>>>,
    reg_addr_origin: RegisterAddr,
    reg_add_destiny: RegisterAddr,
) {
    let moved_share = processor.secret_register().read(reg_addr_origin);
    let index = processor.reg_addr_register().read(reg_add_destiny);
    secret_memory.lock().unwrap().write(index, moved_share);
}

// Copy clear register
pub fn movc<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr_origin: RegisterAddr,
    reg_addr_destiny: RegisterAddr,
) {
    let value = processor.clear_register().read(reg_addr_origin);
    processor
        .clear_register_mut()
        .write(reg_addr_destiny, value);
}

// Copy secret register
pub fn movs<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr_origin: RegisterAddr,
    reg_addr_destiny: RegisterAddr,
) {
    let value = processor.secret_register().read(reg_addr_origin);
    processor
        .secret_register_mut()
        .write(reg_addr_destiny, value);
}

// Store number of current thread in clear integer register
pub fn ldtn<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr: RegisterAddr,
    thread_n: usize,
) {
    processor.reg_addr_register_mut().write(reg_addr, thread_n);
}

// Require on computation modulus
// @dev: Do we need to keep this?
pub fn reqbl<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Custom preprocessed data usage
pub fn use_prep<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Matrix multiplication usage
pub fn use_matmul<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

/// Clear addition between two values in the clear register given two register
/// addresses.
pub fn addc<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr1: RegisterAddr,
    reg_addr2: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let value1 = processor.clear_register().read(reg_addr1);
    let value2 = processor.clear_register().read(reg_addr2);
    let result = value1 + value2;
    processor
        .clear_register_mut()
        .write(reg_addr_result, result);
}

/// Secret addition of two values in the secret register given two register
/// addresses.
pub fn adds<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr1: RegisterAddr,
    reg_addr2: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let share1 = processor.secret_register().read(reg_addr1);
    let share2 = processor.secret_register().read(reg_addr2);
    let share_result = share1 + share2;
    processor
        .secret_register_mut()
        .write(reg_addr_result, share_result);
}

// Mixed addition
pub fn addm<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    sec_reg_addr: RegisterAddr,
    clear_reg_addr: RegisterAddr,
    sec_reg_result: RegisterAddr,
) {
    let sec_value = processor.secret_register().read(sec_reg_addr);
    let cle_value = processor.clear_register().read(clear_reg_addr);
    let sum = sec_value + cle_value;
    processor.secret_register_mut().write(sec_reg_result, sum);
}

// Addition of clear register and immediate value
pub fn addci<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr: RegisterAddr,
    imm_value: T::Domain,
    reg_addr_result: RegisterAddr,
) {
    let stored_value = processor.clear_register().read(reg_addr);
    let sum = stored_value + imm_value;
    processor.clear_register_mut().write(reg_addr_result, sum);
}

// Addition of secret register and immediate value
pub fn addsi<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr: RegisterAddr,
    imm_value: T::Domain,
    reg_addr_result: RegisterAddr,
) {
    let stored_value = processor.secret_register().read(reg_addr);
    let sum = stored_value + imm_value;
    processor.secret_register_mut().write(reg_addr_result, sum);
}

/// Clear subtraction of two values in the clear register given two register
/// addresses.
pub fn subc<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr1: RegisterAddr,
    reg_addr2: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let value1 = processor.clear_register().read(reg_addr1);
    let value2 = processor.clear_register().read(reg_addr2);
    let result = value1 - value2;
    processor
        .clear_register_mut()
        .write(reg_addr_result, result);
}

/// Secret subtraction of two values in the secret register given two register
/// addresses.
pub fn subs<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr1: RegisterAddr,
    reg_addr2: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let share1 = processor.secret_register().read(reg_addr1);
    let share2 = processor.secret_register().read(reg_addr2);
    let share_result = share1 - share2;
    processor
        .secret_register_mut()
        .write(reg_addr_result, share_result);
}

/// Subtract clear from secret value
pub fn subml<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    sec_reg_addr: RegisterAddr,
    clear_reg_addr: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let share = processor.secret_register().read(sec_reg_addr);
    let clear = processor.clear_register().read(clear_reg_addr);
    let result = share - clear;
    processor
        .secret_register_mut()
        .write(reg_addr_result, result);
}

/// Subtract secret from clear value
pub fn submr<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    sec_reg_addr: RegisterAddr,
    clear_reg_addr: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let share = processor.secret_register().read(sec_reg_addr);
    let clear = processor.clear_register().read(clear_reg_addr);
    let result = (-share) + clear;
    processor
        .secret_register_mut()
        .write(reg_addr_result, result);
}

/// Subtraction immediate value from clear register
pub fn subci<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr_stored: RegisterAddr,
    immediate_val: T::Domain,
    reg_addr_result: RegisterAddr,
) {
    let stored_value = processor.clear_register().read(reg_addr_stored);
    let result = stored_value - immediate_val;
    processor
        .clear_register_mut()
        .write(reg_addr_result, result);
}

/// Subtraction immediate value from secret register
pub fn subsi<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr_stored: RegisterAddr,
    immediate_val: T::Domain,
    reg_addr_result: RegisterAddr,
) {
    let stored_value = processor.secret_register().read(reg_addr_stored);
    let result = stored_value - immediate_val;
    processor
        .secret_register_mut()
        .write(reg_addr_result, result);
}

/// Subtraction of clear register from immediate value
pub fn subcfi<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr_stored: RegisterAddr,
    immediate_val: T::Domain,
    reg_addr_result: RegisterAddr,
) {
    let stored_value = processor.clear_register().read(reg_addr_stored);
    let result = immediate_val - stored_value;
    processor
        .clear_register_mut()
        .write(reg_addr_result, result);
}

/// Subtraction of secret register from immediate value
pub fn subsfi<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr_stored: RegisterAddr,
    immediate_val: T::Domain,
    reg_addr_result: RegisterAddr,
) {
    let stored_value = processor.secret_register().read(reg_addr_stored);
    let result = (-stored_value) + immediate_val;
    processor
        .secret_register_mut()
        .write(reg_addr_result, result);
}

/// Clear multiplcation
pub fn mulc<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr1: RegisterAddr,
    reg_addr2: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let value1 = processor.clear_register().read(reg_addr1);
    let value2 = processor.clear_register().read(reg_addr2);
    let result = value1 * value2;
    processor
        .clear_register_mut()
        .write(reg_addr_result, result);
}

/// Multiply secret and clear value
pub fn mulm<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    secret_reg_addr: RegisterAddr,
    clear_reg_addr: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let clear_value = processor.clear_register().read(clear_reg_addr);
    let share_value = processor.secret_register().read(secret_reg_addr);
    let mult_share = share_value * clear_value;
    processor
        .secret_register_mut()
        .write(reg_addr_result, mult_share);
}

/// Multiplication of clear register and immediate value
pub fn mulci<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr_value: RegisterAddr,
    immediate_val: T::Domain,
    reg_addr_result: RegisterAddr,
) {
    let stored_value = processor.clear_register().read(reg_addr_value);
    let result = stored_value * immediate_val;
    processor
        .clear_register_mut()
        .write(reg_addr_result, result);
}

/// Multiplication of secret register and immediate value
pub fn mulsi<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr_value: RegisterAddr,
    immediate_val: T::Domain,
    reg_addr_result: RegisterAddr,
) {
    let secret_value = processor.secret_register().read(reg_addr_value);
    let result = secret_value * immediate_val;
    processor
        .secret_register_mut()
        .write(reg_addr_result, result);
}

/// Clear division
pub fn divc<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr1: RegisterAddr,
    reg_addr2: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let value1 = processor.clear_register().read(reg_addr1);
    let value2 = processor.clear_register().read(reg_addr2);
    let result = value1 / value2;
    processor
        .clear_register_mut()
        .write(reg_addr_result, result);
}

/// Division of secret register and immediate value
pub fn divci<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr_value: RegisterAddr,
    immediate_val: T::Domain,
    reg_addr_result: RegisterAddr,
) {
    let stored_value = processor.clear_register().read(reg_addr_value);
    let result = stored_value / immediate_val;
    processor
        .clear_register_mut()
        .write(reg_addr_result, result);
}

/// Clear modular reduction
pub fn modc<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr1: RegisterAddr,
    reg_addr2: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    // Read values from the clear register.
    let value1 = processor.clear_register().read(reg_addr1);
    let value2 = processor.clear_register().read(reg_addr2);

    // support for arithmetic operations.
    let value1_bigint = from_domain_to_bigint::<T>(value1);
    let value2_bigint = from_domain_to_bigint::<T>(value2);

    let modulus = value1_bigint % value2_bigint;

    // Converts the resulting modulus into the corresponding domain of the
    // protocol.
    let modulus_field = from_bigint_to_domain::<T>(modulus);

    processor
        .clear_register_mut()
        .write(reg_addr_result, modulus_field);
}

/// Modular reduction of clear register and immediate value
pub fn modci<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr_value: RegisterAddr,
    immediate_val: T::Domain,
    reg_addr_result: RegisterAddr,
) {
    // Converts the stored value into a BigUint value given that this library
    // has better support for arithmetic computation.
    let stored_value = processor.clear_register().read(reg_addr_value);
    let stored_value_bigint = from_domain_to_bigint::<T>(stored_value);

    // Converts the immediate value into a BigUint
    let immediate_value_bigint = from_domain_to_bigint::<T>(immediate_val);

    // Computes the modulus and transforms it in afield element.
    let modulus = stored_value_bigint % immediate_value_bigint;
    let modulus_field = from_bigint_to_domain::<T>(modulus);

    processor
        .clear_register_mut()
        .write(reg_addr_result, modulus_field);
}

/// Clear legendre symbol computation over prime p
pub fn legendrec<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr_value: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let value = processor.clear_register().read(reg_addr_value);
    let legendre_symbol = match value.legendre() {
        LegendreSymbol::Zero => T::Domain::ZERO,
        LegendreSymbol::QuadraticNonResidue => -T::Domain::ONE,
        LegendreSymbol::QuadraticResidue => T::Domain::ONE,
    };
    processor
        .clear_register_mut()
        .write(reg_addr_result, legendre_symbol);
}

// Clear truncated hash computation
pub fn digestc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Inverse of power of two modulo prime
pub fn inv2m<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

/// Clear integer floor division
pub fn floordivc<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr1: RegisterAddr,
    reg_addr2: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let value1 = processor.clear_register().read(reg_addr1);
    let value2 = processor.clear_register().read(reg_addr2);

    let value1_bigint = from_domain_to_bigint::<T>(value1);
    let value2_bigint = from_domain_to_bigint::<T>(value2);

    let div = value1_bigint / value2_bigint;

    let div_domain = from_bigint_to_domain::<T>(div);
    processor
        .clear_register_mut()
        .write(reg_addr_result, div_domain);
}

// Store fresh random triples in secret register
pub fn triple<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store fresh random bits in secret register
pub fn bit<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store fresh random squares in secret register
pub fn square<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store fresh random inverses in secret register
pub fn inv<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store custom preprocessed data in secret register
pub fn prep<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store fresh length-restricted random shares in secret register
pub fn randoms<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store fresh random input maskes in secret register and clear regiser of the relevant player
pub fn inputmaskreg<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store shares of a fresh secret random element in secret register
pub fn randomfulls<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Read a variable number of clear values in internal representation from socket for a specified
// client id and store them in clear registers
pub fn readsocketc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Read a variable number of secret values in internal representation from socket for a specified
// client id and store them in secret registers
pub fn readsockets<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Write a variable number of secret shares from registers into a socket for a specified client id
pub fn writesockets<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Read a variable number of 32-bit integers from socket for a specified client id and store them in clear integer registers
pub fn writesocketshare<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Open a server socket on a party-specific port number and listen for client connections
// @dev: Does this need to be an opcode?
pub fn listen<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Wait for a connection at the given port and write socket handle to clear integer register
// @dev: Does this need to be an opcode?
pub fn acceptclientconnection<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Close connection to client
// @dev: Does this need to be an opcode?
pub fn closeclientconnection<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Logical AND of clear registers
pub fn andc<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr1: RegisterAddr,
    reg_addr2: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let value1 = processor.clear_register().read(reg_addr1);
    let value2 = processor.clear_register().read(reg_addr2);

    let value1_bigint = from_domain_to_bigint::<T>(value1);
    let value2_bigint = from_domain_to_bigint::<T>(value2);

    let and = value1_bigint & value2_bigint;

    let and_domain = from_bigint_to_domain::<T>(and);
    processor
        .clear_register_mut()
        .write(reg_addr_result, and_domain);
}

// Logical XOR of clear registers
pub fn xorc<T: MPCProtocol>(
    processor: &mut ArithmeticCore<T>,
    reg_addr1: RegisterAddr,
    reg_addr2: RegisterAddr,
    reg_addr_result: RegisterAddr,
) {
    let value1 = processor.clear_register().read(reg_addr1);
    let value2 = processor.clear_register().read(reg_addr2);

    let value1_bigint = from_domain_to_bigint::<T>(value1);
    let value2_bigint = from_domain_to_bigint::<T>(value2);

    let xor = value1_bigint ^ value2_bigint;

    let xor_domain = from_bigint_to_domain::<T>(xor);
    processor
        .clear_register_mut()
        .write(reg_addr_result, xor_domain);
}

// Logical OR of clear registers
pub fn orc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>, reg_addr1: RegisterAddr, reg_addr2: RegisterAddr, reg_addr_result: RegisterAddr) {
    let value1 = processor.clear_register().read(reg_addr1);
    let value2 = processor.clear_register().read(reg_addr2);

    let value1_bigint = from_domain_to_bigint::<T>(value1);
    let value2_bigint = from_domain_to_bigint::<T>(value2);

    let or = value1_bigint | value2_bigint;

    let or_domain = from_bigint_to_domain::<T>(or);
    processor
        .clear_register_mut()
        .write(reg_addr_result, or_domain);
}

// Logical AND of clear registers and immediate value
pub fn andci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>, reg_addr: RegisterAddr, immediate_val: T::Domain, reg_addr_result: RegisterAddr) {
    let stored_value = processor.clear_register().read(reg_addr);
    let stored_value_bigint = from_domain_to_bigint::<T>(stored_value);

    let imm_value_bigint = from_domain_to_bigint::<T>(immediate_val);

    let and_bigint = stored_value_bigint & imm_value_bigint;

    let and_domain = from_bigint_to_domain::<T>(and_bigint);
    processor
        .clear_register_mut()
        .write(reg_addr_result, and_domain);
}

// Logical XOR of clear register and immediate value
pub fn xorci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>, reg_addr: RegisterAddr, immediate_val: T::Domain, reg_addr_result: RegisterAddr) {
    let stored_value = processor.clear_register().read(reg_addr);
    let stored_value_bigint = from_domain_to_bigint::<T>(stored_value);

    let imm_value_bigint = from_domain_to_bigint::<T>(immediate_val);

    let xor_bigint = stored_value_bigint ^ imm_value_bigint;

    let xor_domain = from_bigint_to_domain::<T>(xor_bigint);
    processor
        .clear_register_mut()
        .write(reg_addr_result, xor_domain);
}

// Logical OR of clear register and immediate value
pub fn orci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>, reg_addr: RegisterAddr, immediate_val: T::Domain, reg_addr_result: RegisterAddr) {
    let stored_value = processor.clear_register().read(reg_addr);
    let stored_value_bigint = from_domain_to_bigint::<T>(stored_value);

    let imm_value_bigint = from_domain_to_bigint::<T>(immediate_val);

    let or_bigint = stored_value_bigint | imm_value_bigint;

    let or_domain = from_bigint_to_domain::<T>(or_bigint);
    processor
        .clear_register_mut()
        .write(reg_addr_result, or_domain);
}

// Clear logical NOT of a constant number of bits of clear register
pub fn notc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Bitwise left shift of clear register
pub fn shlc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Bitwise right shift of clear register
pub fn shrc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Bitwise left shift of clear by immediate value
pub fn shlci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Bitwise right shift of clear register by immediate value
pub fn shrci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Bitwise right shift of secret register by immediate value
pub fn shrsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Reveat secret registers to clear registers
pub fn open<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Element-wise mupleication of secret registers
pub fn muls<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Constant-vector mutiplication of secret registers
pub fn mulrs<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Dot product of secret register
pub fn dotprods<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Probabilistic truncation if supported by protocol
pub fn trunc_pr<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Secret matrix multiplication from registers
pub fn matmuls<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Secret matrix multiplication reading directly from memory
pub fn matmulsm<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Secret 2D convolution
pub fn conv2ds<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Force MAC check in current thread and all idle thread if current thread is the main thread
pub fn check<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Debugging output of clear register
// @dev: Should we keep this?
pub fn printreg<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store insecure random value of specified length in clear integer register
pub fn rand<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Output clear register
pub fn printregplain<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store publc input in clear register
pub fn pubinput<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Output floating-number from clear registers
pub fn printfloatplain<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Write shares to Persistence/Transactions-P<playerno>.data
pub fn writefileshare<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Read shares from Persistence/Transactions-P<playerno>.data
pub fn readfileshare<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Conditionally output four bytes
pub fn condprintstr<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Convert clear integer register to clear register
pub fn convint<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Convert clear integer register to clear register
pub fn convmodp<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Create incremental clear integer
pub fn incint<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Randomly shuffles clear integer vector with public randomness
pub fn shuffle<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Conditionally output clear register
pub fn condprintplain<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// edaBit usage
pub fn use_edabit<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Binary integer output
pub fn intoutput<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Binary floating-point output
pub fn floatoutput<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store private input in secret registers
pub fn inputmixed<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store private input in secret registers
pub fn inputmixedreg<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store private input in secret registers
pub fn rawinput<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Private input from cint
pub fn inputpersonal<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Bitwise XOR of single secret and clear bit registers
pub fn xorm<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Copy clear bit register vector to clear regiser by bit
pub fn convbitvec<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Copy secret bit register
pub fn mov<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Copy clear bit memory cell with run-time address to clear bit register
pub fn ldmcbi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Copy clear bit register to clear bit memory cell with run-time address
pub fn stmcbi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gldsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gstms<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gldmsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gstmsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gmovs<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gadds<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gaddm<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gaddsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gsubs<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gsubsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gsubsfi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gsubml<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gsubmr<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gmulm<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gmulsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gtriple<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gbit<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

pub fn gconvint<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

#[cfg(test)]
mod test {
    use crate::{processor::arithmetic::ArithmeticCore, state::Memory};
    use ark_bls12_381::Fr;
    use ark_ff::{BigInteger, PrimeField};
    use mpc::protocols::honey_badger::HoneyBadgerMPC;
    use types::vm::RegisterAddr;

    use super::*;

    #[test]
    fn test_ldi() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        let reg_addr: RegisterAddr = 4;
        let value = Fr::new(12_u64.into());

        ldi(&mut processor, reg_addr, value);

        assert_eq!(processor.clear_register().read(reg_addr), value);
    }

    #[test]
    fn test_ldsi() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        let reg_addr: RegisterAddr = 3;
        let value = Fr::new(12_u64.into());
        let share = Share::new(value);

        ldsi(&mut processor, reg_addr, share);

        assert_eq!(processor.secret_register().read(reg_addr), share);
    }

    #[test]
    fn test_stms() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        let reg_addr: RegisterAddr = 3;

        let value = Fr::new(12_u64.into());
        let share = Share::new(value);
        processor.secret_register_mut().write(reg_addr, share);

        let memory: Memory<HoneyBadgerMPC> = Memory::new(10);
        let mem_addr: MemoryAddr = 5;
        let secret_memory = Arc::clone(memory.sec_memory());

        stms(&processor, secret_memory, reg_addr, mem_addr);

        assert_eq!(
            processor.secret_register().read(reg_addr),
            memory.sec_memory().lock().unwrap().read(mem_addr)
        );
    }

    #[test]
    fn test_stmc() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        let reg_addr: RegisterAddr = 3;

        let value = Fr::new(12_u64.into());
        processor.clear_register_mut().write(reg_addr, value);

        let memory: Memory<HoneyBadgerMPC> = Memory::new(10);
        let mem_addr: MemoryAddr = 5;
        let public_memory = Arc::clone(memory.pub_memory());

        stmc(&processor, public_memory, reg_addr, mem_addr);

        assert_eq!(
            processor.clear_register().read(reg_addr),
            memory.pub_memory().lock().unwrap().read(mem_addr)
        );
    }

    #[test]
    fn test_ldmci() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        let reg_addr: RegisterAddr = 3;

        let value = Fr::new(12_u64.into());
        processor.clear_register_mut().write(reg_addr, value);

        let memory: Memory<HoneyBadgerMPC> = Memory::new(10);
        let mem_addr: MemoryAddr = 5;
        let public_memory = Arc::clone(memory.pub_memory());

        ldmci(&mut processor, public_memory, reg_addr, mem_addr);

        assert_eq!(
            processor.clear_register().read(reg_addr),
            memory.pub_memory().lock().unwrap().read(mem_addr)
        );
    }

    #[test]
    fn test_ldmsi() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        let reg_addr: RegisterAddr = 3;

        let value = Fr::new(12_u64.into());
        let share = Share::new(value);
        processor.secret_register_mut().write(reg_addr, share);

        let memory: Memory<HoneyBadgerMPC> = Memory::new(10);
        let mem_addr: MemoryAddr = 5;
        let secret_memory = Arc::clone(memory.sec_memory());

        ldmsi(&mut processor, secret_memory, reg_addr, mem_addr);

        assert_eq!(
            processor.secret_register().read(reg_addr),
            memory.sec_memory().lock().unwrap().read(mem_addr)
        );
    }

    #[test]
    fn test_addc() {
        // Register addresses
        let reg_addr1 = 2;
        let reg_addr2 = 5;
        let reg_addr_result = 7;

        // Values
        let value1 = Fr::new(12_u64.into());
        let value2 = Fr::new(24_u64.into());
        let sum = value1 + value2;

        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        processor.clear_register_mut().write(reg_addr1, value1);
        processor.clear_register_mut().write(reg_addr2, value2);

        addc(&mut processor, reg_addr1, reg_addr2, reg_addr_result);

        assert_eq!(processor.clear_register().read(reg_addr_result), sum);
    }

    #[test]
    fn test_adds() {
        // Register addresses
        let reg_addr1 = 2;
        let reg_addr2 = 5;
        let reg_addr_result = 7;

        // Values
        let value1 = Fr::new(12_u64.into());
        let share1 = Share::new(value1);
        let value2 = Fr::new(24_u64.into());
        let share2 = Share::new(value2);
        let sum = value1 + value2;
        let share_sum = Share::new(sum);

        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        processor.secret_register_mut().write(reg_addr1, share1);
        processor.secret_register_mut().write(reg_addr2, share2);

        adds(&mut processor, reg_addr1, reg_addr2, reg_addr_result);

        assert_eq!(processor.secret_register().read(reg_addr_result), share_sum);
    }

    #[test]
    fn test_subc() {
        // Register addresses
        let reg_addr1 = 2;
        let reg_addr2 = 5;
        let reg_addr_result = 7;

        // Values
        let value1 = Fr::new(12_u64.into());
        let value2 = Fr::new(24_u64.into());
        let sub = value1 - value2;

        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        processor.clear_register_mut().write(reg_addr1, value1);
        processor.clear_register_mut().write(reg_addr2, value2);

        subc(&mut processor, reg_addr1, reg_addr2, reg_addr_result);

        assert_eq!(processor.clear_register().read(reg_addr_result), sub);
    }

    #[test]
    fn test_subs() {
        // Register addresses
        let reg_addr1 = 2;
        let reg_addr2 = 5;
        let reg_addr_result = 7;

        // Values
        let value1 = Fr::new(12_u64.into());
        let share1 = Share::new(value1);
        let value2 = Fr::new(24_u64.into());
        let share2 = Share::new(value2);
        let sub = value1 - value2;
        let share_sub = Share::new(sub);

        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        processor.secret_register_mut().write(reg_addr1, share1);
        processor.secret_register_mut().write(reg_addr2, share2);

        subs(&mut processor, reg_addr1, reg_addr2, reg_addr_result);

        assert_eq!(processor.secret_register().read(reg_addr_result), share_sub);
    }

    #[test]
    fn test_subml() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        let share = Share::new(Fr::new(12_u64.into()));
        let value = Fr::new(8_u64.into());

        let reg_addr_clear = 4;
        processor.clear_register_mut().write(reg_addr_clear, value);
        let reg_addr_sec = 5;
        processor.secret_register_mut().write(reg_addr_sec, share);

        let reg_addr_result = 1;

        subml(
            &mut processor,
            reg_addr_sec,
            reg_addr_clear,
            reg_addr_result,
        );

        assert_eq!(
            processor.secret_register().read(reg_addr_result),
            share - value
        );
    }

    #[test]
    fn test_submr() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        let share = Share::new(Fr::new(12_u64.into()));
        let value = Fr::new(8_u64.into());

        let reg_addr_clear = 4;
        processor.clear_register_mut().write(reg_addr_clear, value);
        let reg_addr_sec = 5;
        processor.secret_register_mut().write(reg_addr_sec, share);

        let reg_addr_result = 1;

        submr(
            &mut processor,
            reg_addr_sec,
            reg_addr_clear,
            reg_addr_result,
        );

        assert_eq!(
            processor.secret_register().read(reg_addr_result),
            -share + value
        );
    }

    #[test]
    fn test_subci() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr_clear = 4;
        let reg_addr_result = 1;

        let stored_value = Fr::new(8_u64.into());
        let immediate_value = Fr::new(2_u64.into());
        processor
            .clear_register_mut()
            .write(reg_addr_clear, stored_value);

        subci(
            &mut processor,
            reg_addr_clear,
            immediate_value,
            reg_addr_result,
        );

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            stored_value - immediate_value
        )
    }

    #[test]
    fn test_subsi() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr_secret = 4;
        let reg_addr_result = 1;

        let stored_value = Share::new(Fr::new(8_u64.into()));
        let immediate_value = Fr::new(2_u64.into());
        processor
            .secret_register_mut()
            .write(reg_addr_secret, stored_value);

        subsi(
            &mut processor,
            reg_addr_secret,
            immediate_value,
            reg_addr_result,
        );

        assert_eq!(
            processor.secret_register().read(reg_addr_result),
            stored_value - immediate_value
        )
    }

    #[test]
    fn test_subcfi() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr_clear = 4;
        let reg_addr_result = 1;

        let stored_value = Fr::new(8_u64.into());
        let immediate_value = Fr::new(2_u64.into());

        processor
            .clear_register_mut()
            .write(reg_addr_clear, stored_value);

        subcfi(
            &mut processor,
            reg_addr_clear,
            immediate_value,
            reg_addr_result,
        );

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            immediate_value - stored_value
        )
    }

    #[test]
    fn test_subsfi() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr_secret = 4;
        let reg_addr_result = 1;

        let stored_share = Share::new(Fr::new(8_u64.into()));
        let immediate_value = Fr::new(2_u64.into());

        processor
            .secret_register_mut()
            .write(reg_addr_secret, stored_share);

        subsfi(
            &mut processor,
            reg_addr_secret,
            immediate_value,
            reg_addr_result,
        );

        assert_eq!(
            processor.secret_register().read(reg_addr_result),
            (-stored_share) + immediate_value
        )
    }

    #[test]
    fn test_mulc() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr1 = 1;
        let reg_addr2 = 3;
        let reg_addr_result = 2;

        let value1 = Fr::from(8_u64);
        let value2 = Fr::from(10_u64);

        processor.clear_register_mut().write(reg_addr1, value1);
        processor.clear_register_mut().write(reg_addr2, value2);

        mulc(&mut processor, reg_addr1, reg_addr2, reg_addr_result);

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            value1 * value2
        );
    }

    #[test]
    fn test_mulm() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr_secret = 1;
        let reg_addr_clear = 3;
        let reg_addr_result = 2;

        let value_clear = Fr::from(8_u64);
        let value_secret = Share::new(Fr::from(10_u64));

        processor
            .clear_register_mut()
            .write(reg_addr_clear, value_clear);
        processor
            .secret_register_mut()
            .write(reg_addr_secret, value_secret);

        mulm(
            &mut processor,
            reg_addr_secret,
            reg_addr_clear,
            reg_addr_result,
        );

        assert_eq!(
            processor.secret_register().read(reg_addr_result),
            value_secret * value_clear
        );
    }

    #[test]
    fn test_mulci() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr_clear = 4;
        let reg_addr_result = 1;

        let stored_value = Fr::new(8_u64.into());
        let immediate_value = Fr::new(2_u64.into());

        processor
            .clear_register_mut()
            .write(reg_addr_clear, stored_value);

        mulci(
            &mut processor,
            reg_addr_clear,
            immediate_value,
            reg_addr_result,
        );

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            stored_value * immediate_value
        );
    }

    #[test]
    fn test_divc() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr1 = 1;
        let reg_addr2 = 3;
        let reg_addr_result = 2;

        let value1 = Fr::from(8_u64);
        let value2 = Fr::from(10_u64);

        processor.clear_register_mut().write(reg_addr1, value1);
        processor.clear_register_mut().write(reg_addr2, value2);

        divc(&mut processor, reg_addr1, reg_addr2, reg_addr_result);

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            value1 / value2
        );
    }

    #[test]
    fn test_divci() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr_clear = 4;
        let reg_addr_result = 1;

        let stored_value = Fr::new(8_u64.into());
        let immediate_value = Fr::new(2_u64.into());

        processor
            .clear_register_mut()
            .write(reg_addr_clear, stored_value);

        divci(
            &mut processor,
            reg_addr_clear,
            immediate_value,
            reg_addr_result,
        );

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            stored_value / immediate_value
        );
    }

    #[test]
    fn test_modc() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr1 = 1;
        let reg_addr2 = 3;
        let reg_addr_result = 2;

        let value1 = Fr::from(8_u64);
        let value2 = Fr::from(10_u64);

        processor.clear_register_mut().write(reg_addr1, value1);
        processor.clear_register_mut().write(reg_addr2, value2);

        modc(&mut processor, reg_addr1, reg_addr2, reg_addr_result);

        let value1_bigint = from_domain_to_bigint::<HoneyBadgerMPC>(value1);
        let value2_bigint = from_domain_to_bigint::<HoneyBadgerMPC>(value2);

        let modulus = value1_bigint % value2_bigint;

        let modulus_in_domain = from_bigint_to_domain::<HoneyBadgerMPC>(modulus);

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            modulus_in_domain
        );
    }

    #[test]
    fn test_modci() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr_clear = 4;
        let reg_addr_result = 1;

        let stored_value = Fr::new(8_u64.into());
        let immediate_value = Fr::new(2_u64.into());

        processor
            .clear_register_mut()
            .write(reg_addr_clear, stored_value);

        modci(
            &mut processor,
            reg_addr_clear,
            immediate_value,
            reg_addr_result,
        );

        let stored_value_bigint = from_domain_to_bigint::<HoneyBadgerMPC>(stored_value);
        let immediate_value_bigint = from_domain_to_bigint::<HoneyBadgerMPC>(immediate_value);

        let modulus_bigint = stored_value_bigint % immediate_value_bigint;

        let modulus_in_domain = from_bigint_to_domain::<HoneyBadgerMPC>(modulus_bigint);

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            modulus_in_domain
        )
    }

    #[test]
    fn test_stmci() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        let clear_memory = Arc::new(Mutex::new(MemoryArray::new(10)));

        let reg_addr_origin = 4;
        let reg_addr_destiny = 3;
        let index = 6;

        let value = Fr::from(5_u64);

        processor
            .reg_addr_register_mut()
            .write(reg_addr_destiny, index);
        processor.clear_register_mut().write(reg_addr_origin, value);

        stmci(
            &processor,
            Arc::clone(&clear_memory),
            reg_addr_origin,
            reg_addr_destiny,
        );

        assert_eq!(clear_memory.lock().unwrap().read(index), value);
    }

    #[test]
    fn test_stmsi() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        let secret_memory = Arc::new(Mutex::new(MemoryArray::new(10)));

        let reg_addr_origin = 4;
        let reg_addr_destiny = 3;
        let index = 6;

        let value = Share::new(Fr::from(5_u64));

        processor
            .reg_addr_register_mut()
            .write(reg_addr_destiny, index);
        processor
            .secret_register_mut()
            .write(reg_addr_origin, value);

        stmsi(
            &processor,
            Arc::clone(&secret_memory),
            reg_addr_origin,
            reg_addr_destiny,
        );

        assert_eq!(secret_memory.lock().unwrap().read(index), value);
    }

    #[test]
    fn test_movc() {
        let reg_addr_origin = 3;
        let reg_addr_destiny = 2;

        let value = Fr::from(13_u64);

        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        processor.clear_register_mut().write(reg_addr_origin, value);

        movc(&mut processor, reg_addr_origin, reg_addr_destiny);

        assert_eq!(processor.clear_register().read(reg_addr_destiny), value);
    }

    #[test]
    fn test_movs() {
        let reg_addr_origin = 3;
        let reg_addr_destiny = 2;

        let value = Share::new(Fr::from(13_u64));

        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        processor
            .secret_register_mut()
            .write(reg_addr_origin, value);

        movs(&mut processor, reg_addr_origin, reg_addr_destiny);

        assert_eq!(processor.secret_register().read(reg_addr_destiny), value);
    }

    #[test]
    fn test_ldtn() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        let reg_addr = 3;
        let thread_n = 4;

        ldtn(&mut processor, reg_addr, thread_n);

        assert_eq!(processor.reg_addr_register().read(reg_addr), thread_n);
    }

    #[test]
    fn test_addm() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr_secret = 1;
        let reg_addr_clear = 3;
        let reg_addr_result = 2;

        let value_clear = Fr::from(8_u64);
        let value_secret = Share::new(Fr::from(10_u64));

        processor
            .clear_register_mut()
            .write(reg_addr_clear, value_clear);
        processor
            .secret_register_mut()
            .write(reg_addr_secret, value_secret);

        addm(
            &mut processor,
            reg_addr_secret,
            reg_addr_clear,
            reg_addr_result,
        );

        assert_eq!(
            processor.secret_register().read(reg_addr_result),
            value_secret + value_clear
        );
    }

    #[test]
    fn test_addci() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr_clear = 4;
        let reg_addr_result = 1;

        let stored_value = Fr::new(8_u64.into());
        let immediate_value = Fr::new(2_u64.into());

        processor
            .clear_register_mut()
            .write(reg_addr_clear, stored_value);

        addci(
            &mut processor,
            reg_addr_clear,
            immediate_value,
            reg_addr_result,
        );

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            stored_value + immediate_value
        );
    }

    #[test]
    fn test_addsi() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr_secret = 4;
        let reg_addr_result = 1;

        let stored_value = Share::new(Fr::new(8_u64.into()));
        let immediate_value = Fr::new(2_u64.into());
        processor
            .secret_register_mut()
            .write(reg_addr_secret, stored_value);

        addsi(
            &mut processor,
            reg_addr_secret,
            immediate_value,
            reg_addr_result,
        );

        assert_eq!(
            processor.secret_register().read(reg_addr_result),
            stored_value + immediate_value
        )
    }

    #[test]
    fn test_mulsi() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr_secret = 4;
        let reg_addr_result = 1;

        let stored_value = Share::new(Fr::new(8_u64.into()));
        let immediate_value = Fr::new(2_u64.into());
        processor
            .secret_register_mut()
            .write(reg_addr_secret, stored_value);

        mulsi(
            &mut processor,
            reg_addr_secret,
            immediate_value,
            reg_addr_result,
        );

        assert_eq!(
            processor.secret_register().read(reg_addr_result),
            stored_value * immediate_value
        )
    }

    #[test]
    fn test_legendre_is_quad() {
        let value = Fr::from(43525_u64);
        let quad = value * value;

        let reg_addr = 4;
        let reg_result = 3;

        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        processor.clear_register_mut().write(reg_addr, quad);

        legendrec(&mut processor, reg_addr, reg_result);

        assert_eq!(processor.clear_register().read(reg_result), Fr::ONE);
    }

    #[test]
    fn test_legendre_is_zero() {
        let modulus = Fr::MODULUS;

        let modulus_domain = Fr::from_be_bytes_mod_order(&modulus.to_bytes_be());

        let reg_addr = 4;
        let reg_result = 3;

        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        processor
            .clear_register_mut()
            .write(reg_addr, modulus_domain);

        legendrec(&mut processor, reg_addr, reg_result);

        assert_eq!(processor.clear_register().read(reg_result), Fr::ZERO);
    }

    #[test]
    fn test_legendre_is_non_quad() {
        let value = Fr::from(5_u64);

        let reg_addr = 4;
        let reg_result = 3;

        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        processor.clear_register_mut().write(reg_addr, value);

        legendrec(&mut processor, reg_addr, reg_result);

        assert_eq!(processor.clear_register().read(reg_result), -Fr::ONE);
    }

    #[test]
    fn test_floordivc() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr1 = 4;
        let reg_addr2 = 5;
        let reg_addr_result = 1;

        let value1_int = 12_u64;
        let value2_int = 9_u64;

        let division_int = value1_int / value2_int;

        let value1 = Fr::from(value1_int);
        let value2 = Fr::from(value2_int);

        processor.clear_register_mut().write(reg_addr1, value1);
        processor.clear_register_mut().write(reg_addr2, value2);

        floordivc(&mut processor, reg_addr1, reg_addr2, reg_addr_result);

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            Fr::from(division_int)
        );
    }

    #[test]
    fn test_xor() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr1 = 4;
        let reg_addr2 = 5;
        let reg_addr_result = 1;

        let value1_int = 12_u64;
        let value2_int = 9_u64;

        let xor_int = value1_int ^ value2_int;

        let value1 = Fr::from(value1_int);
        let value2 = Fr::from(value2_int);

        processor.clear_register_mut().write(reg_addr1, value1);
        processor.clear_register_mut().write(reg_addr2, value2);

        xorc(&mut processor, reg_addr1, reg_addr2, reg_addr_result);

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            Fr::from(xor_int)
        );
    }

    #[test]
    fn test_and() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr1 = 4;
        let reg_addr2 = 5;
        let reg_addr_result = 1;

        let value1_int = 12_u64;
        let value2_int = 9_u64;

        let and_int = value1_int & value2_int;

        let value1 = Fr::from(value1_int);
        let value2 = Fr::from(value2_int);

        processor.clear_register_mut().write(reg_addr1, value1);
        processor.clear_register_mut().write(reg_addr2, value2);

        andc(&mut processor, reg_addr1, reg_addr2, reg_addr_result);

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            Fr::from(and_int)
        );
    }

    #[test]
    fn test_orc() {
        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();

        let reg_addr1 = 4;
        let reg_addr2 = 5;
        let reg_addr_result = 1;

        let value1_int = 12_u64;
        let value2_int = 9_u64;

        let or_int = value1_int | value2_int;

        let value1 = Fr::from(value1_int);
        let value2 = Fr::from(value2_int);

        processor.clear_register_mut().write(reg_addr1, value1);
        processor.clear_register_mut().write(reg_addr2, value2);

        orc(&mut processor, reg_addr1, reg_addr2, reg_addr_result);

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            Fr::from(or_int)
        );
    }

    #[test]
    fn test_andci() {
        let reg_addr_stored = 5;
        let reg_addr_result = 2;

        let stored_val_int = 14_u64;
        let imm_val_int = 2_u64;
        let and_int = stored_val_int & imm_val_int;

        let stored_val = Fr::from(stored_val_int);
        let imm_val = Fr::from(imm_val_int);

        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        processor.clear_register_mut().write(reg_addr_stored, stored_val);
        
        andci(&mut processor, reg_addr_stored, imm_val, reg_addr_result);

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            Fr::from(and_int)
        )
    }

    #[test]
    fn test_orci() {
        let reg_addr_stored = 5;
        let reg_addr_result = 2;

        let stored_val_int = 14_u64;
        let imm_val_int = 2_u64;
        let or_int = stored_val_int | imm_val_int;

        let stored_val = Fr::from(stored_val_int);
        let imm_val = Fr::from(imm_val_int);

        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        processor.clear_register_mut().write(reg_addr_stored, stored_val);
        
        orci(&mut processor, reg_addr_stored, imm_val, reg_addr_result);

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            Fr::from(or_int)
        )
    }

    #[test]
    fn test_xorci() {
        let reg_addr_stored = 5;
        let reg_addr_result = 2;

        let stored_val_int = 14_u64;
        let imm_val_int = 2_u64;
        let xor_int = stored_val_int ^ imm_val_int;

        let stored_val = Fr::from(stored_val_int);
        let imm_val = Fr::from(imm_val_int);

        let mut processor: ArithmeticCore<HoneyBadgerMPC> = ArithmeticCore::new();
        processor.clear_register_mut().write(reg_addr_stored, stored_val);
        
        xorci(&mut processor, reg_addr_stored, imm_val, reg_addr_result);

        assert_eq!(
            processor.clear_register().read(reg_addr_result),
            Fr::from(xor_int)
        )
    }
}
