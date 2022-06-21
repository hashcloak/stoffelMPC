#![allow(non_camel_case_types)]
/// These are the instructions for the StoffelMPC VM. These instructions are taken from the MP-SPDZ's VM instructions
/// These instructions may be reduced or added to at a later time.

#[derive(Debug, Clone, Copy)]
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
pub const OP_PROTECTMEMS: InstructionOpcode = 0x0D;
pub const OP_PROTECTMEMC: InstructionOpcode = 0x0E;
pub const OP_PROTECTMEMINT: InstructionOpcode = 0x0F;
pub const OP_LDMINT: InstructionOpcode = 0xCA;
pub const OP_STMINT: InstructionOpcode = 0xCB;
pub const OP_LDMINTI: InstructionOpcode = 0xCC;
pub const OP_STMINTI: InstructionOpcode = 0xCD;
pub const OP_PUSHINT: InstructionOpcode = 0xCE;
pub const OP_POPINT: InstructionOpcode = 0xCF
pub const OP_MOVINT: InstructionOpcode = 0xD0;
/// Machine Opcodes
pub const OP_LDTN: InstructionOpcode = 0x10;
pub const OP_LDARG: InstructionOpcode = 0x11;
pub const OP_REQBL: InstructionOpcode = 0x12;
pub const OP_STARG: InstructionOpcode = 0x13;
pub const OP_TIME: InstructionOpcode = 0x14;
pub const OP_START: InstructionOpcode = 0x15;
pub const OP_STOP: InstructionOpcode = 0x16;
pub const OP_USE: InstructionOpcode = 0x17;
pub const OP_USE_INP: InstructionOpcode = 0x18;
pub const OP_USE_RUN_TAPE: InstructionOpcode = 0x19;
pub const OP_JOIN_TAPE: InstructionOpcode = 0x1A;
pub const OP_CRASH: InstructionOpcode = 0x1B;
pub const OP_USE_PREP: InstructionOpcode = 0x1C;
pub const OP_STARTGRIND: InstructionOpcode = 0x1D; // May not be necessary
pub const OP_STOPGRIND: InstructionOpcode = 0x1E; // May not be necessary
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
pub const OP_OPEN: InstructionOpcode = 0xA5;
pub const OP_MULS: InstructionOpcode = 0xA6;
pub const OP_MULRS: InstructionOpcode = 0xA7;
pub const OP_DOTPRODS: InstructionOpcode = 0xA8;
pub const OP_TRUNC_PR: InstructionOpcode = 0xA9;
pub const OP_MATMULS: InstructionOpcode = 0xAA;
pub const OP_MATMULSM: InstructionOpcode = 0xAB;
pub const OP_CONV2DS: InstructionOpcode = 0xAC;
pub const OP_CHECK: InstructionOpcode = 0xAF;
/// Data access
pub const OP_TRIPLE: InstructionOpcode = 0x50;
pub const OP_BIT: InstructionOpcode = 0x51;
pub const OP_SQUARE: InstructionOpcode = 0x52;
pub const OP_INV: InstructionOpcode = 0x53;
pub const OP_GBITTRIPLE: InstructionOpcode = 0x154;
pub const OP_GBITGF2NTRIPLE: InstructionOpcode = 0x155;
pub const OP_INPUTMASK: InstructionOpcode = 0x56;
pub const OP_INPUTMASKREG: InstructionOpcode = 0x5C;
pub const OP_PREP: InstructionOpcode = 0x57;
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
    CISC = 0x00,
    /// Load/store
    /// LDI(bytecode as a hex string, immediate value, register position)
    LDI(InstructionOpcode, u64, usize),
    LDSI = 0x02,
    LDMC = 0x03,
    LDMS = 0x04,
    STMC = 0x05,
    STMS = 0x06,
    LDMCI = 0x07,
    LDMSI = 0x08,
    STMCI = 0x09,
    STMSI = 0x0A,
    MOVC = 0x0B,
    MOVS = 0x0C,
    PROTECTMEMS = 0x0D,
    PROTECTMEMC = 0x0E,
    PROTECTMEMINT = 0xF,
    LDMINT = 0xCA,
    STMINT = 0xCB,
    LDMINTI = 0xCC,
    STMINTI = 0xCD,
    PUSHINT = 0xCE,
    POPINT = 0xCF,
    MOVINT = 0xD0,
    /// Machine
    LDTN = 0x10,
    LDARG = 0x11,
    REQBL = 0x12,
    STARG = 0x13,
    TIME = 0x14,
    START = 0x15,
    STOP = 0x16,
    USE = 0x17,
    USE_INP = 0x18,
    RUN_TAPE = 0x19,
    JOIN_TAPE = 0x1A,
    CRASH = 0x1B,
    USE_PREP = 0x1C,
    STARTGRIND = 0x1D,
    STOPGRIND = 0x1E,
    NPLAYERS = 0xE2,
    THRESHOLD = 0xE3,
    PLAYERID = 0xE4,
    USE_EDABIT = 0xE5,
    /// Addition
    ADDC = 0x20,
    ADDS = 0x21,
    ADDM = 0x22,
    ADDCI = 0x23,
    ADDSI = 0x24,
    SUBC = 0x25,
    SUBS = 0x26,
    SUBML = 0x27,
    SUBMR = 0x28,
    SUBCI = 0x29,
    SUBSI = 0x2A,
    SUBCFI = 0x2B,
    SUBSFI = 0x2C,
    /// Multiplication/division
    MULC = 0x30,
    MULM = 0x31,
    MULCI = 0x32,
    MULSI = 0x33,
    DIVC = 0x34,
    DIVCI = 0x35,
    MODC = 0x36,
    MODCI = 0x37,
    LEGENDREC = 0x38,
    DIGESTC = 0x39,
    INV2M = 0x3a,
    FLOORDIVC = 0x3b,
    GMULBITC = 0x136,
    GMULBITM = 0x137,
    /// Open
    OPEN = 0xA5,
    MULS = 0xA6,
    MULRS = 0xA7,
    DOTPRODS = 0xA8,
    TRUNC_PR = 0xA9,
    MATMULS = 0xAA,
    MATMULSM = 0xAB,
    CONV2DS = 0xAC,
    CHECK = 0xAF,
    /// Data access
    TRIPLE = 0x50,
    BIT = 0x51,
    SQUARE = 0x52,
    INV = 0x53,
    GBITTRIPLE = 0x154,
    GBITGF2NTRIPLE = 0x155,
    INPUTMASK = 0x56,
    INPUTMASKREG = 0x5C,
    PREP = 0x57,
    DABIT = 0x58,
    EDABIT = 0x59,
    SEDABIT = 0x5A,
    RANDOMS = 0x5B,
    RANDOMFULLS = 0x5D,
    /// Input
    INPUT = 0x60,
    INPUTFIX = 0xF0,
    INPUTFLOAT = 0xF1,
    INPUTMIXED = 0xF2,
    INPUTMIXEDREG = 0xF3,
    RAWINPUT = 0xF4,
    INPUTPERSONAL = 0xF5,
    STARTIPUT = 0x61,
    STOPINPUT = 0x62,
    READSOCKETC = 0x63,
    READSOCKETS = 0x64,
    WRITESOCKETC = 0x65,
    WRITESOCKETS = 0x66,
    READSOCKETINT = 0x69,
    WRITESOCKETINT = 0x6a,
    WRITESOCKETSHARE = 0x6b,
    LISTEN = 0x6c,
    ACCEPTCLIENTCONNECTION = 0x6d,
    CLOSECLIENTCONNECTION = 0x6e,
    READCLIENTPUBLICKEY = 0x6f,
    /// Bitwise logic
    ANDC = 0x70,
    XORC = 0x71,
    ORC = 0x72,
    ANDCI = 0x73,
    XORCI = 0x74,
    ORCI = 0x75,
    NOTC = 0x76,
    /// Bitwise shifts
    SHLC = 0x80,
    SHRC = 0x81,
    SHLCI = 0x82,
    SHRCI = 0x83,
    SHRSI = 0x84,
    /// Branching and comparison
    JMP = 0x90,
    JMPNZ = 0x91,
    JMPEQZ = 0x92,
    EQZC = 0x93,
    LTZC = 0x94,
    LTC = 0x95,
    GTC = 0x96,
    EQC = 0x97,
    JMPI = 0x98,
    /// Integers
    BITDECINT = 0x99,
    LDINT = 0x9A,
    ADDINT = 0x9B,
    SUBINT = 0x9C,
    MULINT = 0x9D,
    DIVINT = 0x9E,
    PRINTINT = 0x9F,
    INCINT = 0xD1,
    SHUFFLE = 0xD2,
    /// Conversion
    CONVINT = 0xC0,
    CONVMODP = 0xC1,
    GCONVGF2N = 0x1C1,
    /// IO
    PRINTMEM = 0xB0,
    PRINTREG = 0xB1,
    RAND = 0xB2,
    PRINTREGPLAIN = 0xB3,
    PRINTCHR = 0xB4,
    PRINTSTR = 0xB5,
    PUBINPUT = 0xB6,
    RAWOUTPUT = 0xB7,
    STARTPRIVATEOUTPUT = 0xB8,
    STOPPRIVATEOUTPUT = 0xB9,
    PRINTCHRINT = 0xBA,
    PRINTSTRINT = 0xBB,
    PRINTFLOATPLAIN = 0xBC,
    WRITEFILESHARE = 0xBD,
    READFILESHARE = 0xBE,
    CONDPRINTSTR = 0xBF,
    PRINTFLOATPREC = 0xE0,
    CONDPRINTPLAIN = 0xE1,
    INTOUTPUT = 0xE6,
    FLOATOUTPUT = 0xE7,
    GBITDEC = 0x184,
    GBITCOM = 0x185,
    /// Secure socket
    INITSECURESOCKET() = 0x1BA,
    RESPSECURESOCKET = 0x1BB,
}