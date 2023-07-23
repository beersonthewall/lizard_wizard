use super::err::EmuErr;
use super::bus::Bus;
use super::opcodes::{OPCODES,I,AM,Op};

pub struct Cpu {
    // Registers
    reg_pc: u16,
    reg_sp: u8,
    reg_x: u8,
    reg_y: u8,
    reg_a: u8,
    reg_p: u8,

    cycles: usize,

    interrupt: Option<Interrupt>,
}

impl std::default::Default for Cpu {
    fn default() -> Self {
	Self {
	    reg_pc: 0,
	    reg_sp: 0,
	    reg_x: 0,
	    reg_y: 0,
	    reg_a: 0,
	    reg_p: 0,

	    cycles: 0,

	    interrupt: None,
	}
    }
}

/// Macro rule to implement post-increment for a mutable expression.
macro_rules! post_inc {
    ($i:expr) => {
	{
	    let expr = &mut $i;
	    let old = *expr;
	    *expr += 1;
	    old
	}
    };
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    Nmi,
    Brk,
}

impl Cpu {

    const RESET_VECTOR: u16 = 0xFFFC;
    const INITIAL_SP: u8 = 0xFD;

    /// 6502 CPU reset
    ///
    /// - sets pc to 0xFFFC
    /// - sets interrupt disable flag (I)
    /// - sets initial stack pointer to 0xFD
    pub fn reset(&mut self, bus: &mut Bus) {
	self.reg_pc = bus.read_u16(Self::RESET_VECTOR);
	self.set_i(true);
	self.reg_sp = Self::INITIAL_SP;
	self.reg_a = 0;
	self.reg_x = 0;
	self.reg_y = 0;
	self.cycles = 0;
    }

    #[allow(dead_code)]
    pub fn interrupt(&mut self, kind: Interrupt, memory: &mut Bus) {
	self.interrupt = Some(kind);
    }

    /// Jumps to the appropriate interrupt vector:
    ///
    /// - pushes PC and status registers onto the stack
    /// - sets interrupt disabled flag (I)
    /// - picks interrupt vector
    /// - sets pc to that vector
    fn execute_interrupt(&mut self, kind: Interrupt, memory: &mut Bus) {
	if !matches!(kind, Interrupt::Nmi) && self.i() {
	    return;
	}

	self.push((self.reg_pc >> 8) as u8, memory);
	self.push(self.reg_pc as u8, memory);
	self.push(self.reg_p, memory);

	self.set_i(true);

	let addr = match kind {
	    Interrupt::Nmi => 0xFFFE,
	    Interrupt::Brk => 0xFFFF,
	};

	let new_pc = memory.read_u16(addr);
	self.reg_pc = new_pc;
    }

    pub fn step(&mut self, memory: &mut Bus) -> Result<bool, EmuErr> {
	if self.cycles == 0 {
	    if let Some(kind) = self.interrupt {
		self.execute_interrupt(kind, memory);
	    }

	    let opcode: u8 = memory.read(post_inc!(self.reg_pc));
	    let lsd: usize = (opcode & 0x0F) as usize;
	    let msd: usize = ((opcode >> 4) & 0xF) as usize;
	    let instruction = &OPCODES[msd][lsd];
	    self.cycles += instruction.cycles as usize;
	    if self.execute(*instruction, memory)? {
		return Ok(true);
	    }
	}

	self.cycles -= 1;
	Ok(false)
    }

