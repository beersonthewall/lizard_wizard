use super::err::EmuErr;
use super::memory::Memory;

pub struct Cpu {
    // Registers
    reg_pc: u16,
    reg_sp: u8,
    reg_x: u8,
    reg_y: u8,
    reg_a: u8,
    reg_s: u8,
    
    memory: Memory,

    cycles: usize,
}

impl std::default::Default for Cpu {
    fn default() -> Self {
	Self {
	    reg_pc: 0,
	    reg_sp: 0,
	    reg_x: 0,
	    reg_y: 0,
	    reg_a: 0,
	    reg_s: 0,

	    memory: Memory::default(),

	    cycles: 0,
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

impl Cpu {
    
    pub fn _reset(&mut self) {
	// TODO: Implement
    }

    fn step(&mut self) -> Result<(), EmuErr> {
	// An instruction can be 1, 2, or 3 bytes. The first byte always specifies
	// which instruction (opcode).
	// Opcode format: aaabbbcc
	// bits: aaa & cc determine the opcode
	// bits: bbb determine the addressing mode
	// ref: https://llx.com/Neil/a2/opcodes.html
	let instruction = self.memory.read(post_inc!(self.reg_pc));
	self.cycles += 1;
	let cc = instruction & 0b11;

	if self.maybe_single_byte(instruction)? {
	    return Ok(())
	}

	match cc {
	    // group one instructions
	    0b01 => self.execute_group_one(instruction),
	    // group two instructions
	    0b10 => self.execute_group_two(instruction),
	    // group three instructions
	    0b00 => self.execute_group_three(instruction),
	    _ => Err(EmuErr::UnrecognizedOpCode(self.reg_pc)),
	}
    }

    /// pushes a value onto the stack
    fn push(&mut self, val: u8) {
	self.memory.write(0x100 | self.reg_sp as u16, val);
	self.reg_sp -= 1;
    }

    /// pulls a value off the top of the stack
    fn pull(&mut self) -> u8 {
	let val = self.memory.read(0x100 | self.reg_sp as u16);
	self.reg_sp += 1;
	val
    }

    /// Implied addressing mode instructions. May not match.
    fn maybe_single_byte(&mut self, instruction: u8) -> Result<bool, EmuErr> {
	match instruction {
	    // PHP
	    0x08 => self.push(self.reg_s),

	    // PLP
	    0x28 => self.reg_s = self.pull(),

	    // PHA
	    0x48 => self.push(self.reg_a),

	    // PLA
	    0x68 => self.reg_a = self.pull(),

	    // DEY
	    0x88 => {
		self.reg_y -= 1;
		self.set_zn(self.reg_y);
	    },

	    // TAY
	    0xA8 => {
		self.reg_y = self.reg_a;
		self.set_zn(self.reg_y);
	    },

	    // INY
	    0xC8 => {
		self.reg_y += 1;
		self.set_zn(self.reg_y);
	    },

	    // INX
	    0xE8 => {
		self.reg_x += 1;
		self.set_zn(self.reg_x);
	    },

	    // CLC
	    0x18 => self.set_c(false), // clears the carry flag

	    // SEC
	    0x38 => self.set_c(true),

	    // CLI
	    0x58 => self.set_i(false), // clear interrupt disable flag

	    // SEI
	    0x78 => self.set_i(true),

	    // TYA
	    0x98 => {
		self.reg_a = self.reg_y;
		self.set_zn(self.reg_a);
	    },

	    // CLV
	    0xB8 => self.set_v(false), // clears overflow flag

	    // CLD
	    0xD8 => {}, // decimal flag is disabled on NES

	    // SED
	    0xF8 => {}, // decimal flag is disabled on NES

	    // TXA
	    0x8A => {
		self.reg_a = self.reg_x;
		self.set_zn(self.reg_a);
	    },

	    // TXS
	    0x9A => self.reg_sp = self.reg_x,

	    // TAX
	    0xAA => {
		self.reg_x = self.reg_a;
		self.set_zn(self.reg_x);
	    },

	    // TSX
	    0xBA => {
		self.reg_x = self.reg_sp;
		self.set_zn(self.reg_x);
	    },

	    // DEX
	    0xCA => {
		self.reg_x -= 1;
		self.set_zn(self.reg_x);
	    },

	    // NOP
	    0xEA => {
		// be lazy :)
	    },

	    // Didn't find an instruction, let caller know.
	    _ => return Ok(false),
	}
	Ok(true)
    }

    /// 'group one' instructions are:
    /// ORA, AND, EOR, ADC, STA, LDA, CMP, SBC
    fn execute_group_one(&mut self, instruction: u8) -> Result<(), EmuErr> {
	let addressing_mode = (instruction >> 2) & 0b111;
	// FIXME: don't think this is corret or even how this should be done
	let location = match addressing_mode {
	    // indexed indirect
	    // Example: LDA ($20, X)
	    0b000 => self.indexed_indirect(),

	    // zero page
	    // Example: LDA $20
	    0b001 => self.zero_page(),

	    // #immediate
	    // Example: LDA #20
	    0b010 => post_inc!(self.reg_pc),

	    // absolute
	    // Example: LDA $32FF
	    0b011 => self.absolute(),

	    // indirect indexed
	    // (zero page), Y
	    0b100 => self.indirect_indexed(),

	    // zero page, X
	    0b101 => self.zero_page_x(),

	    // absolute, Y
	    // Example: LDA $2000,Y where Y = $92 => loads value at $2092 to acc
	    0b110 => self.absolute_y(),

	    // absolute, X
	    // Example: LDA $32F0,X
	    0b111 => self.absolute_x(),

	    _ => return Err(EmuErr::UnrecognizedAddressingMode(instruction as u16)),
	};

	let aaa = (instruction >> 5) & 0b111;
	match aaa {
	    // ORA
	    0b000 => {
		self.reg_a |= self.memory.read(location);
		self.set_zn(self.reg_a);
	    },
	    // AND
	    0b001 => {
		self.reg_a |= self.memory.read(location);
		self.set_zn(self.reg_a);
	    },
	    // EOR
	    0b010 => {
		self.reg_a ^= self.memory.read(location);
		self.set_zn(self.reg_a);
	    },
	    // ADC
	    0b011 => {
		let carry = self.reg_s & 1;
		let (intermediate, o1) = self.memory.read(location).overflowing_add(carry);
		let (result, o2) = self.reg_a.overflowing_add(intermediate);
		// Overflow
		if o1 || o2 {
		    self.reg_s |= Cpu::CARRY;
		}
		self.reg_a = result;
		self.set_zn(self.reg_a);
	    },
	    // STA
	    0b100 => self.memory.write(location, self.reg_a),
	    // LDA
	    0b101 => {
		self.reg_a = self.memory.read(location);
		self.set_zn(self.reg_a);
	    },
	    // CMP
	    0b110 => self.compare(self.reg_a, self.memory.read(location)),
	    // SBC
	    0b111 => {
		let data = self.memory.read(location);
		let (intermediate, o1) = self.reg_a.overflowing_sub(data);
		let (result, o2) = intermediate.overflowing_sub(1 - (self.reg_s & Cpu::CARRY));
		if o1 || o2 {
		    self.reg_s &= !Cpu::CARRY;
		}
		self.reg_a = result;
	    },
	    _ => return Err(EmuErr::UnrecognizedOpCode(self.reg_pc)),
	}
	Ok(())
    }

    /// group two instructions
    /// ASL, ROL, LSR, ROR, STX, LDX, DEC, INC
    fn execute_group_two(&mut self, instruction: u8) -> Result<(), EmuErr> {
	let addressing_mode = (instruction >> 2) & 0b111;
	let mut is_accumulator = false;
	let location = match addressing_mode {
	    0b000 => self.memory.read(post_inc!(self.reg_pc)) as u16,
	    0b001 => self.zero_page(),
	    0b010 => {
		is_accumulator = true;
		0
	    },
	    0b011 => self.absolute(),
	    0b101 => self.zero_page_x(),
	    0b111 => self.absolute_x(),
	    _ => return Err(EmuErr::UnrecognizedAddressingMode(instruction as u16)),
	};

	let aaa = (instruction >> 5) & 0b111;
	match aaa {
	    // Arithmetic left shift (ASL)
	    0b000 => {
		if is_accumulator {
		    self.set_c((self.reg_a >> 7) & Cpu::CARRY > 0);
		    self.reg_a <<= 1;
		    self.set_zn(self.reg_a);
		} else {
		    let m = self.memory.read(location);
		    self.set_c((m >> 7) & Cpu::CARRY > 0);
		    let m = m << 1;
		    self.memory.write(location, m);
		    self.set_zn(m);
		}
	    },

	    // (ROL)
	    0b001 => {
		let carry = self.c();
		if is_accumulator {
		    self.set_c((self.reg_a >> 7) & Cpu::CARRY > 0);
		    self.reg_a = (self.reg_a >> 1) | (carry << 7);
		    self.set_zn(self.reg_a);
		} else {
		    let m = self.memory.read(location);
		    self.set_c((m >> 7) & Cpu::CARRY > 0);
		    let m = (m >> 1)| (carry << 7);
		    self.memory.write(location, m);
		    self.set_zn(m);
		}
	    },

	    // (LSR)
	    0b010 => {
		if is_accumulator {
		    self.set_c(self.reg_a & Cpu::CARRY > 0);
		    self.reg_a >>= 1;
		    self.set_zn(self.reg_a);
		} else {
		    let m = self.memory.read(location);
		    self.set_c(m & Cpu::CARRY > 0);
		    let m = m >> 1;
		    self.memory.write(location, m);
		    self.set_zn(m);
		}
	    },

	    // ROR
	    0b011 => {
		if is_accumulator {
		    let old_zero_bit: u8 = self.reg_a & 1;
		    self.reg_a >>= 1;
		    self.reg_a |= self.reg_s & (1 << 6);
		    self.reg_s |= old_zero_bit;
		    self.set_zn(self.reg_a);
		} else {
		    let mut m = self.memory.read(location);
		    let old_zero_bit: u8 = m & 1;
		    m >>= 1;
		    m |= self.reg_s & (1 << 6);
		    self.reg_s |= old_zero_bit;
		    self.set_zn(m);
		}
	    },

	    // STX
	    0b100 => self.memory.write(location, self.reg_x),

	    // LDX
	    0b101 => {
		self.reg_x = self.memory.read(location);
		self.set_zn(self.reg_x);
	    },

	    // DEC
	    0b110 => {
		let result = self.memory.read(location).wrapping_sub(1);
		self.reg_s |= result & (1 << 7) | if result == 0 { 1 } else { 0 };
		self.set_zn(result);
	    },

	    // INC
	    0b111 => {
		let result = self.memory.read(location).wrapping_add(1);
		self.reg_s |= result & (1 << 7) | if result == 0 { 1 } else { 0 };
		self.set_zn(result);
	    },

	    _ => return Err(EmuErr::UnrecognizedOpCode(self.reg_pc)),
	};
	Ok(())
    }

    /// group three instructions
    /// BIT, JMP, JMP (abs), STY, LDY, CPY, CPX
    // aaabbbcc
    fn execute_group_three(&mut self, instruction: u8) -> Result<(), EmuErr> {
	let addressing_mode = (instruction >> 2) & 0b111;
	let mut is_conditional_branch = false;
	let location = match addressing_mode {
	    0b000 => post_inc!(self.reg_pc),
	    0b001 => self.zero_page(),
	    0b011 => self.absolute(),
	    0b101 => self.zero_page_x(),
	    0b111 => self.absolute_x(),
	    0b100 => {
		is_conditional_branch = true;
		0
	    }
	    _ => return Err(EmuErr::UnrecognizedAddressingMode(self.reg_pc)),
	};

	if is_conditional_branch {
	    return self.execute_cond_branch(instruction);
	}

	let aaa: u8 = (instruction >> 5) & 0b111;
	match aaa {
	    // BIT
	    0b001 => {
		let m = self.memory.read(location);
		self.set_v((m >> 6) & 1 > 0);
		self.set_z(m & self.reg_a);
		self.set_n(m);
	    },

	    // JMP
	    0b010 => {
		self.reg_pc = location;
	    },

	    // JMP (abs)
	    0b011 => {},

	    // STY
	    0b100 => self.memory.write(location, self.reg_y),

	    // LDY
	    0b101 => {
		self.reg_y = self.memory.read(location);
		self.set_zn(self.reg_y);
	    },

	    // CPY
	    0b110 => self.compare(self.reg_x, self.memory.read(location)),

	    // CPX
	    0b111 => self.compare(self.reg_x, self.memory.read(location)),

	    _ => return Err(EmuErr::UnrecognizedOpCode(self.reg_pc)),
	};
	Ok(())
    }

    /// BPL, BMI, BVC, BCC, BCS, BNE, BEQ
    fn execute_cond_branch(&mut self, instruction: u8) -> Result<(), EmuErr> {
	let flag = match (instruction >> 6) & 0b11 {
	    0b00 => self.n() > 0,
	    0b01 => self.v() > 0,
	    0b10 => self.c() > 0,
	    0b11 => self.z() > 0,
	    _ => return Err(EmuErr::UnrecognizedCondBranchFlag(self.reg_pc)),
	};
	let val = (instruction >> 5) & 1 > 0;

	if flag == val {
	    let offset = self.memory.read(post_inc!(self.reg_pc));
	    let offset = offset as i8;
	    // mixed integer ops :)
	    self.reg_pc = self.reg_pc.wrapping_add_signed(offset as i16);
	}

	Ok(())
    }

    /* Addressing mode utilities */

    /// indexed indirect addressing mode resolution
    fn indexed_indirect(&mut self) -> u16 {
	let base = self.memory.read(post_inc!(self.reg_pc));
	self.memory.read_u16(base.wrapping_add(self.reg_x) as u16)
    }

    /// indirect indexed addressing mode resolution
    fn indirect_indexed(&mut self) -> u16 {
	let addr = self.memory.read(post_inc!(self.reg_pc));
	addr as u16 + self.reg_y as u16
    }

    /// absolute addressing mode resolution
    fn absolute(&mut self) -> u16 {
	let val = self.memory.read_u16(self.reg_pc);
	self.reg_pc += 2;
	val
    }

    /// indexed (by X) absolute addressing
    fn absolute_x(&mut self) -> u16 {
	let result = self.memory.read_u16(self.reg_pc) + self.reg_x as u16;
	self.reg_pc += 2;
	result
    }

    /// indexed (by Y) absolute addressing
    fn absolute_y(&mut self) -> u16 {
	let result = self.reg_y as u16 + self.memory.read_u16(self.reg_pc);
	self.reg_pc += 2;
	result
    }

    /// zero page addressing mode resolution
    fn zero_page(&mut self) -> u16 {
	self.memory.read(post_inc!(self.reg_pc)) as u16
    }

    /// indexed (by X) zero page addressing mode resolution
    fn zero_page_x(&mut self) -> u16 {
	// Note: If we have LDA $80,X with X = $FF then memory location will be
	// $7F and NOT $017F.
	// Example: LDA $20,X
	self.memory.read(post_inc!(self.reg_pc)).wrapping_add(self.reg_x) as u16
    }

    /* Utilities for manipulating the status register */
    
    // status register masks
    const CARRY: u8 = 1;
    const ZERO: u8 = 1 << 1;
    const INTERRUPT_DISABLE: u8 = 1 << 2;
    const _BREAK_CMD: u8 = 1 << 4;
    const OVERFLOW: u8 = 1 << 6;
    const NEGATIVE: u8 = 1 << 7;

    fn c(&self) -> u8 {
	self.reg_s & Cpu::CARRY
    }

    fn v(&self) -> u8 {
	self.reg_s & Cpu::OVERFLOW
    }

    fn z(&self) -> u8 {
	self.reg_s & Cpu::ZERO
    }

    fn n(&self) -> u8 {
	self.reg_s & Cpu::NEGATIVE
    }

    fn set_c(&mut self, c: bool) {
	if c {
	    self.reg_s |= Cpu::CARRY;
	} else {
	    self.reg_s &= !Cpu::CARRY;
	}
    }

    fn set_zn(&mut self, val: u8) {
	self.set_z(val);
	self.set_n(val);
    }

    fn set_n(&mut self, val: u8) {
	let negative = ((val >> 7) & 1) > 0;
	if negative {
	    self.reg_s |= Cpu::NEGATIVE;
	} else {
	    self.reg_s &= !Cpu::NEGATIVE;
	}
    }

    fn set_z(&mut self, val: u8) {
	if val == 0 {
	    self.reg_s |= Cpu::ZERO;
	} else {
	    self.reg_s &= !Cpu::ZERO;
	}
    }

    fn set_v(&mut self, v: bool) {
	if v {
	    self.reg_s |= Cpu::OVERFLOW;
	} else {
	    self.reg_s &= !Cpu::OVERFLOW;
	}
    }

    fn set_i(&mut self, d: bool) {
	if d {
	    self.reg_s |= Cpu::INTERRUPT_DISABLE;
	} else {
	    self.reg_s &= !Cpu::INTERRUPT_DISABLE;
	}
    }

    fn compare(&mut self, fst: u8, snd: u8) {
	self.set_zn(fst - snd);
	self.set_c(fst >= snd);
    }

    /// Run the CPU
    pub fn run(&mut self) -> Result<(), EmuErr> {
	loop {
	    self.step()?;
	}
    }
}
