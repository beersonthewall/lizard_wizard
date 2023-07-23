
/// 6502 Opcodes
#[derive(Clone, Copy, Debug)]
pub enum Op {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    XXX, // Illegal instruction

    // Unofficial Opcodes
    AHX,
    ALR,
    ANC,
    ARR,
    AXS,
    DCP,
    ISC,
    KIL,
    LAS,
    LAX,
    RLA,
    RRA,
    SAX,
    SHX,
    SHY,
    SLO,
    SRE,
    TAS,
    XAA,
}

/// Addressing Modes
///
/// - ABS: absolute. fetches the value from a 16-bit address anywhere in memory. 4 cycles
/// - ABX: absolute indexed by X. val = PEEK(arg + X) 4+ cycles
/// - ABY: absolute indexed by Y. val = PEEK(arg + Y) 4+ cycles
/// - IMM: immedaite
/// - IMP: implicit (e.g. RTS or CLC which have no address operand)
/// - IND: indirect (JMP has special addressing mode that can jump to address stored in a 16 bit ptr anywhere in memory)
/// - INX: indexed indirect. val = PEEK(PEEK((arg + X) % 256) + PEEK((arg + X + 1) % 256) * 256) 6 cycles
/// - INY: indirect indexed. val = PEEK(PEEK(arg) + PEEK((arg + 1) % 256) * 256 + Y) 5+ cycles
/// - REL: relative (BEQ, BCS)
/// - ZPG: zero page. Fetches value from an 8 bit addr on zero page
/// - ZPX: zero page indexed by X. val = PEEK((arg + X) % 256)
/// - ZPY: zero page indexed by Y. val = PEEK((arg + Y) % 256)
#[derive(Debug, Clone, Copy)]
pub enum AM {
    ABS,
    ABX,
    ABY,
    IMM,
    IMP,
    IND,
    INX,
    INY,
    REL,
    ZPG,
    ZPX,
    ZPY,
}


/// Instruction
#[derive(Debug, Clone, Copy)]
pub struct I {
    pub opcode: Op,
    pub cycles: u8,
    pub addr_mode: AM,
}

impl I {
    pub const fn new(opcode: Op, cycles: u8, addr_mode: AM) -> Self {
	I {
	    opcode,
	    cycles,
	    addr_mode,
	}
    }
}