    fn execute(&mut self, instruction: I, bus: &mut Bus) -> Result<bool, EmuErr> {
	println!("{:?}", instruction);
	match instruction {
	    /* logical and arithmetic instructions */

	    // ORA
	    I{ opcode: Op::ORA, addr_mode: AM::IMM, ..} => {
		let location = post_inc!(self.reg_pc);
		self.ora(location, bus);
	    },
	    I{ opcode: Op::ORA, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.ora(location, bus);
	    },
	    I{ opcode: Op::ORA, addr_mode: AM::INX, ..} => {
		let location = self.indexed_indirect(bus);
		self.ora(location, bus);
	    },
	    I{ opcode: Op::ORA, addr_mode: AM::INY, ..} => {
		let location = self.indirect_indexed(bus);
		self.ora(location, bus);
	    },
	    I{ opcode: Op::ORA, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.ora(location, bus);
	    },
	    I{ opcode: Op::ORA, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.ora(location, bus);
	    },
	    I{ opcode: Op::ORA, addr_mode: AM::ABY, ..} => {
		let location = self.absolute_y(bus);
		self.ora(location, bus);
	    },

	    // AND
	    I{ opcode: Op::AND, addr_mode: AM::IMM, ..} => {
		let location = post_inc!(self.reg_pc);
		self.and(location, bus);
	    },
	    I{ opcode: Op::AND, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.and(location, bus);
	    },
	    I{ opcode: Op::AND, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.and(location, bus);
	    },
	    I{ opcode: Op::AND, addr_mode: AM::INX, ..} => {
		let location = self.indexed_indirect(bus);
		self.and(location, bus);
	    },
	    I{ opcode: Op::AND, addr_mode: AM::INY, ..} => {
		let location = self.indirect_indexed(bus);
		self.and(location, bus);
	    },
	    I{ opcode: Op::AND, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.and(location, bus);
	    },
	    I{ opcode: Op::AND, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.and(location, bus);
	    },
	    I{ opcode: Op::AND, addr_mode: AM::ABY, ..} => {
		let location = self.absolute_y(bus);
		self.and(location, bus);
	    },

	    // EOR
	    I{ opcode: Op::EOR, addr_mode: AM::IMM, ..} => {
		let location = post_inc!(self.reg_pc);
		self.eor(location, bus);
	    },
	    I{ opcode: Op::EOR, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.eor(location, bus);
	    },
	    I{ opcode: Op::EOR, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.eor(location, bus);
	    },
	    I{ opcode: Op::EOR, addr_mode: AM::INX, ..} => {
		let location = self.indexed_indirect(bus);
		self.eor(location, bus);
	    },
	    I{ opcode: Op::EOR, addr_mode: AM::INY, ..} => {
		let location = self.indirect_indexed(bus);
		self.eor(location, bus);
	    },
	    I{ opcode: Op::EOR, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.eor(location, bus);
	    },
	    I{ opcode: Op::EOR, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.eor(location, bus);
	    },
	    I{ opcode: Op::EOR, addr_mode: AM::ABY, ..} => {
		let location = self.absolute_y(bus);
		self.eor(location, bus);
	    },

	    // ADC
	    I{ opcode: Op::ADC, addr_mode: AM::IMM, ..} => {
		let location = post_inc!(self.reg_pc);
		self.adc(location, bus);
	    },
	    I{ opcode: Op::ADC, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.adc(location, bus);
	    },
	    I{ opcode: Op::ADC, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.adc(location, bus);
	    },
	    I{ opcode: Op::ADC, addr_mode: AM::INX, ..} => {
		let location = self.indexed_indirect(bus);
		self.adc(location, bus);
	    },
	    I{ opcode: Op::ADC, addr_mode: AM::INY, ..} => {
		let location = self.indirect_indexed(bus);
		self.adc(location, bus);
	    },
	    I{ opcode: Op::ADC, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.adc(location, bus);
	    },
	    I{ opcode: Op::ADC, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.adc(location, bus);
	    },
	    I{ opcode: Op::ADC, addr_mode: AM::ABY, ..} => {
		let location = self.absolute_y(bus);
		self.adc(location, bus);
	    },

	    // SBC
	    I{ opcode: Op::SBC, addr_mode: AM::IMM, ..} => {
		let location = post_inc!(self.reg_pc);
		self.sbc(location, bus);
	    },
	    I{ opcode: Op::SBC, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.sbc(location, bus);
	    },
	    I{ opcode: Op::SBC, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.sbc(location, bus);
	    },
	    I{ opcode: Op::SBC, addr_mode: AM::INX, ..} => {
		let location = self.indexed_indirect(bus);
		self.sbc(location, bus);
	    },
	    I{ opcode: Op::SBC, addr_mode: AM::INY, ..} => {
		let location = self.indirect_indexed(bus);
		self.sbc(location, bus);
	    },
	    I{ opcode: Op::SBC, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.sbc(location, bus);
	    },
	    I{ opcode: Op::SBC, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.sbc(location, bus)
	    },
	    I{ opcode: Op::SBC, addr_mode: AM::ABY, ..} => {
		let location = self.absolute_y(bus);
		self.sbc(location, bus);
	    },

	    // CMP
	    I{ opcode: Op::CMP, addr_mode: AM::IMM, ..} => {
		let location = post_inc!(self.reg_pc);
		self.cmp(self.reg_a, bus.read(location));
	    },
	    I{ opcode: Op::CMP, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.cmp(self.reg_a, bus.read(location));
	    },
	    I{ opcode: Op::CMP, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.cmp(self.reg_a, bus.read(location));
	    },
	    I{ opcode: Op::CMP, addr_mode: AM::INX, ..} => {
		let location = self.indexed_indirect(bus);
		self.cmp(self.reg_a, bus.read(location));
	    },
	    I{ opcode: Op::CMP, addr_mode: AM::INY, ..} => {
		let location = self.indirect_indexed(bus);
		self.cmp(self.reg_a, bus.read(location));
	    },
	    I{ opcode: Op::CMP, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.cmp(self.reg_a, bus.read(location));
	    },
	    I{ opcode: Op::CMP, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.cmp(self.reg_a, bus.read(location));
	    },
	    I{ opcode: Op::CMP, addr_mode: AM::ABY, ..} => {
		let location = self.absolute_y(bus);
		self.cmp(self.reg_a, bus.read(location));
	    },

	    // CPX
	    I{ opcode: Op::CPX, addr_mode: AM::IMM, ..} => {
		let location = post_inc!(self.reg_pc);
		self.cmp(self.reg_x, bus.read(location));
	    },
	    I{ opcode: Op::CPX, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.cmp(self.reg_x, bus.read(location));
	    },
	    I{ opcode: Op::CPX, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.cmp(self.reg_x, bus.read(location));
	    },

	    // CPY
	    I{ opcode: Op::CPY, addr_mode: AM::IMM, ..} => {
		let location = post_inc!(self.reg_pc);
		self.cmp(self.reg_y, bus.read(location));
	    },
	    I{ opcode: Op::CPY, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.cmp(self.reg_y, bus.read(location));
	    },
	    I{ opcode: Op::CPY, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.cmp(self.reg_y, bus.read(location));
	    },

	    // DEC
	    I{ opcode: Op::DEC, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.dec(location, bus);
	    },
	    I{ opcode: Op::DEC, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.dec(location, bus);
	    },
	    I{ opcode: Op::DEC, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.dec(location, bus);
	    },
	    I{ opcode: Op::DEC, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.dec(location, bus);
	    },

	    // DEX
	    I{ opcode: Op::DEX, addr_mode: AM::IMP, ..} => {
		self.reg_x = self.reg_x.wrapping_sub(1);
		self.set_zn(self.reg_x);
	    },

	    // DEY
	    I{ opcode: Op::DEY, addr_mode: AM::IMP, ..} => {
		self.reg_y -= 1;
		self.set_zn(self.reg_y);
	    },

	    // INC
	    I{ opcode: Op::INC, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.inc(location, bus);
	    },
	    I{ opcode: Op::INC, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.inc(location, bus);
	    },
	    I{ opcode: Op::INC, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.inc(location, bus);
	    },
	    I{ opcode: Op::INC, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.inc(location, bus);
	    },

	    // INX
	    I{ opcode: Op::INX, addr_mode: AM::IMP, ..} => {
		self.reg_x += 1;
		self.set_zn(self.reg_x);
	    },

	    // INY
	    I{ opcode: Op::INY, addr_mode: AM::IMP, ..} => {
		self.reg_y += 1;
		self.set_zn(self.reg_y);
	    },

	    // ASL
	    I{ opcode: Op::ASL, addr_mode: AM::IMP, ..} => {
		self.asl_acc();
	    },
	    I{ opcode: Op::ASL, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.asl(location, bus);
	    },
	    I{ opcode: Op::ASL, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.asl(location, bus);
	    },
	    I{ opcode: Op::ASL, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.asl(location, bus);
	    },
	    I{ opcode: Op::ASL, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.asl(location, bus);
	    },

	    // ROL
	    I{ opcode: Op::ROL, addr_mode: AM::IMP, ..} => self.rol_acc(),
	    I{ opcode: Op::ROL, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.rol(location, bus);
	    },
	    I{ opcode: Op::ROL, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.rol(location, bus);
	    },
	    I{ opcode: Op::ROL, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.rol(location, bus);
	    },
	    I{ opcode: Op::ROL, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.rol(location, bus);
	    },

	    // LSR
	    I{ opcode: Op::LSR, addr_mode: AM::IMP, ..} => self.lsr_acc(),
	    I{ opcode: Op::LSR, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.lsr(location, bus);
	    },
	    I{ opcode: Op::LSR, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.lsr(location, bus);
	    },
	    I{ opcode: Op::LSR, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.lsr(location, bus);
	    },
	    I{ opcode: Op::LSR, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.lsr(location, bus);
	    },

	    // ROR
	    I{ opcode: Op::ROR, addr_mode: AM::IMP, ..} => self.ror_acc(),
	    I{ opcode: Op::ROR, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.ror(location, bus);
	    },
	    I{ opcode: Op::ROR, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.ror(location, bus);
	    },
	    I{ opcode: Op::ROR, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.ror(location, bus);
	    },
	    I{ opcode: Op::ROR, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.ror(location, bus);
	    },

	    /* move instructions */

	    // LDA
	    I{ opcode: Op::LDA, addr_mode: AM::IMM, ..} => {
		let location = post_inc!(self.reg_pc);
		self.lda(location, bus);
	    },
	    I{ opcode: Op::LDA, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.lda(location, bus);
	    },
	    I{ opcode: Op::LDA, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.lda(location, bus);
	    },
	    I{ opcode: Op::LDA, addr_mode: AM::INX, ..} => {
		let location = self.indexed_indirect(bus);
		self.lda(location, bus);
	    },
	    I{ opcode: Op::LDA, addr_mode: AM::INY, ..} => {
		let location = self.indirect_indexed(bus);
		self.lda(location, bus);
	    },
	    I{ opcode: Op::LDA, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.lda(location, bus);
	    },
	    I{ opcode: Op::LDA, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.lda(location, bus);
	    },
	    I{ opcode: Op::LDA, addr_mode: AM::ABY, ..} => {
		let location = self.absolute_y(bus);
		self.lda(location, bus);
	    },

	    // STA
	    I{ opcode: Op::STA, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.sta(location, bus);
	    },
	    I{ opcode: Op::STA, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.sta(location, bus);
	    },
	    I{ opcode: Op::STA, addr_mode: AM::INX, ..} => {
		let location = self.indexed_indirect(bus);
		self.sta(location, bus);
	    },
	    I{ opcode: Op::STA, addr_mode: AM::INY, ..} => {
		let location = self.indirect_indexed(bus);
		self.sta(location, bus);
	    },
	    I{ opcode: Op::STA, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.sta(location, bus);
	    },
	    I{ opcode: Op::STA, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.sta(location, bus);
	    },
	    I{ opcode: Op::STA, addr_mode: AM::ABY, ..} => {
		let location = self.absolute_y(bus);
		self.sta(location, bus);
	    },

	    // LDX
	    I{ opcode: Op::LDX, addr_mode: AM::IMM, ..} => {
		let location = post_inc!(self.reg_pc);
		self.ldx(location, bus);
	    },
	    I{ opcode: Op::LDX, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.ldx(location, bus);
	    },
	    I{ opcode: Op::LDX, addr_mode: AM::ZPY, ..} => {
		let location = self.zero_page_y(bus);
		self.ldx(location, bus);
	    },
	    I{ opcode: Op::LDX, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.ldx(location, bus);
	    },
	    I{ opcode: Op::LDX, addr_mode: AM::ABY, ..} => {
		let location = self.absolute_y(bus);
		self.ldx(location, bus);
	    },

	    // STX
	    I{ opcode: Op::STX, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		bus.write(location, self.reg_x);
	    },
	    I{ opcode: Op::STX, addr_mode: AM::ZPY, ..} => {
		let location = self.zero_page_y(bus);
		bus.write(location, self.reg_x);
	    },
	    I{ opcode: Op::STX, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		bus.write(location, self.reg_x);
	    },

	    // LDY
	    I{ opcode: Op::LDY, addr_mode: AM::IMM, ..} => {
		let location = post_inc!(self.reg_pc);
		self.ldy(location, bus);
	    },
	    I{ opcode: Op::LDY, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.ldy(location, bus);
	    },
	    I{ opcode: Op::LDY, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		self.ldy(location, bus);
	    },
	    I{ opcode: Op::LDY, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.ldy(location, bus);
	    },
	    I{ opcode: Op::LDY, addr_mode: AM::ABX, ..} => {
		let location = self.absolute_x(bus);
		self.ldy(location, bus);
	    },

	    // STY
	    I{ opcode: Op::STY, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		bus.write(location, self.reg_y);
	    },
	    I{ opcode: Op::STY, addr_mode: AM::ZPX, ..} => {
		let location = self.zero_page_x(bus);
		bus.write(location, self.reg_y);
	    },
	    I{ opcode: Op::STY, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		bus.write(location, self.reg_y);
	    },

	    // TAX
	    I{ opcode: Op::TAX, addr_mode: AM::IMP, ..} => {
		self.reg_x = self.reg_a;
		self.set_zn(self.reg_x);
	    },

	    // TXA
	    I{ opcode: Op::TXA, addr_mode: AM::IMP, ..} => {
		self.reg_a = self.reg_x;
		self.set_zn(self.reg_a);
	    },

	    // TAY
	    I{ opcode: Op::TAY, addr_mode: AM::IMP, ..} => {
		self.reg_y = self.reg_a;
		self.set_zn(self.reg_y);
	    },

	    // TYA
	    I{ opcode: Op::TYA, addr_mode: AM::IMP, ..} => {
		self.reg_a = self.reg_y;
		self.set_zn(self.reg_a);
	    },

	    // TSX
	    I{ opcode: Op::TSX, addr_mode: AM::IMP, ..} => {
		self.reg_x = self.reg_sp;
		self.set_zn(self.reg_x);
	    },

	    // TXS
	    I{ opcode: Op::TXS, addr_mode: AM::IMP, ..} => self.reg_sp = self.reg_x,

	    // PLA
	    I{ opcode: Op::PLA, addr_mode: AM::IMP, ..} => self.reg_a = self.pull(bus),

	    // PHA
	    I{ opcode: Op::PHA, addr_mode: AM::IMP, ..} => self.push(self.reg_a, bus),

	    // PLP
	    I{ opcode: Op::PLP, addr_mode: AM::IMP, ..} => self.reg_p = self.pull(bus),

	    // PHP
	    I{ opcode: Op::PHP, addr_mode: AM::IMP, ..} => self.push(self.reg_p, bus),

	    /* jump/flag instructions */

	    // BPL
	    I{ opcode: Op::BPL, addr_mode: AM::REL, ..} => self.execute_cond_branch(self.n() == 0, bus),

	    // BMI
	    I{ opcode: Op::BMI, addr_mode: AM::REL, ..} => self.execute_cond_branch(self.n() != 0, bus),

	    // BVC
	    I{ opcode: Op::BVC, addr_mode: AM::REL, ..} => self.execute_cond_branch(self.v() == 0, bus),

	    // BVS
	    I{ opcode: Op::BVS, addr_mode: AM::REL, ..} => self.execute_cond_branch(self.v() != 0, bus),

	    // BCS
	    I{ opcode: Op::BCS, addr_mode: AM::REL, ..} => self.execute_cond_branch(self.c() != 0, bus),

	    // BNE
	    I{ opcode: Op::BNE, addr_mode: AM::REL, ..} => self.execute_cond_branch(self.z() == 0, bus),

	    // BEQ
	    I{ opcode: Op::BEQ, addr_mode: AM::REL, ..} => self.execute_cond_branch(self.z() != 0, bus),
	    
	    // BRK
	    I{ opcode: Op::BRK, addr_mode: AM::IMP, ..} => self.execute_interrupt(Interrupt::Brk, bus),

	    // RTI
	    I{ opcode: Op::RTI, addr_mode: AM::IMP, ..} => {
		self.reg_p = self.pull(bus);
		let pc_lo = self.pull(bus) as u16;
		let pc_hi = self.pull(bus) as u16;
		self.reg_pc = pc_hi << 8 | pc_lo;
	    },

	    // JSR
	    I{ opcode: Op::JSR, addr_mode: AM::ABS, ..} => {
		self.push(((self.reg_pc + 1) >> 8) as u8, bus);
		self.push((self.reg_pc + 1) as u8, bus);
		self.reg_pc = bus.read_u16(self.reg_pc);
		// Don't need to increment pc.
	    },

	    // RTS
	    I{ opcode: Op::RTS, addr_mode: AM::IMP, ..} => {
		let pc_lo = self.pull(bus) as u16;
		let pc_hi = self.pull(bus) as u16;
		self.reg_pc = (pc_hi << 8) | pc_lo;
		self.reg_pc += 1;
	    },

	    // JMP
	    I{ opcode: Op::JMP, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.reg_pc = location;
	    },
	    I{ opcode: Op::JMP, addr_mode: AM::IND, ..} => {
		/*
		Quoted from: https://www.nesdev.org/obelisk-6502-guide/reference.html#INX
		"""
		NB:
		An original 6502 has does not correctly fetch the target address if
		the indirect vector falls on a page boundary (e.g. $xxFF where xx is
		any value from $00 to $FF). In this case fetches the LSB from $xxFF
		as expected but takes the MSB from $xx00. This is fixed in some later
		chips like the 65SC02 so for compatibility always ensure the indirect
		vector is not at the end of the page.
		"""
		 */
		let location = self.absolute(bus);
		let page = location & 0xff00;
		self.reg_pc = bus.read(location) as u16 | ((bus.read(page | (location + 1) & 0xff) as u16) << 8);
	    },

	    // BIT
	    I{ opcode: Op::BIT, addr_mode: AM::ZPG, ..} => {
		let location = self.zero_page(bus);
		self.bit(location, bus);
	    },
	    I{ opcode: Op::BIT, addr_mode: AM::ABS, ..} => {
		let location = self.absolute(bus);
		self.bit(location, bus);
	    },

	    // CLC
	    I{ opcode: Op::CLC, addr_mode: AM::IMP, ..} => self.set_c(false),

	    // SEC
	    I{ opcode: Op::SEC, addr_mode: AM::IMP, ..} => self.set_c(true),

	    // CLD
	    I{ opcode: Op::CLD, addr_mode: AM::IMP, ..} => {}, // decimal flag is disabled on NES

	    // SED
	    I{ opcode: Op::SED, addr_mode: AM::IMP, ..} => {}, // decimal flag is disabled on NES

	    // CLI
	    I{ opcode: Op::CLI, addr_mode: AM::IMP, ..} => self.set_i(false),

	    // SEI
	    I{ opcode: Op::SEI, addr_mode: AM::IMP, ..} => self.set_i(true),

	    // CLV
	    I{ opcode: Op::CLV, addr_mode: AM::IMP, ..} => self.set_v(false),

	    // NOP
	    I{ opcode: Op::NOP, addr_mode: AM::IMP, ..} => {},
	    
	    /* illegal opcodes (most unimplemented for now) */

	    I{ opcode: Op::KIL, .. } => return Ok(true),

	    /* catch all */
	    _ => return Err(EmuErr::UnrecognizedOpCode(0x0)),
	}
	Ok(false)
    }

