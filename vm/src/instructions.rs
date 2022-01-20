#![allow(non_camel_case_types)]
/// These are the instructions for the StoffelMPC VM. These instructions are taken from the MP-SPDZ's VM instructions
/// These instructions may be reduced or added to at a later time.

pub enum Instructions {
    /// Emulation
    CISC = 0x00,
    /// Load/store
    LDI = 0x01,
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
    INITSECURESOCKET = 0x1BA,
    RESPSECURESOCKET = 0x1BB,
}
