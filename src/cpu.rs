use super::err::EmuErr;
use super::memory::Memory;

pub struct Cpu {
    // Registers
    reg_pc: u16,
    _reg_sp: u8,
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
	    _reg_sp: 0,
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

    // status register masks
    const CARRY: u8 = 1;
    const ZERO: u8 = 1 << 1;
    const _INTERRUPT_DISABLE: u8 = 1 << 2;
    const _BREAK_CMD: u8 = 1 << 4;
    const _OVERFLOW: u8 = 1 << 6;
    const _NEGATIVE: u8 = 1 << 7;
    
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

    /// 'group one' instructions are:
    /// ORA, AND, EOR, ADC, STA, LDA, CMP, SBC
    fn execute_group_one(&mut self, instruction: u8) -> Result<(), EmuErr> {
	let aaa = (instruction >> 5) & 0b111;
	let addressing_mode = (instruction >> 2) & 0b111;
	// FIXME: don't think this is corret or even how this should be done
	let location = match addressing_mode {
	    // indexed indirect
	    // (zero page, X)
	    0b000 => {
		let base = self.memory.read(post_inc!(self.reg_pc));
		let idx = self.reg_x;
		// This should have wrapping add, I think...
		self.memory.read_u16(base.wrapping_add(idx) as u16)
	    },

	    // zero page
	    // Example: LDA $20
	    0b001 => self.memory.read(post_inc!(self.reg_pc)) as u16,

	    // #immediate
	    0b010 => post_inc!(self.reg_pc),

	    // absolute
	    // Example: LDA $32FF
	    0b011 => {
		let val = self.memory.read_u16(self.reg_pc);
		self.reg_pc += 2;
		val
	    },

	    // indirect indexed
	    // (zero page), Y
	    0b100 => {
		let addr = self.memory.read(post_inc!(self.reg_pc));
		addr as u16 + self.reg_y as u16
	    },

	    // zero page, X
	    // Example: LDA $20,X
	    0b101 => (self.reg_x + self.memory.read(post_inc!(self.reg_pc))) as u16,

	    // absolute, Y
	    // Example: LDA $32F0,Y
	    0b110 => self.reg_y as u16 + self.memory.read(post_inc!(self.reg_pc)) as u16,

	    // absolute, X
	    // Example: LDA $32F0,X
	    0b111 => (self.reg_x + self.memory.read(post_inc!(self.reg_pc))) as u16,

	    _ => return Err(EmuErr::UnrecognizedAddressingMode(instruction as u16)),
	};
	match aaa {
	    // ORA
	    0b000 => self.reg_a |= self.memory.read(location),
	    // AND
	    0b001 => self.reg_a |= self.memory.read(location),
	    // EOR
	    0b010 => self.reg_a ^= self.memory.read(location),
	    // ADC
	    0b011 => {
		let carry = self.reg_s & 1;
		let (intermediate, o1) = self.memory.read(location).overflowing_add(carry);
		let (result, o2) = self.reg_a.overflowing_add(intermediate);
		// Overflow
		if o1 || o2 {
		    self.reg_s |= 1;
		}
		self.reg_a = result;
	    },
	    // STA
	    0b100 => self.memory.write(location, self.reg_a),
	    // LDA
	    0b101 => self.reg_a = self.memory.read(location),
	    // CMP
	    0b110 => {
		let other = self.memory.read(location);
		if self.reg_a >= other {
		    // set carry
		    self.reg_s |= Cpu::CARRY;
		}
		if self.reg_a == other {
		    // set zero
		    self.reg_s |= Cpu::ZERO;
		}
	    },
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

    fn execute_group_two(&mut self, _instruction: u8) -> Result<(), EmuErr> {
	Ok(())
    }

    fn execute_group_three(&mut self, _instruction: u8) -> Result<(), EmuErr> {
	Ok(())
    }

    pub fn run(&mut self) -> Result<(), EmuErr> {
	loop {
	    self.step()?;
	}
    }

}