    fn ora(&mut self, location: u16, bus: &mut Bus) {
	self.reg_a |= bus.read(location);
	self.set_zn(self.reg_a);
    }

    fn and(&mut self, location: u16, bus: &mut Bus) {
	self.reg_a |= bus.read(location);
	self.set_zn(self.reg_a);
    }

    fn eor(&mut self, location: u16, bus: &mut Bus) {
	self.reg_a ^= bus.read(location);
	self.set_zn(self.reg_a);
    }

    fn adc(&mut self, location: u16, bus: &mut Bus) {
	let carry = self.reg_p & 1;
	let (intermediate, o1) = bus.read(location).overflowing_add(carry);
	let (result, o2) = self.reg_a.overflowing_add(intermediate);
	// Overflow
	if o1 || o2 {
	    self.reg_p |= Self::CARRY;
	}
	self.reg_a = result;
	self.set_zn(self.reg_a);
    }

    fn sbc(&mut self, location: u16, bus: &mut Bus) {
	let data = bus.read(location);
	let (intermediate, o1) = self.reg_a.overflowing_sub(data);
	let (result, o2) = intermediate.overflowing_sub(1 - (self.reg_p & Self::CARRY));
	if o1 || o2 {
	    self.reg_p &= !Self::CARRY;
	}
	self.reg_a = result;
    }

