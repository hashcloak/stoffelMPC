use super::processors::{ArithmeticProcessor};

// Assign immediate value to clear register
pub fn LDI<A: ArithmeticProcessor>() {
    todo!();
}

// Assign immeidate value to secret register
pub fn LDSI<A: ArithmeticProcessor>() {
    todo!();
}

// Assign clear memory value(s) to clear register by immediate address
pub fn LDMC<A: ArithmeticProcessor>() {
    todo!();
}

// Assign secret memory value(s) to secret register by immediate address
pub fn LDMS<A: ArithmeticProcessor>() {
    todo!();
}

// Assign clear register to clear memory value(s) by immediate address
pub fn STMC<A: ArithmeticProcessor>() {
    todo!();
}

// Assign secret register to secret memory value(s) by immediate address
pub fn STMS<A: ArithmeticProcessor>() {
    todo!();
}

// Assign clear memory value(s) to clear register by register address
pub fn LDMCI<A: ArithmeticProcessor>() {
    todo!();
}

// Assign secret memory value(s) to secret register by register address
pub fn LDMSI<A: ArithmeticProcessor>() {
    todo!();
}

// Assign clear register to clear memory value(s) by register address
pub fn STMCI<A: ArithmeticProcessor>() {
    todo!();
}

// Assign secret register to secret memory value(s) by register address
pub fn STMSI<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear register
pub fn MOVC<A: ArithmeticProcessor>() {
    todo!();
}

// Copy secret register
pub fn MOVS<A: ArithmeticProcessor>() {
    todo!();
}

// Store number of current thread in clear integer register
pub fn LDTN<A: ArithmeticProcessor>() {
    todo!();
}

// Store the argument passed to the current thread in clear integer register
pub fn LDARG<A: ArithmeticProcessor>() {
    todo!();
}

// Require on computation modulus
// @dev: Do we need to keep this?
pub fn REQBL<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear integer register to the thread argument
pub fn STARG<A: ArithmeticProcessor>() {
    todo!();
}

// Output time since start of computation
pub fn TIME<A: ArithmeticProcessor>() {
    todo!();
}

// Start timer
pub fn START<A: ArithmeticProcessor>() {
    todo!();
}

// Stop timer
pub fn STOP<A: ArithmeticProcessor>() {
    todo!();
}

// Offline data usage
pub fn USE<A: ArithmeticProcessor>() {
    todo!();
}

// Input usage
pub fn USE_INP<A: ArithmeticProcessor>() {
    todo!();
}

// Start tape/bytecode file in another thread
pub fn RUN_TAPE<A: ArithmeticProcessor>() {
    todo!();
}

// Join thread
pub fn JOIN_TAPE<A: ArithmeticProcessor>() {
    todo!();
}

// Crash runtime if the register's value is > 0
pub fn CRASH<A: ArithmeticProcessor>() {
    todo!();
}

// Custom preprocessed data usage
pub fn USE_PREP<A: ArithmeticProcessor>() {
    todo!();
}

// Matrix multiplication usage
pub fn USE_MATMUL<A: ArithmeticProcessor>() {
    todo!();
}

// Clear addition
pub fn ADDC<A: ArithmeticProcessor>() {
    todo!();
}

// Secret addition
pub fn ADDS<A: ArithmeticProcessor>() {
    todo!();
}

// Mixed addition
pub fn ADDM<A: ArithmeticProcessor>() {
    todo!();
}

// Addition of clear register and immediate value
pub fn ADDCI<A: ArithmeticProcessor>() {
    todo!();
}

// Addition of secret register and immediate value
pub fn ADDSI<A: ArithmeticProcessor>() {
    todo!();
}

// Clear subtraction
pub fn SUBC<A: ArithmeticProcessor>() {
    todo!();
}

// Secret subtraction
pub fn SUBS<A: ArithmeticProcessor>() {
    todo!();
}

// Subtract clear from secret value
pub fn SUBML<A: ArithmeticProcessor>() {
    todo!();
}

// Subtract secret from clear value
pub fn SUBMR<A: ArithmeticProcessor>() {
    todo!();
}

// Subtraction immediate value from clear register
pub fn SUBCI<A: ArithmeticProcessor>() {
    todo!();
}

// Subtraction immediate value from secret register
pub fn SUBSI<A: ArithmeticProcessor>() {
    todo!();
}

