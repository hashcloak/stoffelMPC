use crate::processor::arithmetic::ArithmeticCore;
use mpc::protocols::MPCProtocol;

/// Assign immediate value to clear register
pub fn ldi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>, register_pos: usize, immediate_value: T::VmType) {
    // TODO: Add error handling and a return type for successful execution
    processor.public_register.write(register_pos, immediate_value);
}

/// Assign immeidate value to secret register
pub fn ldsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

/// Assign clear register to clear memory value(s) by immediate address
pub fn stmc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Assign secret register to secret memory value(s) by immediate address
pub fn stms<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Assign clear memory value(s) to clear register by register address
pub fn ldmci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Assign secret memory value(s) to secret register by register address
pub fn ldmsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Assign clear register to clear memory value(s) by register address
pub fn stmci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Assign secret register to secret memory value(s) by register address
pub fn stmsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Copy clear register
pub fn movc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Copy secret register
pub fn movs<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Store number of current thread in clear integer register
pub fn ldtn<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
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

// Clear addition
pub fn addc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Secret addition
pub fn adds<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Mixed addition
pub fn addm<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Addition of clear register and immediate value
pub fn addci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Addition of secret register and immediate value
pub fn addsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Clear subtraction
pub fn subc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Secret subtraction
pub fn subs<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Subtract clear from secret value
pub fn subml<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Subtract secret from clear value
pub fn submr<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Subtraction immediate value from clear register
pub fn subci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Subtraction immediate value from secret register
pub fn subsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Subtraction of clear register from immediate value
pub fn subcfi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Subtraction of secret register from immediate value
pub fn subsfi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Clear multiplcation
pub fn mulc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Multiply secret and clear value
pub fn mulm<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Multipleication of clear register and immediate value
pub fn mulci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Multiplication of secret register and immediate value
pub fn mulsi<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Clear division
pub fn divc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Division of secret register and immediate value
pub fn divci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Clear modular reduction
pub fn modc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Modular reduction of clear register and immediate value
pub fn modci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Clear legendre symbol computation over prime p
pub fn legendrec<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Clear truncated hash computation
pub fn digestc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Inverse of power of two modulo prime
pub fn inv2m<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Clear integer floor division
pub fn floordivc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
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
pub fn andc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Logical XOR of clear registers
pub fn xorc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Logical OR of clear registers
pub fn orc<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Logical AND of clear registers and immediate value
pub fn andci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Logical XOR of clear register and immediate value
pub fn xorci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
}

// Logical OR of clear register and immediate value
pub fn orci<T: MPCProtocol>(processor: &mut ArithmeticCore<T>) {
    todo!();
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