    fn cmp(&mut self, fst: u8, snd: u8) {
	let tmp = fst as i16 - snd as i16;
	self.set_z((tmp & 0xFF) as u8);
	self.set_n((tmp & 0x80) as u8);
	self.set_c(fst >= snd);
    }

    fn dec(&mut self, location: u16, bus: &mut Bus) {
	let result = bus.read(location).wrapping_sub(1);
	self.reg_p |= result & (1 << 7) | if result == 0 { 1 } else { 0 };
	self.set_zn(result);
    }

    fn inc(&mut self, location: u16, bus: &mut Bus) {
	let result = bus.read(location).wrapping_add(1);
	self.reg_p |= result & (1 << 7) | if result == 0 { 1 } else { 0 };
	self.set_zn(result);
    }

    fn asl_acc(&mut self) {
	self.set_c((self.reg_a >> 7) & Self::CARRY > 0);
	self.reg_a <<= 1;
	self.set_zn(self.reg_a);
    }

    fn asl(&mut self, location: u16, bus: &mut Bus) {
	let m = bus.read(location);
	self.set_c((m >> 7) & Self::CARRY > 0);
	let m = m << 1;
	bus.write(location, m);
	self.set_zn(m);
    }

    fn rol_acc(&mut self) {
	let carry = self.c();
	self.set_c((self.reg_a >> 7) & Self::CARRY > 0);
	self.reg_a = (self.reg_a >> 1) | (carry << 7);
	self.set_zn(self.reg_a);
    }

