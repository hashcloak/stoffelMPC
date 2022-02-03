use crate::processors::arithmetic::ArithmeticProcessor;
use types::numbers::Number;

// Assign immediate value to clear register
pub fn ldi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Assign immeidate value to secret register
pub fn ldsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Assign clear register to clear memory value(s) by immediate address
pub fn stmc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Assign secret register to secret memory value(s) by immediate address
pub fn stms<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Assign clear memory value(s) to clear register by register address
pub fn ldmci<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Assign secret memory value(s) to secret register by register address
pub fn ldmsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Assign clear register to clear memory value(s) by register address
pub fn stmci<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Assign secret register to secret memory value(s) by register address
pub fn stmsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Copy clear register
pub fn movc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Copy secret register
pub fn movs<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store number of current thread in clear integer register
pub fn ldtn<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Require on computation modulus
// @dev: Do we need to keep this?
pub fn reqbl<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Custom preprocessed data usage
pub fn use_prep<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Matrix multiplication usage
pub fn use_matmul<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Clear addition
pub fn addc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Secret addition
pub fn adds<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Mixed addition
pub fn addm<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Addition of clear register and immediate value
pub fn addci<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Addition of secret register and immediate value
pub fn addsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Clear subtraction
pub fn subc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Secret subtraction
pub fn subs<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Subtract clear from secret value
pub fn subml<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Subtract secret from clear value
pub fn submr<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Subtraction immediate value from clear register
pub fn subci<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Subtraction immediate value from secret register
pub fn subsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Subtraction of clear register from immediate value
pub fn subcfi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Subtraction of secret register from immediate value
pub fn subsfi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Clear multiplcation
pub fn mulc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Multiply secret and clear value
pub fn mulm<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Multipleication of clear register and immediate value
pub fn mulci<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Multiplication of secret register and immediate value
pub fn mulsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Clear division
pub fn divc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Division of secret register and immediate value
pub fn divci<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Clear modular reduction
pub fn modc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Modular reduction of clear register and immediate value
pub fn modci<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Clear legendre symbol computation over prime p
pub fn legendrec<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Clear truncated hash computation
pub fn digestc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Inverse of power of two modulo prime
pub fn inv2m<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Clear integer floor division
pub fn floordivc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store fresh random triples in secret register
pub fn triple<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store fresh random bits in secret register
pub fn bit<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store fresh random squares in secret register
pub fn square<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store fresh random inverses in secret register
pub fn inv<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store custom preprocessed data in secret register
pub fn prep<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store fresh length-restricted random shares in secret register
pub fn randoms<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store fresh random input maskes in secret register and clear regiser of the relevant player
pub fn inputmaskreg<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store shares of a fresh secret random element in secret register
pub fn randomfulls<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Read a variable number of clear values in internal representation from socket for a specified
// client id and store them in clear registers
pub fn readsocketc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Read a variable number of secret values in internal representation from socket for a specified
// client id and store them in secret registers
pub fn readsockets<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Write a variable number of secret shares from registers into a socket for a specified client id
pub fn writesockets<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Read a variable number of 32-bit integers from socket for a specified client id and store them in clear integer registers
pub fn writesocketshare<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Open a server socket on a party-specific port number and listen for client connections
// @dev: Does this need to be an opcode?
pub fn listen<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Wait for a connection at the given port and write socket handle to clear integer register
// @dev: Does this need to be an opcode?
pub fn acceptclientconnection<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Close connection to client
// @dev: Does this need to be an opcode?
pub fn closeclientconnection<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Logical AND of clear registers
pub fn andc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Logical XOR of clear registers
pub fn xorc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Logical OR of clear registers
pub fn orc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Logical AND of clear registers and immediate value
pub fn andci<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Logical XOR of clear register and immediate value
pub fn xorci<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Logical OR of clear register and immediate value
pub fn orci<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Clear logical NOT of a constant number of bits of clear register
pub fn notc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Bitwise left shift of clear register
pub fn shlc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Bitwise right shift of clear register
pub fn shrc<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Bitwise left shift of clear by immediate value
pub fn shlci<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Bitwise right shift of clear register by immediate value
pub fn shrci<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Bitwise right shift of secret register by immediate value
pub fn shrsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Reveat secret registers to clear registers
pub fn open<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Element-wise mupleication of secret registers
pub fn muls<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Constant-vector mutiplication of secret registers
pub fn mulrs<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Dot product of secret register
pub fn dotprods<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Probabilistic truncation if supported by protocol
pub fn trunc_pr<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Secret matrix multiplication from registers
pub fn matmuls<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Secret matrix multiplication reading directly from memory
pub fn matmulsm<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Secret 2D convolution
pub fn conv2ds<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Force MAC check in current thread and all idle thread if current thread is the main thread
pub fn check<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Debugging output of clear register
// @dev: Should we keep this?
pub fn printreg<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store insecure random value of specified length in clear integer register
pub fn rand<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Output clear register
pub fn printregplain<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store publc input in clear register
pub fn pubinput<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Output floating-number from clear registers
pub fn printfloatplain<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Write shares to Persistence/Transactions-P<playerno>.data
pub fn writefileshare<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Read shares from Persistence/Transactions-P<playerno>.data
pub fn readfileshare<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Conditionally output four bytes
pub fn condprintstr<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Convert clear integer register to clear register
pub fn convint<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Convert clear integer register to clear register
pub fn convmodp<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Create incremental clear integer
pub fn incint<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Randomly shuffles clear integer vector with public randomness
pub fn shuffle<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Conditionally output clear register
pub fn condprintplain<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// edaBit usage
pub fn use_edabit<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Binary integer output
pub fn intoutput<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Binary floating-point output
pub fn floatoutput<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store private input in secret registers
pub fn inputmixed<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store private input in secret registers
pub fn inputmixedreg<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Store private input in secret registers
pub fn rawinput<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Private input from cint
pub fn inputpersonal<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Bitwise XOR of single secret and clear bit registers
pub fn xorm<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Copy clear bit register vector to clear regiser by bit
pub fn convbitvec<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Copy secret bit register
pub fn mov<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Copy clear bit memory cell with run-time address to clear bit register
pub fn ldmcbi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

// Copy clear bit register to clear bit memory cell with run-time address
pub fn stmcbi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gldsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gstms<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gldmsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gstmsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gmovs<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gadds<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gaddm<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gaddsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gsubs<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gsubsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gsubsfi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gsubml<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gsubmr<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gmulm<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gmulsi<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gtriple<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gbit<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}

pub fn gconvint<T: Number, U: Number, const M: usize, const N: usize>(
    ap: &mut ArithmeticProcessor<T, U, M, N>,
) {
    todo!();
}
