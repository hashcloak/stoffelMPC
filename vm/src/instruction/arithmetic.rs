use crate::processor::arithmetic::{ArithmeticProcessor};
use types::numbers::{MPCType};

// Assign immediate value to clear register
pub fn LDI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign immeidate value to secret register
pub fn LDSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign clear register to clear memory value(s) by immediate address
pub fn STMC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign secret register to secret memory value(s) by immediate address
pub fn STMS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign clear memory value(s) to clear register by register address
pub fn LDMCI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign secret memory value(s) to secret register by register address
pub fn LDMSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign clear register to clear memory value(s) by register address
pub fn STMCI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Assign secret register to secret memory value(s) by register address
pub fn STMSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy clear register
pub fn MOVC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy secret register
pub fn MOVS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store number of current thread in clear integer register
pub fn LDTN<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Require on computation modulus
// @dev: Do we need to keep this?
pub fn REQBL<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Custom preprocessed data usage
pub fn USE_PREP<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Matrix multiplication usage
pub fn USE_MATMUL<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear addition
pub fn ADDC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret addition
pub fn ADDS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Mixed addition
pub fn ADDM<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Addition of clear register and immediate value
pub fn ADDCI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Addition of secret register and immediate value
pub fn ADDSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear subtraction
pub fn SUBC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret subtraction
pub fn SUBS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtract clear from secret value
pub fn SUBML<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtract secret from clear value
pub fn SUBMR<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtraction immediate value from clear register
pub fn SUBCI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtraction immediate value from secret register
pub fn SUBSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtraction of clear register from immediate value
pub fn SUBCFI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Subtraction of secret register from immediate value
pub fn SUBSFI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear multiplcation
pub fn MULC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Multiply secret and clear value
pub fn MULM<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Multipleication of clear register and immediate value
pub fn MULCI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Multiplication of secret register and immediate value
pub fn MULSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear division
pub fn DIVC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Division of secret register and immediate value
pub fn DIVCI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear modular reduction
pub fn MODC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Modular reduction of clear register and immediate value
pub fn MODCI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear legendre symbol computation over prime p
pub fn LEGENDREC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear truncated hash computation
pub fn DIGESTC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Inverse of power of two modulo prime
pub fn INV2M<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear integer floor division
pub fn FLOORDIVC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random triples in secret register
pub fn TRIPLE<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random bits in secret register
pub fn BIT<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random squares in secret register
pub fn SQUARE<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random inverses in secret register
pub fn INV<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store custom preprocessed data in secret register
pub fn PREP<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh length-restricted random shares in secret register
pub fn RANDOMS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store fresh random input maskes in secret register and clear regiser of the relevant player
pub fn INPUTMASKREG<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store shares of a fresh secret random element in secret register
pub fn RANDOMFULLS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Read a variable number of clear values in internal representation from socket for a specified 
// client id and store them in clear registers
pub fn READSOCKETC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Read a variable number of secret values in internal representation from socket for a specified 
// client id and store them in secret registers
pub fn READSOCKETS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Write a variable number of secret shares from registers into a socket for a specified client id
pub fn WRITESOCKETS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Read a variable number of 32-bit integers from socket for a specified client id and store them in clear integer registers
pub fn WRITESOCKETSHARE<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Open a server socket on a party-specific port number and listen for client connections
// @dev: Does this need to be an opcode?
pub fn LISTEN<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Wait for a connection at the given port and write socket handle to clear integer register
// @dev: Does this need to be an opcode?
pub fn ACCEPTCLIENTCONNECTION<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Close connection to client
// @dev: Does this need to be an opcode?
pub fn CLOSECLIENTCONNECTION<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical AND of clear registers
pub fn ANDC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical XOR of clear registers
pub fn XORC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical OR of clear registers
pub fn ORC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical AND of clear registers and immediate value
pub fn ANDCI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical XOR of clear register and immediate value
pub fn XORCI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Logical OR of clear register and immediate value
pub fn ORCI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Clear logical NOT of a constant number of bits of clear register
pub fn NOTC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise left shift of clear register
pub fn SHLC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise right shift of clear register
pub fn SHRC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise left shift of clear by immediate value
pub fn SHLCI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise right shift of clear register by immediate value
pub fn SHRCI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise right shift of secret register by immediate value
pub fn SHRSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Reveat secret registers to clear registers
pub fn OPEN<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Element-wise mupleication of secret registers
pub fn MULS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Constant-vector mutiplication of secret registers
pub fn MULRS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Dot product of secret register
pub fn DOTPRODS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Probabilistic truncation if supported by protocol
pub fn TRUNC_PR<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret matrix multiplication from registers
pub fn MATMULS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret matrix multiplication reading directly from memory
pub fn MATMULSM<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Secret 2D convolution
pub fn CONV2DS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Force MAC check in current thread and all idle thread if current thread is the main thread
pub fn CHECK<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Debugging output of clear register
// @dev: Should we keep this?
pub fn PRINTREG<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store insecure random value of specified length in clear integer register
pub fn RAND<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Output clear register
pub fn PRINTREGPLAIN<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store publc input in clear register
pub fn PUBINPUT<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Output floating-number from clear registers
pub fn PRINTFLOATPLAIN<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Write shares to Persistence/Transactions-P<playerno>.data
pub fn WRITEFILESHARE<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Read shares from Persistence/Transactions-P<playerno>.data
pub fn READFILESHARE<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Conditionally output four bytes
pub fn CONDPRINTSTR<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Convert clear integer register to clear register
pub fn CONVINT<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Convert clear integer register to clear register
pub fn CONVMODP<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Create incremental clear integer
pub fn INCINT<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Randomly shuffles clear integer vector with public randomness
pub fn SHUFFLE<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Conditionally output clear register
pub fn CONDPRINTPLAIN<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// edaBit usage
pub fn USE_EDABIT<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Binary integer output
pub fn INTOUTPUT<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Binary floating-point output
pub fn FLOATOUTPUT<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store private input in secret registers
pub fn INPUTMIXED<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store private input in secret registers
pub fn INPUTMIXEDREG<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Store private input in secret registers
pub fn RAWINPUT<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Private input from cint
pub fn INPUTPERSONAL<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Bitwise XOR of single secret and clear bit registers
pub fn XORM<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy clear bit register vector to clear regiser by bit
pub fn CONVBITVEC<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy secret bit register
pub fn MOV<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy clear bit memory cell with run-time address to clear bit register
pub fn LDMCBI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

// Copy clear bit register to clear bit memory cell with run-time address
pub fn STMCBI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GLDSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GSTMS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GLDMSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GSTMSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GMOVS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GADDS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GADDM<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GADDSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GSUBS<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GSUBSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GSUBSFI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GSUBML<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GSUBMR<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GMULM<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GMULSI<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GTRIPLE<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GBIT<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}

pub fn GCONVINT<T: MPCType>(ap: &mut ArithmeticProcessor<T>) {
    todo!();
}