    fn rol(&mut self, location: u16, bus: &mut Bus) {
	let carry = self.c();
	let m = bus.read(location);
	self.set_c((m >> 7) & Self::CARRY > 0);
	let m = (m >> 1)| (carry << 7);
	bus.write(location, m);
	self.set_zn(m);
    }

    fn lsr_acc(&mut self) {
	self.set_c(self.reg_a & Self::CARRY > 0);
	self.reg_a >>= 1;
	self.set_zn(self.reg_a);
    }

    fn lsr(&mut self, location: u16, bus: &mut Bus) {
	let m = bus.read(location);
	self.set_c(m & Self::CARRY > 0);
	let m = m >> 1;
	bus.write(location, m);
	self.set_zn(m);
    }

    fn ror_acc(&mut self) {
	let old_zero_bit: u8 = self.reg_a & 1;
	self.reg_a >>= 1;
	self.reg_a |= self.reg_p & (1 << 6);
	self.reg_p |= old_zero_bit;
	self.set_zn(self.reg_a);
    }

    fn ror(&mut self, location: u16, bus: &mut Bus) {
	let mut m = bus.read(location);
	let old_zero_bit: u8 = m & 1;
	m >>= 1;
	m |= self.reg_p & (1 << 6);
	self.reg_p |= old_zero_bit;
	self.set_zn(m);
    }