// Subtraction of clear register from immediate value
pub fn SUBCFI<A: ArithmeticProcessor>() {
    todo!();
}

// Subtraction of secret register from immediate value
pub fn SUBSFI<A: ArithmeticProcessor>() {
    todo!();
}

// Clear multiplcation
pub fn MULC<A: ArithmeticProcessor>() {
    todo!();
}

// Multiply secret and clear value
pub fn MULM<A: ArithmeticProcessor>() {
    todo!();
}

// Multipleication of clear register and immediate value
pub fn MULCI<A: ArithmeticProcessor>() {
    todo!();
}

// Multiplication of secret register and immediate value
pub fn MULSI<A: ArithmeticProcessor>() {
    todo!();
}

// Clear division
pub fn DIVC<A: ArithmeticProcessor>() {
    todo!();
}

// Division of secret register and immediate value
pub fn DIVCI<A: ArithmeticProcessor>() {
    todo!();
}

// Clear modular reduction
pub fn MODC<A: ArithmeticProcessor>() {
    todo!();
}

// Modular reduction of clear register and immediate value
pub fn MODCI<A: ArithmeticProcessor>() {
    todo!();
}

// Clear legendre symbol computation over prime p
pub fn LEGENDREC<A: ArithmeticProcessor>() {
    todo!();
}

// Clear truncated hash computation
pub fn DIGESTC<A: ArithmeticProcessor>() {
    todo!();
}

// Inverse of power of two modulo prime
pub fn INV2M<A: ArithmeticProcessor>() {
    todo!();
}

// Clear integer floor division
pub fn FLOORDIVC<A: ArithmeticProcessor>() {
    todo!();
}

// Store fresh random triples in secret register
pub fn TRIPLE<A: ArithmeticProcessor>() {
    todo!();
}

// Store fresh random bits in secret register
pub fn BIT<A: ArithmeticProcessor>() {
    todo!();
}

// Store fresh random squares in secret register
pub fn SQUARE<A: ArithmeticProcessor>() {
    todo!();
}

// Store fresh random inverses in secret register
pub fn INV<A: ArithmeticProcessor>() {
    todo!();
}

// Store custom preprocessed data in secret register
pub fn PREP<A: ArithmeticProcessor>() {
    todo!();
}

// Store fresh random daBits in secret register
// @dev: Do we need to keep this?
pub fn DABIT<A: ArithmeticProcessor>() {
    todo!();
}

// Store frehs random loose edaBits in secret register
// @dev: Do we need to keep this?
pub fn EDABIT<A: ArithmeticProcessor>() {
    todo!();
}

// Store fresh random strict edaBits in secret register
pub fn SEDABIT<A: ArithmeticProcessor>() {
    todo!();
}

// Store fresh length-restricted random shares in secret register
pub fn RANDOMS<A: ArithmeticProcessor>() {
    todo!();
}

// Store fresh random input maskes in secret register and clear regiser of the relevant player
pub fn INPUTMASKREG<A: ArithmeticProcessor>() {
    todo!();
}

// Store shares of a fresh secret random element in secret register
pub fn RANDOMFULLS<A: ArithmeticProcessor>() {
    todo!();
}

// Read a variable number of clear values in internal representation from socket for a specified 
// client id and store them in clear registers
pub fn READSOCKETC<A: ArithmeticProcessor>() {
    todo!();
}

// Read a variable number of secret values in internal representation from socket for a specified 
// client id and store them in secret registers
pub fn READSOCKETS<A: ArithmeticProcessor>() {
    todo!();
}

// Write a variable number of secret shares from registers into a socket for a specified client id
pub fn WRITESOCKETS<A: ArithmeticProcessor>() {
    todo!();
}

// Read a variable number of 32-bit integers from socket for a specified client id and store them in clear integer registers
pub fn WRITESOCKETSHARE<A: ArithmeticProcessor>() {
    todo!();
}

// Open a server socket on a party-specific port number and listen for client connections
// @dev: Does this need to be an opcode?
pub fn LISTEN<A: ArithmeticProcessor>() {
    todo!();
}

// Wait for a connection at the given port and write socket handle to clear integer register
// @dev: Does this need to be an opcode?
pub fn ACCEPTCLIENTCONNECTION<A: ArithmeticProcessor>() {
    todo!();
}

// Close connection to client
// @dev: Does this need to be an opcode?
pub fn CLOSECLIENTCONNECTION<A: ArithmeticProcessor>() {
    todo!();
}

