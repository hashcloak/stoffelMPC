use crate::processors::NewProcessor;
use mpc::protocols::MPCProtocol;
use types::numbers::Number;

// Assign immediate value to clear register
pub fn ldi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign immeidate value to secret register
pub fn ldsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign clear register to clear memory value(s) by immediate address
pub fn stmc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign secret register to secret memory value(s) by immediate address
pub fn stms<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign clear memory value(s) to clear register by register address
pub fn ldmci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign secret memory value(s) to secret register by register address
pub fn ldmsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign clear register to clear memory value(s) by register address
pub fn stmci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Assign secret register to secret memory value(s) by register address
pub fn stmsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear register
pub fn movc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy secret register
pub fn movs<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store number of current thread in clear integer register
pub fn ldtn<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Require on computation modulus
// @dev: Do we need to keep this?
pub fn reqbl<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Custom preprocessed data usage
pub fn use_prep<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Matrix multiplication usage
pub fn use_matmul<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear addition
pub fn addc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Secret addition
pub fn adds<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Mixed addition
pub fn addm<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Addition of clear register and immediate value
pub fn addci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Addition of secret register and immediate value
pub fn addsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear subtraction
pub fn subc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Secret subtraction
pub fn subs<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Subtract clear from secret value
pub fn subml<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Subtract secret from clear value
pub fn submr<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Subtraction immediate value from clear register
pub fn subci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Subtraction immediate value from secret register
pub fn subsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Subtraction of clear register from immediate value
pub fn subcfi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Subtraction of secret register from immediate value
pub fn subsfi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear multiplcation
pub fn mulc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Multiply secret and clear value
pub fn mulm<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Multipleication of clear register and immediate value
pub fn mulci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Multiplication of secret register and immediate value
pub fn mulsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear division
pub fn divc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Division of secret register and immediate value
pub fn divci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear modular reduction
pub fn modc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Modular reduction of clear register and immediate value
pub fn modci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear legendre symbol computation over prime p
pub fn legendrec<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear truncated hash computation
pub fn digestc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Inverse of power of two modulo prime
pub fn inv2m<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear integer floor division
pub fn floordivc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store fresh random triples in secret register
pub fn triple<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store fresh random bits in secret register
pub fn bit<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store fresh random squares in secret register
pub fn square<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store fresh random inverses in secret register
pub fn inv<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store custom preprocessed data in secret register
pub fn prep<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store fresh length-restricted random shares in secret register
pub fn randoms<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store fresh random input maskes in secret register and clear regiser of the relevant player
pub fn inputmaskreg<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Store shares of a fresh secret random element in secret register
pub fn randomfulls<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Read a variable number of clear values in internal representation from socket for a specified
// client id and store them in clear registers
pub fn readsocketc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Read a variable number of secret values in internal representation from socket for a specified
// client id and store them in secret registers
pub fn readsockets<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Write a variable number of secret shares from registers into a socket for a specified client id
pub fn writesockets<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Read a variable number of 32-bit integers from socket for a specified client id and store them in clear integer registers
pub fn writesocketshare<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Open a server socket on a party-specific port number and listen for client connections
// @dev: Does this need to be an opcode?
pub fn listen<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Wait for a connection at the given port and write socket handle to clear integer register
// @dev: Does this need to be an opcode?
pub fn acceptclientconnection<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Close connection to client
// @dev: Does this need to be an opcode?
pub fn closeclientconnection<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Logical AND of clear registers
pub fn andc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Logical XOR of clear registers
pub fn xorc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Logical OR of clear registers
pub fn orc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Logical AND of clear registers and immediate value
pub fn andci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Logical XOR of clear register and immediate value
pub fn xorci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Logical OR of clear register and immediate value
pub fn orci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Clear logical NOT of a constant number of bits of clear register
pub fn notc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Bitwise left shift of clear register
pub fn shlc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Bitwise right shift of clear register
pub fn shrc<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Bitwise left shift of clear by immediate value
pub fn shlci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Bitwise right shift of clear register by immediate value
pub fn shrci<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Bitwise right shift of secret register by immediate value
pub fn shrsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Reveat secret registers to clear registers
pub fn open<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Element-wise mupleication of secret registers
pub fn muls<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Constant-vector mutiplication of secret registers
pub fn mulrs<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Dot product of secret register
pub fn dotprods<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Probabilistic truncation if supported by protocol
pub fn trunc_pr<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Secret matrix multiplication from registers
pub fn matmuls<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Secret matrix multiplication reading directly from memory
pub fn matmulsm<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Secret 2D convolution
pub fn conv2ds<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Force MAC check in current thread and all idle thread if current thread is the main thread
pub fn check<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Debugging output of clear register
// @dev: Should we keep this?
pub fn printreg<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store insecure random value of specified length in clear integer register
pub fn rand<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Output clear register
pub fn printregplain<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Store publc input in clear register
pub fn pubinput<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Output floating-number from clear registers
pub fn printfloatplain<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Write shares to Persistence/Transactions-P<playerno>.data
pub fn writefileshare<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Read shares from Persistence/Transactions-P<playerno>.data
pub fn readfileshare<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Conditionally output four bytes
pub fn condprintstr<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Convert clear integer register to clear register
pub fn convint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Convert clear integer register to clear register
pub fn convmodp<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Create incremental clear integer
pub fn incint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Randomly shuffles clear integer vector with public randomness
pub fn shuffle<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Conditionally output clear register
pub fn condprintplain<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// edaBit usage
pub fn use_edabit<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Binary integer output
pub fn intoutput<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Binary floating-point output
pub fn floatoutput<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store private input in secret registers
pub fn inputmixed<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Store private input in secret registers
pub fn inputmixedreg<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Store private input in secret registers
pub fn rawinput<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Private input from cint
pub fn inputpersonal<T: MPCProtocol<U>, U: Number, V: Number>(
    processor: &mut NewProcessor<T, U, V>,
) {
    todo!();
}

// Bitwise XOR of single secret and clear bit registers
pub fn xorm<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear bit register vector to clear regiser by bit
pub fn convbitvec<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy secret bit register
pub fn mov<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear bit memory cell with run-time address to clear bit register
pub fn ldmcbi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

// Copy clear bit register to clear bit memory cell with run-time address
pub fn stmcbi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gldsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gstms<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gldmsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gstmsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gmovs<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gadds<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gaddm<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gaddsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gsubs<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gsubsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gsubsfi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gsubml<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gsubmr<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gmulm<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gmulsi<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gtriple<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gbit<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}

pub fn gconvint<T: MPCProtocol<U>, U: Number, V: Number>(processor: &mut NewProcessor<T, U, V>) {
    todo!();
}