    fn lda(&mut self, location: u16, bus: &mut Bus) {
	self.reg_a = bus.read(location);
	self.set_zn(self.reg_a);
    }

    fn sta(&mut self, location: u16, bus: &mut Bus) {
	bus.write(location, self.reg_a);
    }

    fn ldx(&mut self, location: u16, bus: &mut Bus) {
	self.reg_x = bus.read(location);
	self.set_zn(self.reg_x);
    }

    fn ldy(&mut self, location: u16, bus: &mut Bus) {
	self.reg_y = bus.read(location);
	self.set_zn(self.reg_y);
    }

    fn bit(&mut self, location: u16, bus: &mut Bus) {
	let m = bus.read(location);
	self.set_v((m >> 6) & 1 > 0);
	self.set_z(m & self.reg_a);
	self.set_n(m);
    }

    /// pushes a value onto the stack
    fn push(&mut self, val: u8, memory: &mut Bus) {
	memory.write(0x100 | self.reg_sp as u16, val);
	self.reg_sp -= 1;
    }

    /// pulls a value off the top of the stack
    fn pull(&mut self, memory: &mut Bus) -> u8 {
	self.reg_sp += 1;
	let val = memory.read(0x100 | self.reg_sp as u16);
	val
    }

    /// BPL, BMI, BVC, BCC, BCS, BNE, BEQ
    fn execute_cond_branch(&mut self, condition: bool, bus: &mut Bus) {
	if condition {
	    let offset = bus.read(post_inc!(self.reg_pc));
	    let offset = offset as i8;
	    // mixed integer ops :)
	    self.reg_pc = self.reg_pc.wrapping_add_signed(offset as i16);
	} else {
	    self.reg_pc += 1;
	}
    }

