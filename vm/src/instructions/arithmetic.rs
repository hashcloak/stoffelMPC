use crate::processors::arithmetic::ArithmeticProcessor;
use types::numbers::{secret_sharing::SecretSharing, Number};

// Assign immediate value to clear register
pub fn ldi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign immeidate value to secret register
pub fn ldsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign clear register to clear memory value(s) by immediate address
pub fn stmc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign secret register to secret memory value(s) by immediate address
pub fn stms<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign clear memory value(s) to clear register by register address
pub fn ldmci<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign secret memory value(s) to secret register by register address
pub fn ldmsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign clear register to clear memory value(s) by register address
pub fn stmci<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign secret register to secret memory value(s) by register address
pub fn stmsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy clear register
pub fn movc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy secret register
pub fn movs<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store number of current thread in clear integer register
pub fn ldtn<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Require on computation modulus
// @dev: Do we need to keep this?
pub fn reqbl<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Custom preprocessed data usage
pub fn use_prep<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Matrix multiplication usage
pub fn use_matmul<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear addition
pub fn addc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret addition
pub fn adds<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Mixed addition
pub fn addm<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Addition of clear register and immediate value
pub fn addci<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Addition of secret register and immediate value
pub fn addsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear subtraction
pub fn subc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret subtraction
pub fn subs<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtract clear from secret value
pub fn subml<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtract secret from clear value
pub fn submr<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtraction immediate value from clear register
pub fn subci<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtraction immediate value from secret register
pub fn subsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtraction of clear register from immediate value
pub fn subcfi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtraction of secret register from immediate value
pub fn subsfi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear multiplcation
pub fn mulc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Multiply secret and clear value
pub fn mulm<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Multipleication of clear register and immediate value
pub fn mulci<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Multiplication of secret register and immediate value
pub fn mulsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear division
pub fn divc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Division of secret register and immediate value
pub fn divci<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear modular reduction
pub fn modc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Modular reduction of clear register and immediate value
pub fn modci<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear legendre symbol computation over prime p
pub fn legendrec<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear truncated hash computation
pub fn digestc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Inverse of power of two modulo prime
pub fn inv2m<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear integer floor division
pub fn floordivc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random triples in secret register
pub fn triple<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random bits in secret register
pub fn bit<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random squares in secret register
pub fn square<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random inverses in secret register
pub fn inv<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store custom preprocessed data in secret register
pub fn prep<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh length-restricted random shares in secret register
pub fn randoms<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random input maskes in secret register and clear regiser of the relevant player
pub fn inputmaskreg<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store shares of a fresh secret random element in secret register
pub fn randomfulls<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Read a variable number of clear values in internal representation from socket for a specified
// client id and store them in clear registers
pub fn readsocketc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Read a variable number of secret values in internal representation from socket for a specified
// client id and store them in secret registers
pub fn readsockets<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Write a variable number of secret shares from registers into a socket for a specified client id
pub fn writesockets<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Read a variable number of 32-bit integers from socket for a specified client id and store them in clear integer registers
pub fn writesocketshare<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Open a server socket on a party-specific port number and listen for client connections
// @dev: Does this need to be an opcode?
pub fn listen<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Wait for a connection at the given port and write socket handle to clear integer register
// @dev: Does this need to be an opcode?
pub fn acceptclientconnection<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Close connection to client
// @dev: Does this need to be an opcode?
pub fn closeclientconnection<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical AND of clear registers
pub fn andc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical XOR of clear registers
pub fn xorc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical OR of clear registers
pub fn orc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical AND of clear registers and immediate value
pub fn andci<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical XOR of clear register and immediate value
pub fn xorci<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical OR of clear register and immediate value
pub fn orci<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear logical NOT of a constant number of bits of clear register
pub fn notc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise left shift of clear register
pub fn shlc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise right shift of clear register
pub fn shrc<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise left shift of clear by immediate value
pub fn shlci<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise right shift of clear register by immediate value
pub fn shrci<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise right shift of secret register by immediate value
pub fn shrsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Reveat secret registers to clear registers
pub fn open<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Element-wise mupleication of secret registers
pub fn muls<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Constant-vector mutiplication of secret registers
pub fn mulrs<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Dot product of secret register
pub fn dotprods<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Probabilistic truncation if supported by protocol
pub fn trunc_pr<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret matrix multiplication from registers
pub fn matmuls<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret matrix multiplication reading directly from memory
pub fn matmulsm<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret 2D convolution
pub fn conv2ds<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Force MAC check in current thread and all idle thread if current thread is the main thread
pub fn check<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Debugging output of clear register
// @dev: Should we keep this?
pub fn printreg<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store insecure random value of specified length in clear integer register
pub fn rand<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Output clear register
pub fn printregplain<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store publc input in clear register
pub fn pubinput<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Output floating-number from clear registers
pub fn printfloatplain<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Write shares to Persistence/Transactions-P<playerno>.data
pub fn writefileshare<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Read shares from Persistence/Transactions-P<playerno>.data
pub fn readfileshare<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Conditionally output four bytes
pub fn condprintstr<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Convert clear integer register to clear register
pub fn convint<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Convert clear integer register to clear register
pub fn convmodp<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Create incremental clear integer
pub fn incint<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Randomly shuffles clear integer vector with public randomness
pub fn shuffle<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Conditionally output clear register
pub fn condprintplain<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// edaBit usage
pub fn use_edabit<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Binary integer output
pub fn intoutput<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Binary floating-point output
pub fn floatoutput<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store private input in secret registers
pub fn inputmixed<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store private input in secret registers
pub fn inputmixedreg<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store private input in secret registers
pub fn rawinput<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Private input from cint
pub fn inputpersonal<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise XOR of single secret and clear bit registers
pub fn xorm<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy clear bit register vector to clear regiser by bit
pub fn convbitvec<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy secret bit register
pub fn mov<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy clear bit memory cell with run-time address to clear bit register
pub fn ldmcbi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy clear bit register to clear bit memory cell with run-time address
pub fn stmcbi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gldsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gstms<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gldmsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gstmsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gmovs<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gadds<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gaddm<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gaddsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gsubs<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gsubsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gsubsfi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gsubml<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gsubmr<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gmulm<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gmulsi<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gtriple<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gbit<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gconvint<T: Number + SecretSharing>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}
