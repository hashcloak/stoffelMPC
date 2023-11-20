#![allow(non_camel_case_types)]
/// These are the instructions for the StoffelMPC VM. These instructions are taken from the MP-SPDZ's VM instructions
/// These instructions may be reduced or added to at a later time.
use crate::state::{Memory, Register};
use types::vm::{ImmediateValue, RegisterAddr};

/// Not needed yet ;) #[derive(Debug, Clone, Copy)]
pub type InstructionOpcode = u16;

/// Load/Store Opcodes
pub const OP_LDI: InstructionOpcode = 0x01;
pub const OP_LDSI: InstructionOpcode = 0x02;
pub const OP_LDMC: InstructionOpcode = 0x03;
pub const OP_LDMS: InstructionOpcode = 0x04;
pub const OP_STMC: InstructionOpcode = 0x05;
pub const OP_STMS: InstructionOpcode = 0x06;
pub const OP_LDMCI: InstructionOpcode = 0x07;
pub const OP_LDMSI: InstructionOpcode = 0x08;
pub const OP_STMCI: InstructionOpcode = 0x09;
pub const OP_STMSI: InstructionOpcode = 0x0A;
pub const OP_MOVC: InstructionOpcode = 0x0B;
pub const OP_MOVS: InstructionOpcode = 0x0C;
pub const OP_LDMINT: InstructionOpcode = 0xCA;
pub const OP_STMINT: InstructionOpcode = 0xCB;
pub const OP_LDMINTI: InstructionOpcode = 0xCC;
pub const OP_STMINTI: InstructionOpcode = 0xCD;
pub const OP_PUSHINT: InstructionOpcode = 0xCE;
pub const OP_POPINT: InstructionOpcode = 0xCF;
pub const OP_MOVINT: InstructionOpcode = 0xD0;
/// Machine Opcodes
pub const OP_LDTN: InstructionOpcode = 0x10;
pub const OP_LDARG: InstructionOpcode = 0x11;
pub const OP_REQBL: InstructionOpcode = 0x12;
pub const OP_STARG: InstructionOpcode = 0x13;
pub const OP_TIME: InstructionOpcode = 0x14;
pub const OP_STARTTIMER: InstructionOpcode = 0x15;
pub const OP_STOPTIMER: InstructionOpcode = 0x16;
pub const OP_USE: InstructionOpcode = 0x17;
pub const OP_USE_INP: InstructionOpcode = 0x18;
pub const OP_USE_RUN_TAPE: InstructionOpcode = 0x19;
pub const OP_JOIN_TAPE: InstructionOpcode = 0x1A;
pub const OP_CRASH: InstructionOpcode = 0x1B;
pub const OP_NPLAYERS: InstructionOpcode = 0xE2;
pub const OP_THRESHOLD: InstructionOpcode = 0xE3;
pub const OP_PLAYERID: InstructionOpcode = 0xE4;
pub const OP_USE_EDABIT: InstructionOpcode = 0xE5;
/// Addition
pub const OP_ADDC: InstructionOpcode = 0x20;
pub const OP_ADDS: InstructionOpcode = 0x21;
pub const OP_ADDM: InstructionOpcode = 0x22;
pub const OP_ADDCI: InstructionOpcode = 0x23;
pub const OP_ADDSI: InstructionOpcode = 0x24;
pub const OP_SUBC: InstructionOpcode = 0x25;
pub const OP_SUBS: InstructionOpcode = 0x26;
pub const OP_SUBML: InstructionOpcode = 0x27;
pub const OP_SUBMR: InstructionOpcode = 0x28;
pub const OP_SUBCI: InstructionOpcode = 0x29;
pub const OP_SUBSI: InstructionOpcode = 0x2A;
pub const OP_SUBCFI: InstructionOpcode = 0x2B;
pub const OP_SUBSFI: InstructionOpcode = 0x2C;
/// Multiplication/Division
pub const OP_MULC: InstructionOpcode = 0x30;
pub const OP_MULM: InstructionOpcode = 0x31;
pub const OP_MULCI: InstructionOpcode = 0x32;
pub const OP_MULSI: InstructionOpcode = 0x33;
pub const OP_DIVC: InstructionOpcode = 0x34;
pub const OP_DIVCI: InstructionOpcode = 0x35;
pub const OP_MODC: InstructionOpcode = 0x36;
pub const OP_MODCI: InstructionOpcode = 0x37;
pub const OP_LEGENDREC: InstructionOpcode = 0x38;
pub const OP_DIGESTC: InstructionOpcode = 0x39;
pub const OP_INV2M: InstructionOpcode = 0x3A;
pub const OP_FLOORDIVC: InstructionOpcode = 0x3B;
pub const OP_GMULBITC: InstructionOpcode = 0x136;
pub const OP_MULBITM: InstructionOpcode = 0x137;
/// Open
pub const OP_MULS: InstructionOpcode = 0xA6;
pub const OP_MULRS: InstructionOpcode = 0xA7;
pub const OP_DOTPRODS: InstructionOpcode = 0xA8;
pub const OP_TRUNC_PR: InstructionOpcode = 0xA9;
pub const OP_MATMULS: InstructionOpcode = 0xAA;
pub const OP_MATMULSM: InstructionOpcode = 0xAB;
pub const OP_CHECK: InstructionOpcode = 0xAF;
/// Data access
pub const OP_TRIPLE: InstructionOpcode = 0x50;
pub const OP_BIT: InstructionOpcode = 0x51;
pub const OP_SQUARE: InstructionOpcode = 0x52;
pub const OP_INV: InstructionOpcode = 0x53;
pub const OP_INPUTMASK: InstructionOpcode = 0x56;
pub const OP_INPUTMASKREG: InstructionOpcode = 0x5C;
pub const OP_DABIT: InstructionOpcode = 0x58;
pub const OP_EDABIT: InstructionOpcode = 0x59;
pub const OP_SEDABIT: InstructionOpcode = 0x5A;
pub const OP_RANDOMS: InstructionOpcode = 0x5B;
pub const OP_RANDOMFULLS: InstructionOpcode = 0x5D;
/// Input
pub const OP_INPUT: InstructionOpcode = 0x60;
pub const OP_INPUTFIX: InstructionOpcode = 0xF0;
pub const OP_INPUTFLOAT: InstructionOpcode = 0xF1;
pub const OP_INPUTMIXED: InstructionOpcode = 0xF2;
pub const OP_INPUTMIXEDREG: InstructionOpcode = 0xF3;
pub const OP_RAWINPUT: InstructionOpcode = 0xF4;
pub const OP_INPUTPERSONAL: InstructionOpcode = 0xF5;
pub const OP_STARTINPUT: InstructionOpcode = 0x61;
pub const OP_STOPINPUT: InstructionOpcode = 0x62;
pub const OP_READSOCKETC: InstructionOpcode = 0x63; // Are these even necessary?
pub const OP_READSOCKETS: InstructionOpcode = 0x64; // Are these even necessary?
pub const OP_WRITESOCKETC: InstructionOpcode = 0x65;
pub const OP_WRITESOCKETS: InstructionOpcode = 0x66;
pub const OP_READSOCKETINT: InstructionOpcode = 0x69;
pub const OP_WRITESOCKETINT: InstructionOpcode = 0x6A;
pub const OP_WRITESOCKETSHARE: InstructionOpcode = 0x6B;
pub const OP_LISTEN: InstructionOpcode = 0x6C;
pub const OP_ACCEPTCLIENTCONNECTION: InstructionOpcode = 0x6D;
pub const OP_CLOSECLIENTCONNECTION: InstructionOpcode = 0x6E;
pub const OP_READCLIENTPUBCLICKEY: InstructionOpcode = 0x6F;
/// Bitwise logic
pub const OP_ANDC: InstructionOpcode = 0x70;
pub const OP_XORC: InstructionOpcode = 0x71;
pub const OP_ORC: InstructionOpcode = 0x72;
pub const OP_ANDCI: InstructionOpcode = 0x73;
pub const OP_XORCI: InstructionOpcode = 0x74;
pub const OP_ORCI: InstructionOpcode = 0x75;
pub const OP_NOTC: InstructionOpcode = 0x76;
/// Bitwise shifts
pub const OP_SHLC: InstructionOpcode = 0x80;
pub const OP_SHRC: InstructionOpcode = 0x81;
pub const OP_SHLCI: InstructionOpcode = 0x82;
pub const OP_SHRCI: InstructionOpcode = 0x83;
pub const OP_SHRSI: InstructionOpcode = 0x84;
/// Branching and comparison
pub const OP_JMP: InstructionOpcode = 0x90;
pub const OP_JMPNZ: InstructionOpcode = 0x91;
pub const OP_JMPEQZ: InstructionOpcode = 0x92;
pub const OP_EQZC: InstructionOpcode = 0x93;
pub const OP_LTZC: InstructionOpcode = 0x94;
pub const OP_LTC: InstructionOpcode = 0x95;
pub const OP_GTC: InstructionOpcode = 0x96;
pub const OP_EQC: InstructionOpcode = 0x97;
pub const OP_JMPI: InstructionOpcode = 0x98;
/// Integers
pub const OP_BITDECINT: InstructionOpcode = 0x99;
pub const OP_LDINT: InstructionOpcode = 0x9A;
pub const OP_ADDINT: InstructionOpcode = 0x9B;
pub const OP_SUBINT: InstructionOpcode = 0x9C;
pub const OP_MULINT: InstructionOpcode = 0x9D;
pub const OP_DIVINT: InstructionOpcode = 0x9E;
pub const OP_PRINTINT: InstructionOpcode = 0x9F;
pub const OP_INCINT: InstructionOpcode = 0xD1;
pub const OP_SHUFFLE: InstructionOpcode = 0xD2;
/// Conversion
pub const OP_CONVINT: InstructionOpcode = 0xC0;
pub const OP_CONVMODP: InstructionOpcode = 0xC1;
pub const OP_GCONVGF2N: InstructionOpcode = 0x1C1;
/// IO
pub const OP_PRINTMEM: InstructionOpcode = 0xB0;
pub const OP_PRINTREG: InstructionOpcode = 0xB1;
pub const OP_RAND: InstructionOpcode = 0xB2;
pub const OP_PRINTREGPLAIN: InstructionOpcode = 0xB3;
pub const OP_PRINTCHR: InstructionOpcode = 0xB4;
pub const OP_PRINTSTR: InstructionOpcode = 0xB5;
pub const OP_PUBINT: InstructionOpcode = 0xB6;
pub const OP_RAWOUTPUT: InstructionOpcode = 0xB7;
pub const OP_STARTPRIVATEOUTPUT: InstructionOpcode = 0xB8;
pub const OP_STOPPRIVATEOUTPUT: InstructionOpcode = 0xB9;
pub const OP_PRINTCHRINT: InstructionOpcode = 0xBA;
pub const OP_PRINTSTRINT: InstructionOpcode = 0xBB;
pub const OP_PRINTFLOATPLAIN: InstructionOpcode = 0xBC;
pub const OP_WRITEFILESHARE: InstructionOpcode = 0xBD;
pub const OP_READFILESHARE: InstructionOpcode = 0xBE;
pub const OP_CONDPRINTSTR: InstructionOpcode = 0xBF;
pub const OP_PRINTFLOATPREC: InstructionOpcode = 0xE0;
pub const OP_CONDPRINTPLAIN: InstructionOpcode = 0xE1;
pub const OP_INTOUTPUT: InstructionOpcode = 0xE6;
pub const OP_FLOATOUTPUT: InstructionOpcode = 0xE7;
pub const OP_GBITDEC: InstructionOpcode = 0x184;
pub const OP_GBITCOM: InstructionOpcode = 0x185;
pub const OP_INITSECURESOCKET: InstructionOpcode = 0x1BA;
pub const OP_RESPSECURESOCKET: InstructionOpcode = 0x1BB;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    /// Emulation
    // CISC = 0x00,
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

impl Opcode {}