    /* Addressing mode utilities */

    /// indexed indirect addressing mode resolution
    fn indexed_indirect(&mut self, bus: &mut Bus) -> u16 {
	let base = bus.read(post_inc!(self.reg_pc));
	bus.read_u16(base.wrapping_add(self.reg_x) as u16)
    }

    /// indirect indexed addressing mode resolution
    fn indirect_indexed(&mut self, bus: &mut Bus) -> u16 {
	let addr = bus.read(post_inc!(self.reg_pc));
	addr as u16 + self.reg_y as u16
    }

    /// absolute addressing mode resolution
    fn absolute(&mut self, bus: &mut Bus) -> u16 {
	let val = bus.read_u16(self.reg_pc);
	self.reg_pc += 2;
	val
    }

    /// indexed (by X) absolute addressing
    fn absolute_x(&mut self, bus: &mut Bus) -> u16 {
	let result = bus.read_u16(self.reg_pc) + self.reg_x as u16;
	self.reg_pc += 2;
	result
    }

    /// indexed (by Y) absolute addressing
    fn absolute_y(&mut self, bus: &mut Bus) -> u16 {
	let result = self.reg_y as u16 + bus.read_u16(self.reg_pc);
	self.reg_pc += 2;
	result
    }

    /// zero page addressing mode resolution
    fn zero_page(&mut self, bus: &mut Bus) -> u16 {
	bus.read(post_inc!(self.reg_pc)) as u16
    }

