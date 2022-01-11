use crate::processors::arithmetic::ArithmeticProcessor;
use types::numbers::MPCType;

// Assign immediate value to clear register
pub fn ldi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign immeidate value to secret register
pub fn ldsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign clear register to clear memory value(s) by immediate address
pub fn stmc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign secret register to secret memory value(s) by immediate address
pub fn stms<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign clear memory value(s) to clear register by register address
pub fn ldmci<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign secret memory value(s) to secret register by register address
pub fn ldmsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign clear register to clear memory value(s) by register address
pub fn stmci<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign secret register to secret memory value(s) by register address
pub fn stmsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy clear register
pub fn movc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy secret register
pub fn movs<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store number of current thread in clear integer register
pub fn ldtn<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Require on computation modulus
// @dev: Do we need to keep this?
pub fn reqbl<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Custom preprocessed data usage
pub fn use_prep<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Matrix multiplication usage
pub fn use_matmul<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear addition
pub fn addc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret addition
pub fn adds<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Mixed addition
pub fn addm<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Addition of clear register and immediate value
pub fn addci<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Addition of secret register and immediate value
pub fn addsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear subtraction
pub fn subc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret subtraction
pub fn subs<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtract clear from secret value
pub fn subml<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtract secret from clear value
pub fn submr<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtraction immediate value from clear register
pub fn subci<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtraction immediate value from secret register
pub fn subsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtraction of clear register from immediate value
pub fn subcfi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtraction of secret register from immediate value
pub fn subsfi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear multiplcation
pub fn mulc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Multiply secret and clear value
pub fn mulm<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Multipleication of clear register and immediate value
pub fn mulci<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Multiplication of secret register and immediate value
pub fn mulsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear division
pub fn divc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Division of secret register and immediate value
pub fn divci<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear modular reduction
pub fn modc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Modular reduction of clear register and immediate value
pub fn modci<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear legendre symbol computation over prime p
pub fn legendrec<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear truncated hash computation
pub fn digestc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Inverse of power of two modulo prime
pub fn inv2m<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear integer floor division
pub fn floordivc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random triples in secret register
pub fn triple<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random bits in secret register
pub fn bit<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random squares in secret register
pub fn square<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random inverses in secret register
pub fn inv<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store custom preprocessed data in secret register
pub fn prep<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh length-restricted random shares in secret register
pub fn randoms<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random input maskes in secret register and clear regiser of the relevant player
pub fn inputmaskreg<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store shares of a fresh secret random element in secret register
pub fn randomfulls<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Read a variable number of clear values in internal representation from socket for a specified
// client id and store them in clear registers
pub fn readsocketc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Read a variable number of secret values in internal representation from socket for a specified
// client id and store them in secret registers
pub fn readsockets<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Write a variable number of secret shares from registers into a socket for a specified client id
pub fn writesockets<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Read a variable number of 32-bit integers from socket for a specified client id and store them in clear integer registers
pub fn writesocketshare<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Open a server socket on a party-specific port number and listen for client connections
// @dev: Does this need to be an opcode?
pub fn listen<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Wait for a connection at the given port and write socket handle to clear integer register
// @dev: Does this need to be an opcode?
pub fn acceptclientconnection<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Close connection to client
// @dev: Does this need to be an opcode?
pub fn closeclientconnection<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical AND of clear registers
pub fn andc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical XOR of clear registers
pub fn xorc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical OR of clear registers
pub fn orc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical AND of clear registers and immediate value
pub fn andci<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical XOR of clear register and immediate value
pub fn xorci<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical OR of clear register and immediate value
pub fn orci<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear logical NOT of a constant number of bits of clear register
pub fn notc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise left shift of clear register
pub fn shlc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise right shift of clear register
pub fn shrc<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise left shift of clear by immediate value
pub fn shlci<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise right shift of clear register by immediate value
pub fn shrci<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise right shift of secret register by immediate value
pub fn shrsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Reveat secret registers to clear registers
pub fn open<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Element-wise mupleication of secret registers
pub fn muls<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Constant-vector mutiplication of secret registers
pub fn mulrs<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Dot product of secret register
pub fn dotprods<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Probabilistic truncation if supported by protocol
pub fn trunc_pr<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret matrix multiplication from registers
pub fn matmuls<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret matrix multiplication reading directly from memory
pub fn matmulsm<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret 2D convolution
pub fn conv2ds<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Force MAC check in current thread and all idle thread if current thread is the main thread
pub fn check<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Debugging output of clear register
// @dev: Should we keep this?
pub fn printreg<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store insecure random value of specified length in clear integer register
pub fn rand<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Output clear register
pub fn printregplain<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store publc input in clear register
pub fn pubinput<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Output floating-number from clear registers
pub fn printfloatplain<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Write shares to Persistence/Transactions-P<playerno>.data
pub fn writefileshare<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Read shares from Persistence/Transactions-P<playerno>.data
pub fn readfileshare<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Conditionally output four bytes
pub fn condprintstr<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Convert clear integer register to clear register
pub fn convint<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Convert clear integer register to clear register
pub fn convmodp<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Create incremental clear integer
pub fn incint<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Randomly shuffles clear integer vector with public randomness
pub fn shuffle<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Conditionally output clear register
pub fn condprintplain<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// edaBit usage
pub fn use_edabit<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Binary integer output
pub fn intoutput<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Binary floating-point output
pub fn floatoutput<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store private input in secret registers
pub fn inputmixed<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store private input in secret registers
pub fn inputmixedreg<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store private input in secret registers
pub fn rawinput<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Private input from cint
pub fn inputpersonal<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise XOR of single secret and clear bit registers
pub fn xorm<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy clear bit register vector to clear regiser by bit
pub fn convbitvec<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy secret bit register
pub fn mov<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy clear bit memory cell with run-time address to clear bit register
pub fn ldmcbi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy clear bit register to clear bit memory cell with run-time address
pub fn stmcbi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gldsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gstms<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gldmsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gstmsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gmovs<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gadds<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gaddm<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gaddsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gsubs<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gsubsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gsubsfi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gsubml<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gsubmr<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gmulm<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gmulsi<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gtriple<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gbit<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn gconvint<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}