/// Table reference: http://www.oxyron.de/html/opcodes02.html
pub const OPCODES: [[I; 16]; 16] = [
[I::new(Op::BRK,7,AM::IMP),I::new(Op::ORA,6,AM::IMP),I::new(Op::KIL,0,AM::IMP),I::new(Op::SLO,8,AM::IMP),I::new(Op::NOP,3,AM::IMP),I::new(Op::ORA,3,AM::IMP),I::new(Op::ASL,5,AM::IMP),I::new(Op::SLO,5,AM::IMP),I::new(Op::PHP,3,AM::IMP),I::new(Op::ORA,2,AM::IMP),I::new(Op::ASL,2,AM::IMP),I::new(Op::ANC,2,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::ORA,4,AM::IMP),I::new(Op::ASL,6,AM::IMP),I::new(Op::SLO,6,AM::IMP),],
[I::new(Op::BPL,2,AM::IMP),I::new(Op::ORA,5,AM::IMP),I::new(Op::KIL,0,AM::IMP),I::new(Op::SLO,8,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::ORA,4,AM::IMP),I::new(Op::ASL,6,AM::IMP),I::new(Op::SLO,6,AM::IMP),I::new(Op::CLC,2,AM::IMP),I::new(Op::ORA,4,AM::IMP),I::new(Op::NOP,2,AM::IMP),I::new(Op::SLO,7,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::ORA,4,AM::IMP),I::new(Op::ASL,7,AM::IMP),I::new(Op::SLO,7,AM::IMP),],
[I::new(Op::JSR,6,AM::IMP),I::new(Op::AND,6,AM::IMP),I::new(Op::KIL,0,AM::IMP),I::new(Op::RLA,8,AM::IMP),I::new(Op::BIT,3,AM::IMP),I::new(Op::AND,3,AM::IMP),I::new(Op::ROL,5,AM::IMP),I::new(Op::RLA,5,AM::IMP),I::new(Op::PLP,4,AM::IMP),I::new(Op::AND,2,AM::IMP),I::new(Op::ROL,2,AM::IMP),I::new(Op::ANC,2,AM::IMP),I::new(Op::BIT,4,AM::IMP),I::new(Op::AND,4,AM::IMP),I::new(Op::ROL,6,AM::IMP),I::new(Op::RLA,6,AM::IMP),],
[I::new(Op::BMI,2,AM::IMP),I::new(Op::AND,5,AM::IMP),I::new(Op::KIL,0,AM::IMP),I::new(Op::RLA,8,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::AND,4,AM::IMP),I::new(Op::ROL,6,AM::IMP),I::new(Op::RLA,6,AM::IMP),I::new(Op::SEC,2,AM::IMP),I::new(Op::AND,4,AM::IMP),I::new(Op::NOP,2,AM::IMP),I::new(Op::RLA,7,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::AND,4,AM::IMP),I::new(Op::ROL,7,AM::IMP),I::new(Op::RLA,7,AM::IMP),],
[I::new(Op::RTI,6,AM::IMP),I::new(Op::EOR,6,AM::IMP),I::new(Op::KIL,0,AM::IMP),I::new(Op::SRE,8,AM::IMP),I::new(Op::NOP,3,AM::IMP),I::new(Op::EOR,3,AM::IMP),I::new(Op::LSR,5,AM::IMP),I::new(Op::SRE,5,AM::IMP),I::new(Op::PHA,3,AM::IMP),I::new(Op::EOR,2,AM::IMP),I::new(Op::LSR,2,AM::IMP),I::new(Op::ALR,2,AM::IMP),I::new(Op::JMP,3,AM::IMP),I::new(Op::EOR,4,AM::IMP),I::new(Op::LSR,6,AM::IMP),I::new(Op::SRE,6,AM::IMP),],
[I::new(Op::BVC,2,AM::IMP),I::new(Op::EOR,5,AM::IMP),I::new(Op::KIL,0,AM::IMP),I::new(Op::SRE,8,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::EOR,4,AM::IMP),I::new(Op::LSR,6,AM::IMP),I::new(Op::SRE,6,AM::IMP),I::new(Op::CLI,2,AM::IMP),I::new(Op::EOR,4,AM::IMP),I::new(Op::NOP,2,AM::IMP),I::new(Op::SRE,7,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::EOR,4,AM::IMP),I::new(Op::LSR,7,AM::IMP),I::new(Op::SRE,7,AM::IMP),],
[I::new(Op::RTS,6,AM::IMP),I::new(Op::ADC,6,AM::IMP),I::new(Op::KIL,0,AM::IMP),I::new(Op::RRA,8,AM::IMP),I::new(Op::NOP,3,AM::IMP),I::new(Op::ADC,3,AM::IMP),I::new(Op::ROR,5,AM::IMP),I::new(Op::RRA,5,AM::IMP),I::new(Op::PLA,4,AM::IMP),I::new(Op::ADC,2,AM::IMP),I::new(Op::ROR,2,AM::IMP),I::new(Op::ARR,2,AM::IMP),I::new(Op::JMP,5,AM::IMP),I::new(Op::ADC,4,AM::IMP),I::new(Op::ROR,6,AM::IMP),I::new(Op::RRA,6,AM::IMP),],
[I::new(Op::BVS,2,AM::IMP),I::new(Op::ADC,5,AM::IMP),I::new(Op::KIL,0,AM::IMP),I::new(Op::RRA,8,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::ADC,4,AM::IMP),I::new(Op::ROR,6,AM::IMP),I::new(Op::RRA,6,AM::IMP),I::new(Op::SEI,2,AM::IMP),I::new(Op::ADC,4,AM::IMP),I::new(Op::NOP,2,AM::IMP),I::new(Op::RRA,7,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::ADC,4,AM::IMP),I::new(Op::ROR,7,AM::IMP),I::new(Op::RRA,7,AM::IMP),],
[I::new(Op::NOP,2,AM::IMP),I::new(Op::STA,6,AM::IMP),I::new(Op::NOP,2,AM::IMP),I::new(Op::SAX,6,AM::IMP),I::new(Op::STY,3,AM::IMP),I::new(Op::STA,3,AM::IMP),I::new(Op::STX,3,AM::IMP),I::new(Op::SAX,3,AM::IMP),I::new(Op::DEY,2,AM::IMP),I::new(Op::NOP,2,AM::IMP),I::new(Op::TXA,2,AM::IMP),I::new(Op::XAA,2,AM::IMP),I::new(Op::STY,4,AM::IMP),I::new(Op::STA,4,AM::IMP),I::new(Op::STX,4,AM::IMP),I::new(Op::SAX,4,AM::IMP),],
[I::new(Op::BCC,2,AM::IMP),I::new(Op::STA,6,AM::IMP),I::new(Op::KIL,0,AM::IMP),I::new(Op::AHX,6,AM::IMP),I::new(Op::STY,4,AM::IMP),I::new(Op::STA,4,AM::IMP),I::new(Op::STX,4,AM::IMP),I::new(Op::SAX,4,AM::IMP),I::new(Op::TYA,2,AM::IMP),I::new(Op::STA,5,AM::IMP),I::new(Op::TXS,2,AM::IMP),I::new(Op::TAS,5,AM::IMP),I::new(Op::SHY,5,AM::IMP),I::new(Op::STA,5,AM::IMP),I::new(Op::SHX,5,AM::IMP),I::new(Op::AHX,5,AM::IMP),],
[I::new(Op::LDY,2,AM::IMP),I::new(Op::LDA,6,AM::IMP),I::new(Op::LDX,2,AM::IMP),I::new(Op::LAX,6,AM::IMP),I::new(Op::LDY,3,AM::IMP),I::new(Op::LDA,3,AM::IMP),I::new(Op::LDX,3,AM::IMP),I::new(Op::LAX,3,AM::IMP),I::new(Op::TAY,2,AM::IMP),I::new(Op::LDA,2,AM::IMP),I::new(Op::TAX,2,AM::IMP),I::new(Op::LAX,2,AM::IMP),I::new(Op::LDY,4,AM::IMP),I::new(Op::LDA,4,AM::IMP),I::new(Op::LDX,4,AM::IMP),I::new(Op::LAX,4,AM::IMP),],
[I::new(Op::BCS,2,AM::IMP),I::new(Op::LDA,5,AM::IMP),I::new(Op::KIL,0,AM::IMP),I::new(Op::LAX,5,AM::IMP),I::new(Op::LDY,4,AM::IMP),I::new(Op::LDA,4,AM::IMP),I::new(Op::LDX,4,AM::IMP),I::new(Op::LAX,4,AM::IMP),I::new(Op::CLV,2,AM::IMP),I::new(Op::LDA,4,AM::IMP),I::new(Op::TSX,2,AM::IMP),I::new(Op::LAS,4,AM::IMP),I::new(Op::LDY,4,AM::IMP),I::new(Op::LDA,4,AM::IMP),I::new(Op::LDX,4,AM::IMP),I::new(Op::LAX,4,AM::IMP),],
[I::new(Op::CPY,2,AM::IMP),I::new(Op::CMP,6,AM::IMP),I::new(Op::NOP,2,AM::IMP),I::new(Op::DCP,8,AM::IMP),I::new(Op::CPY,3,AM::IMP),I::new(Op::CMP,3,AM::IMP),I::new(Op::DEC,5,AM::IMP),I::new(Op::DCP,5,AM::IMP),I::new(Op::INY,2,AM::IMP),I::new(Op::CMP,2,AM::IMP),I::new(Op::DEX,2,AM::IMP),I::new(Op::AXS,2,AM::IMP),I::new(Op::CPY,4,AM::IMP),I::new(Op::CMP,4,AM::IMP),I::new(Op::DEC,6,AM::IMP),I::new(Op::DCP,6,AM::IMP),],
[I::new(Op::BNE,2,AM::IMP),I::new(Op::CMP,5,AM::IMP),I::new(Op::KIL,0,AM::IMP),I::new(Op::DCP,8,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::CMP,4,AM::IMP),I::new(Op::DEC,6,AM::IMP),I::new(Op::DCP,6,AM::IMP),I::new(Op::CLD,2,AM::IMP),I::new(Op::CMP,4,AM::IMP),I::new(Op::NOP,2,AM::IMP),I::new(Op::DCP,7,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::CMP,4,AM::IMP),I::new(Op::DEC,7,AM::IMP),I::new(Op::DCP,7,AM::IMP),],
[I::new(Op::CPX,2,AM::IMP),I::new(Op::SBC,6,AM::IMP),I::new(Op::NOP,2,AM::IMP),I::new(Op::ISC,8,AM::IMP),I::new(Op::CPX,3,AM::IMP),I::new(Op::SBC,3,AM::IMP),I::new(Op::INC,5,AM::IMP),I::new(Op::ISC,5,AM::IMP),I::new(Op::INX,2,AM::IMP),I::new(Op::SBC,2,AM::IMP),I::new(Op::NOP,2,AM::IMP),I::new(Op::SBC,2,AM::IMP),I::new(Op::CPX,4,AM::IMP),I::new(Op::SBC,4,AM::IMP),I::new(Op::INC,6,AM::IMP),I::new(Op::ISC,6,AM::IMP),],
[I::new(Op::BEQ,2,AM::IMP),I::new(Op::SBC,5,AM::IMP),I::new(Op::KIL,0,AM::IMP),I::new(Op::ISC,8,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::SBC,4,AM::IMP),I::new(Op::INC,6,AM::IMP),I::new(Op::ISC,6,AM::IMP),I::new(Op::SED,2,AM::IMP),I::new(Op::SBC,4,AM::IMP),I::new(Op::NOP,2,AM::IMP),I::new(Op::ISC,7,AM::IMP),I::new(Op::NOP,4,AM::IMP),I::new(Op::SBC,4,AM::IMP),I::new(Op::INC,7,AM::IMP),I::new(Op::ISC,7,AM::IMP),],
];