    /// indexed (by X) zero page addressing mode resolution
    fn zero_page_x(&mut self, bus: &mut Bus) -> u16 {
	// Note: If we have LDA $80,X with X = $FF then memory location will be
	// $7F and NOT $017F.
	// Example: LDA $20,X
	bus.read(post_inc!(self.reg_pc)).wrapping_add(self.reg_x) as u16
    }

    fn zero_page_y(&mut self, bus: &mut Bus) -> u16 {
	bus.read(post_inc!(self.reg_pc)).wrapping_add(self.reg_y) as u16
    }

    /* Utilities for manipulating the status register */

    // FIXME: standardize the CPU flags. Should the accessors return bool or u8?

    // status register masks
    const CARRY: u8 = 1;
    const ZERO: u8 = 1 << 1;
    const INTERRUPT_DISABLE: u8 = 1 << 2;
    const _BREAK_CMD: u8 = 1 << 4;
    const OVERFLOW: u8 = 1 << 6;
    const NEGATIVE: u8 = 1 << 7;

    fn c(&self) -> u8 {
	self.reg_p & Self::CARRY
    }

    fn v(&self) -> u8 {
	self.reg_p & Self::OVERFLOW
    }

    fn z(&self) -> u8 {
	self.reg_p & Self::ZERO
    }

    fn n(&self) -> u8 {
	self.reg_p & Self::NEGATIVE
    }

    fn i(&self) -> bool {
	self.reg_p & Self::INTERRUPT_DISABLE > 0
    }

    fn set_c(&mut self, c: bool) {
	if c {
	    self.reg_p |= Self::CARRY;
	} else {
	    self.reg_p &= !Self::CARRY;
	}
    }

    fn set_zn(&mut self, val: u8) {
	self.set_z(val);
	self.set_n(val);
    }

    fn set_n(&mut self, val: u8) {
	let negative = ((val >> 7) & 1) > 0;
	if negative {
	    self.reg_p |= Self::NEGATIVE;
	} else {
	    self.reg_p &= !Self::NEGATIVE;
	}
    }

    fn set_z(&mut self, val: u8) {
	if val == 0 {
	    self.reg_p |= Self::ZERO;
	} else {
	    self.reg_p &= !Self::ZERO;
	}
    }

    fn set_v(&mut self, v: bool) {
	if v {
	    self.reg_p |= Self::OVERFLOW;
	} else {
	    self.reg_p &= !Self::OVERFLOW;
	}
    }

    fn set_i(&mut self, d: bool) {
	if d {
	    self.reg_p |= Self::INTERRUPT_DISABLE;
	} else {
	    self.reg_p &= !Self::INTERRUPT_DISABLE;
	}
    }

    fn compare(&mut self, fst: u8, snd: u8) {
	self.set_zn(fst - snd);
	self.set_c(fst >= snd);
    }
}