// Logical AND of clear registers
pub fn ANDC<A: ArithmeticProcessor>() {
    todo!();
}

// Logical XOR of clear registers
pub fn XORC<A: ArithmeticProcessor>() {
    todo!();
}

// Logical OR of clear registers
pub fn ORC<A: ArithmeticProcessor>() {
    todo!();
}

// Logical AND of clear registers and immediate value
pub fn ANDCI<A: ArithmeticProcessor>() {
    todo!();
}

// Logical XOR of clear register and immediate value
pub fn XORCI<A: ArithmeticProcessor>() {
    todo!();
}

// Logical OR of clear register and immediate value
pub fn ORCI<A: ArithmeticProcessor>() {
    todo!();
}

// Clear logical NOT of a constant number of bits of clear register
pub fn NOTC<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise left shift of clear register
pub fn SHLC<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise right shift of clear register
pub fn SHRC<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise left shift of clear by immediate value
pub fn SHLCI<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise right shift of clear register by immediate value
pub fn SHRCI<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise right shift of secret register by immediate value
pub fn SHRSI<A: ArithmeticProcessor>() {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn JMP<A: ArithmeticProcessor>() {
    todo!();
}

// Conditional relative jump in the bytecode
// NZ = Not Zero
pub fn JMPNZ<A: ArithmeticProcessor>() {
    todo!();
}

// Conditional relative jump in the bytecode
pub fn JMPEQZ<A: ArithmeticProcessor>() {
    todo!();
}

// Clear integer zero test
pub fn EQZC<A: ArithmeticProcessor>() {
    todo!();
}

// Clear integer less than zero test
pub fn LTZC<A: ArithmeticProcessor>() {
    todo!();
}

// Clear integer less than comparison
pub fn LTC<A: ArithmeticProcessor>() {
    todo!();
}

// Clear integer greater than comparison
pub fn GTC<A: ArithmeticProcessor>() {
    todo!();
}

// Clear integer equality test
pub fn EQC<A: ArithmeticProcessor>() {
    todo!();
}

// Unconditional relative jump in the bytecode
pub fn JMPI<A: ArithmeticProcessor>() {
    todo!();
}

// Clear integer bit decomposition
pub fn BITDECINT<A: ArithmeticProcessor>() {
    todo!();
}

// Store immediate value in clear integer register
pub fn LDINT<A: ArithmeticProcessor>() {
    todo!();
}

// Clear integer register addition
pub fn ADDINT<A: ArithmeticProcessor>() {
    todo!();
}

// Clera integer register subtraction
pub fn SUBINT<A: ArithmeticProcessor>() {
    todo!();
}

// Clear integer register multiplication
pub fn MULINT<A: ArithmeticProcessor>() {
    todo!();
}

// Clear integer register division with floor rounding
pub fn DIVINT<A: ArithmeticProcessor>() {
    todo!();
}

// Output clear integer register
// @dev: Should we keep this?
pub fn PRINTINT<A: ArithmeticProcessor>() {
    todo!();
}

// Reveat secret registers to clear registers
pub fn OPEN<A: ArithmeticProcessor>() {
    todo!();
}

// Element-wise mupleication of secret registers
pub fn MULS<A: ArithmeticProcessor>() {
    todo!();
}

// Constant-vector mutiplication of secret registers
pub fn MULRS<A: ArithmeticProcessor>() {
    todo!();
}

// Dot product of secret register
pub fn DOTPRODS<A: ArithmeticProcessor>() {
    todo!();
}

// Probabilistic truncation if supported by protocol
pub fn TRUNC_PR<A: ArithmeticProcessor>() {
    todo!();
}

// Secret matrix multiplication from registers
pub fn MATMULS<A: ArithmeticProcessor>() {
    todo!();
}

// Secret matrix multiplication reading directly from memory
pub fn MATMULSM<A: ArithmeticProcessor>() {
    todo!();
}

// Secret 2D convolution
pub fn CONV2DS<A: ArithmeticProcessor>() {
    todo!();
}

// Force MAC check in current thread and all idle thread if current thread is the main thread
pub fn CHECK<A: ArithmeticProcessor>() {
    todo!();
}

// Debugging output of clear register
// @dev: Should we keep this?
pub fn PRINTREG<A: ArithmeticProcessor>() {
    todo!();
}

// Store insecure random value of specified length in clear integer register
pub fn RAND<A: ArithmeticProcessor>() {
    todo!();
}

// Output clear register
pub fn PRINTREGPLAIN<A: ArithmeticProcessor>() {
    todo!();
}

// Output a single byte
pub fn PRINTCHR<A: ArithmeticProcessor>() {
    todo!();
}

// Output four bytes
pub fn PRINTSTR<A: ArithmeticProcessor>() {
    todo!();
}

// Store publc input in clear register
pub fn PUBINPUT<A: ArithmeticProcessor>() {
    todo!();
}

// Output floating-number from clear registers
pub fn PRINTFLOATPLAIN<A: ArithmeticProcessor>() {
    todo!();
}

// Write shares to Persistence/Transactions-P<playerno>.data
pub fn WRITEFILESHARE<A: ArithmeticProcessor>() {
    todo!();
}

// Read shares from Persistence/Transactions-P<playerno>.data
pub fn READFILESHARE<A: ArithmeticProcessor>() {
    todo!();
}

// Conditionally output four bytes
pub fn CONDPRINTSTR<A: ArithmeticProcessor>() {
    todo!();
}

// Convert clear integer register to clear register
pub fn CONVINT<A: ArithmeticProcessor>() {
    todo!();
}

// Convert clear integer register to clear register
pub fn CONVMODP<A: ArithmeticProcessor>() {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immediate address
pub fn LDMINT<A: ArithmeticProcessor>() {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by immeiate address
pub fn STMINT<A: ArithmeticProcessor>() {
    todo!();
}

// Assign clear integer memory value(s) to clear integer register by register address
pub fn LDMINTI<A: ArithmeticProcessor>() {
    todo!();
}

// Assign clear integer to clear integer memory value(s) by register address
pub fn STMINTI<A: ArithmeticProcessor>() {
    todo!();
}

// Pushes clear integer register to the thread-local stack
pub fn PUSHINT<A: ArithmeticProcessor>() {
    todo!();
}

// Pops from the thread-local stack to clear integer
pub fn POPINT<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear integer register
pub fn MOVINT<A: ArithmeticProcessor>() {
    todo!();
}

// Create incremental clear integer
pub fn INCINT<A: ArithmeticProcessor>() {
    todo!();
}

// Randomly shuffles clear integer vector with public randomness
pub fn SHUFFLE<A: ArithmeticProcessor>() {
    todo!();
}

// Set number of digits after decimal point for PRINTFLOATPLAIN
// @dev: Do we need this?
pub fn PRINTFLOATPREC<A: ArithmeticProcessor>() {
    todo!();
}

// Conditionally output clear register
pub fn CONDPRINTPLAIN<A: ArithmeticProcessor>() {
    todo!();
}

// Store number of players in clear integer register
pub fn NPLAYERS<A: ArithmeticProcessor>() {
    todo!();
}

// Store maximal number of corrupt players in clear integer register
pub fn THRESHOLD<A: ArithmeticProcessor>() {
    todo!();
}

// Store current player number in clear integer register
pub fn PLAYERID<A: ArithmeticProcessor>() {
    todo!();
}

// edaBit usage
pub fn USE_EDABIT<A: ArithmeticProcessor>() {
    todo!();
}

// Binary integer output
pub fn INTOUTPUT<A: ArithmeticProcessor>() {
    todo!();
}

// Binary floating-point output
pub fn FLOATOUTPUT<A: ArithmeticProcessor>() {
    todo!();
}

// Store private input in secret registers
pub fn INPUTMIXED<A: ArithmeticProcessor>() {
    todo!();
}

// Store private input in secret registers
pub fn INPUTMIXEDREG<A: ArithmeticProcessor>() {
    todo!();
}

// Store private input in secret registers
pub fn RAWINPUT<A: ArithmeticProcessor>() {
    todo!();
}

// Private input from cint
pub fn INPUTPERSONAL<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise XOR of secret bit register
pub fn XORS<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise XOR of single secret and clear bit registers
pub fn XORM<A: ArithmeticProcessor>() {
    todo!();
}

// Constant-vector AND of secret bit registeers
pub fn ANDRS<A: ArithmeticProcessor>() {
    todo!();
}

// Secret bit register decomposition
pub fn BITDECS<A: ArithmeticProcessor>() {
    todo!();
}

// Secret bit register decomposition
pub fn BITCOMS<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear integer register to secret bit register
pub fn CONVSINT<A: ArithmeticProcessor>() {
    todo!();
}

// Store immediate in secret bit register
pub fn LDBITS<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise AND of secret bit register vector
pub fn ANDS<A: ArithmeticProcessor>() {
    todo!();
}

// Secret bit register vector transpose
pub fn TRANS<A: ArithmeticProcessor>() {
    todo!();
}

// Copy fresh secret random bit to secret bit register
pub fn BITB<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise AND of single secret and clear bit registers
pub fn ANDM<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise NOT of secret register vector
pub fn NOTS<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise XOR of single clear bit register and immediate
pub fn XORCBI<A: ArithmeticProcessor>() {
    todo!();
}

// Secret bit register decomposition
pub fn BITDECC<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise NOT of secret register vector
pub fn NOTCB<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear integer register to clear bit register
pub fn CONVCINT<A: ArithmeticProcessor>() {
    todo!();
}

// Reveal secret bit register vectors and copy result to clear bit register vectors
pub fn REVEAL<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear bit memory cell with compile-time address to clear bit register
pub fn LDMCB<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear bit register to cleare bit memory cell with compile-time address
pub fn STMCB<A: ArithmeticProcessor>() {
    todo!();
}

// Bitwise XOR of two single clear bit registers
pub fn XORCB<A: ArithmeticProcessor>() {
    todo!();
}

// Integer addition two single clear bit registers
pub fn ADDCB<A: ArithmeticProcessor>() {
    todo!();
}

// Integer addition single clear bit register and immediate
pub fn ADDCBI<A: ArithmeticProcessor>() {
    todo!();
}

// Integer multiplication single clear bit register and immediate
pub fn MULCBI<A: ArithmeticProcessor>() {
    todo!();
}

// Right shift of clear bit register by immediate
pub fn SHRCBI<A: ArithmeticProcessor>() {
    todo!();
}

// Left shift of clear bit register by immediate
pub fn SHLCBI<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear register vector by bit to clear bit register vectors
pub fn CONVCINTVEC<A: ArithmeticProcessor>() {
    todo!();
}

// Signed output of clear bit register
pub fn PRINTREGSIGNED<A: ArithmeticProcessor>() {
    todo!();
}

// Debug output of clear bit register
pub fn PRINTREGB<A: ArithmeticProcessor>() {
    todo!();
}

// Output clear bit register
pub fn PRINTREGPLAINB<A: ArithmeticProcessor>() {
    todo!();
}

// Output floating-number from clear bit registers
pub fn PRINTFLOATPLAINB<A: ArithmeticProcessor>() {
    todo!();
}

// Conditionally output four bytes
pub fn CONDPRINTSTRB<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear bit register to clear integer register
pub fn CONVCBIT<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear bit register vector to clear regiser by bit
pub fn CONVBITVEC<A: ArithmeticProcessor>() {
    todo!();
}

// Copy secret bit memory cell with compile-time address to secret bit register
pub fn LDMSB<A: ArithmeticProcessor>() {
    todo!();
}

// Copy secret bit register to secret bit memory cell with compile-time address
pub fn STMSB<A: ArithmeticProcessor>() {
    todo!();
}

// Copy secret bit memory cell with run-time address to secret bit register
pub fn LDMSBI<A: ArithmeticProcessor>() {
    todo!();
}

// Copy secret bit register to secret bit memory cell with run-time address
pub fn STMSBI<A: ArithmeticProcessor>() {
    todo!();
}

// Copy secret bit register
pub fn MOV<A: ArithmeticProcessor>() {
    todo!();
}

// Copy private input to secret bit register vectors
pub fn INPUTB<A: ArithmeticProcessor>() {
    todo!();
}

// Copy private input secret bit registers bit by bit
pub fn INPUTBVEC<A: ArithmeticProcessor>() {
    todo!();
}

// Local share conversion
pub fn SPLIT<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear bit register vector to secret bit register vector
pub fn CONVCBIT2S<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear bit memory cell with run-time address to clear bit register
pub fn LDMCBI<A: ArithmeticProcessor>() {
    todo!();
}

// Copy clear bit register to clear bit memory cell with run-time address
pub fn STMCBI<A: ArithmeticProcessor>() {
    todo!();
}