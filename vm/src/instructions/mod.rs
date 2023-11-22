pub mod arithmetic;
pub mod boolean;
pub mod common;
pub mod opcodes;

use crate::state::Memory;

use super::processor::Processor;
use mpc::protocols::MPCProtocol;
use opcodes::InstructionOpcode;
use types::vm::{ImmediateValue, RegisterAddr};

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    /// Load/store
    /// (bytecode, immediate value, register/memory position)
    LDI(InstructionOpcode, ImmediateValue, RegisterAddr),
    LDSI(InstructionOpcode, ImmediateValue, RegisterAddr),
    LDMC(InstructionOpcode, ImmediateValue, RegisterAddr),
    LDMS(InstructionOpcode, ImmediateValue, RegisterAddr),
    STMC(InstructionOpcode, ImmediateValue, RegisterAddr),
    STMS(InstructionOpcode, ImmediateValue, RegisterAddr),
    /// (bytecode, memory, register)
    LDMCI(InstructionOpcode, RegisterAddr, RegisterAddr),
    LDMSI(InstructionOpcode, RegisterAddr, RegisterAddr),
    STMCI(InstructionOpcode, RegisterAddr, RegisterAddr),
    STMSI(InstructionOpcode, RegisterAddr, RegisterAddr),
    MOVC(InstructionOpcode, RegisterAddr, RegisterAddr),
    MOVS(InstructionOpcode, RegisterAddr, RegisterAddr),
    LDMINT(InstructionOpcode, ImmediateValue, RegisterAddr),
    STMINT(InstructionOpcode, ImmediateValue, RegisterAddr),
    LDMINTI(InstructionOpcode, RegisterAddr, RegisterAddr),
    STMINTI(InstructionOpcode, RegisterAddr, RegisterAddr),
    PUSHINT(InstructionOpcode, RegisterAddr),
    POPINT(InstructionOpcode, RegisterAddr),
    MOVINT(InstructionOpcode, RegisterAddr, RegisterAddr),
    /// Machine
    LDTN(InstructionOpcode, RegisterAddr),
    LDARG(InstructionOpcode, RegisterAddr),
    REQBL(InstructionOpcode, ImmediateValue),
    STARG(InstructionOpcode, RegisterAddr),
    TIME(InstructionOpcode),
    STARTTIMER(InstructionOpcode, ImmediateValue),
    STOPTIMER(InstructionOpcode, ImmediateValue),
    USE(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    USE_INP(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    // TODO: Figure out sensible way of handling variable length arguments
    RUN_TAPE(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        RegisterAddr,
        RegisterAddr,
    ),
    JOIN_TAPE(InstructionOpcode, RegisterAddr),
    CRASH(InstructionOpcode, RegisterAddr),
    NPLAYERS(InstructionOpcode, RegisterAddr),
    THRESHOLD(InstructionOpcode, RegisterAddr),
    PLAYERID(InstructionOpcode, RegisterAddr),
    USE_EDABIT(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    /// Addition
    ADDC(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    ADDS(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    ADDM(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    ADDCI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    ADDSI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    SUBC(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    SUBS(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    SUBML(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    SUBMR(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    SUBCI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    SUBSI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    SUBCFI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    SUBSFI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    /// Multiplication/division
    MULC(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    MULM(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    MULCI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    MULSI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    DIVC(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    DIVCI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    MODC(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    MODCI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    LEGENDREC(InstructionOpcode, RegisterAddr, RegisterAddr),
    DIGESTC(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    INV2M(InstructionOpcode, RegisterAddr, ImmediateValue),
    FLOORDIVC(InstructionOpcode, RegisterAddr, RegisterAddr),
    /// Open
    // TODO: Figure out sensible way of handling variable length arguments
    MULS(
        InstructionOpcode,
        ImmediateValue,
        RegisterAddr,
        RegisterAddr,
        RegisterAddr,
    ),
    // TODO: Figure out sensible way of handling variable length arguments
    MULRS(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
        RegisterAddr,
        RegisterAddr,
    ),
    // TODO: Minimize parameters
    // TODO: Figure out sensible way of handling variable length arguments
    DOTPRODS(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
        RegisterAddr,
        RegisterAddr,
        RegisterAddr,
    ),
    TRUNC_PR(
        InstructionOpcode,
        ImmediateValue,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
        ImmediateValue,
    ),
    MATMULS(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
    ),
    MATMULSM(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
        RegisterAddr,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
        ImmediateValue,
    ),
    CHECK(InstructionOpcode),
    /// Data access
    TRIPLE(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    BIT(InstructionOpcode, RegisterAddr),
    SQUARE(InstructionOpcode, RegisterAddr, RegisterAddr),
    INV(InstructionOpcode, RegisterAddr, RegisterAddr),
    INPUTMASK(InstructionOpcode, RegisterAddr, ImmediateValue),
    INPUTMASKREG(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    DABIT(InstructionOpcode, RegisterAddr, RegisterAddr),
    // TODO: Figure out sensible way of handling variable length arguments
    EDABIT(
        InstructionOpcode,
        ImmediateValue,
        RegisterAddr,
        RegisterAddr,
    ),
    // TODO: Figure out sensible way of handling variable length arguments
    SEDABIT(
        InstructionOpcode,
        ImmediateValue,
        RegisterAddr,
        RegisterAddr,
    ),
    RANDOMS(InstructionOpcode, RegisterAddr, ImmediateValue),
    RANDOMFULLS(InstructionOpcode, RegisterAddr),
    /// Input
    /// May not be needed INPUT = 0x60,
    /// INPUTFIX = 0xF0,
    /// INPUTFLOAT = 0xF1,
    INPUTMIXED(
        InstructionOpcode,
        ImmediateValue,
        RegisterAddr,
        Option<RegisterAddr>,
        Option<RegisterAddr>,
        Option<RegisterAddr>,
        Option<ImmediateValue>,
        ImmediateValue,
    ),
    /// INPUTMIXEDREG = 0xF3,
    RAWINPUT(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
    ),
    /// May not be needed INPUTPERSONAL = 0xF5,
    // TODO: Figure out sensible way of handling variable length arguments
    READSOCKETC(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
    ),
    // TODO: Figure out sensible way of handling variable length arguments
    READSOCKETS(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
    ),
    // TODO: Figure out sensible way of handling variable length arguments
    WRITESOCKETC(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
    ),
    // TODO: Figure out sensible way of handling variable length arguments
    WRITESOCKETS(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
    ),
    // TODO: Figure out sensible way of handling variable length arguments
    READSOCKETINT(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
    ),
    // TODO: Figure out sensible way of handling variable length arguments
    WRITESOCKETINT(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
    ),
    // TODO: Figure out sensible way of handling variable length arguments
    WRITESOCKETSHARE(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
    ),
    LISTEN(InstructionOpcode, RegisterAddr),
    ACCEPTCLIENTCONNECTION(InstructionOpcode, RegisterAddr, RegisterAddr),
    CLOSECLIENTCONNECTION(InstructionOpcode, RegisterAddr),
    /// Bitwise logic
    ANDC(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    XORC(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    ORC(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    ANDCI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    XORCI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    ORCI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    NOTC(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    /// Bitwise shifts
    SHLC(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    SHRC(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    SHLCI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    SHRCI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    SHRSI(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    /// Branching and comparison
    JMP(InstructionOpcode, ImmediateValue),
    JMPNZ(InstructionOpcode, ImmediateValue, ImmediateValue),
    JMPEQZ(InstructionOpcode, ImmediateValue, ImmediateValue),
    EQZC(InstructionOpcode, ImmediateValue, ImmediateValue),
    LTZC(InstructionOpcode, ImmediateValue, ImmediateValue),
    LTC(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
    ),
    GTC(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
    ),
    EQC(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
    ),
    JMPI(InstructionOpcode, ImmediateValue),
    /// Integers
    // TODO: Figure out sensible way of handling variable length arguments
    BITDECINT(
        InstructionOpcode,
        ImmediateValue,
        RegisterAddr,
        RegisterAddr,
        RegisterAddr,
    ),
    LDINT(InstructionOpcode, RegisterAddr, ImmediateValue),
    ADDINT(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    SUBINT(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    MULINT(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    DIVINT(InstructionOpcode, RegisterAddr, RegisterAddr, RegisterAddr),
    /// Do we need print instructions?
    /// PRINTINT(InstructionOpcode, RegisterAddr),
    INCINT(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
        ImmediateValue,
    ),
    SHUFFLE(InstructionOpcode, RegisterAddr, RegisterAddr),
    /// Conversion
    CONVINT(InstructionOpcode, RegisterAddr, RegisterAddr),
    CONVMODP(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    GCONVGF2N(InstructionOpcode, RegisterAddr, RegisterAddr),
    /// IO
    /// Do we need print instructions?
    /// PRINTREG = 0xB1,
    RAND(InstructionOpcode, RegisterAddr, RegisterAddr),
    /// PRINTREGPLAIN = 0xB3,
    /// PRINTCHR = 0xB4,
    /// PRINTSTR = 0xB5,
    PUBINPUT(InstructionOpcode, RegisterAddr),
    RAWOUTPUT(InstructionOpcode, RegisterAddr),
    STARTPRIVATEOUTPUT(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    STOPPRIVATEOUTPUT(
        InstructionOpcode,
        RegisterAddr,
        RegisterAddr,
        ImmediateValue,
    ),
    /// PRINTFLOATPLAIN = 0xBC,
    // TODO: Figure out sensible way of handling variable length arguments
    WRITEFILESHARE(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
    ),
    // TODO: Figure out sensible way of handling variable length arguments
    READFILESHARE(
        InstructionOpcode,
        ImmediateValue,
        ImmediateValue,
        RegisterAddr,
        RegisterAddr,
    ),
    /// CONDPRINTSTR = 0xBF,
    /// PRINTFLOATPREC = 0xE0,
    /// CONDPRINTPLAIN = 0xE1,
    INTOUTPUT(InstructionOpcode, ImmediateValue, RegisterAddr),
    FLOATOUTPUT(
        InstructionOpcode,
        ImmediateValue,
        RegisterAddr,
        RegisterAddr,
        RegisterAddr,
        RegisterAddr,
    ),
    // TODO: Figure out sensible way of handling variable length arguments
    GBITDEC(
        InstructionOpcode,
        RegisterAddr,
        ImmediateValue,
        RegisterAddr,
    ),
    // TODO: Figure out sensible way of handling variable length arguments
    GBITCOM(
        InstructionOpcode,
        RegisterAddr,
        ImmediateValue,
        RegisterAddr,
    ),
}

impl Instruction {
    /// Creates a new instruction
    pub fn new() -> Self {
        todo!();
    }

    /// Parses the instruction from a bytecode.
    pub fn parse(&mut self, bytes: impl AsRef<[u8]>) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

    /// Executes an instruction using a processor and a given memory.
    pub fn execute<P, T>(
        &self,
        processor: &mut P,
        memory: &mut Memory<T>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        P: Processor,
        T: MPCProtocol,
    {
        todo!();
    }
}
