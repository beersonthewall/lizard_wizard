use super::err::EmuErr;
use super::mapper::Mapper;

/// Picture Processing Unit (PPU)
/// https://www.nesdev.org/wiki/PPU
///
/// The PPU has eight memory mapped registers which are exposed to the CPU.
/// https://www.nesdev.org/wiki/PPU_registers.
///
/// In addition the memory mapped registers the PPU has 2KiB of VRAM, 256 bytes
/// of Object Attribute Memory (OAM), and 32 bytes for pallete tables. The chr rom
/// mapped onto the cartridge chr rom.
pub struct Ppu {
    palletes: [u8;32],
    // name tables
    vram: [u8; 2048],
    oam: [u8;256],
    _interal_buf: u8,

    latch: u8,

    reg_addr: u16,
    reg_addr_write_hi: bool,

    reg_status: u8,

    reg_oam_addr: u8,

    reg_ctrl: u8,
}

impl std::default::Default for Ppu {
    fn default() -> Self {
	Ppu {
	    palletes: [0;32],
	    vram: [0; 2048],
	    oam: [0;256],
	    _interal_buf: 0,

	    latch: 0,

	    reg_addr: 0,
	    reg_addr_write_hi: true,

	    reg_status: 0,

	    reg_oam_addr: 0,

	    reg_ctrl: 0,
	}
    }
}

impl Ppu {
    pub fn step(&mut self, _mapper: &dyn Mapper) -> Result<(), EmuErr> {
	Ok(())
    }

    /// Read one of the memory-mapped ppu registers.
    /// 0x2000 - control register
    /// 0x2001 - mask
    /// 0x2002 - ppu status
    /// 0x2003 - oam read/write address
    /// 0x2004 - oam data read/write
    /// 0x2005 - fine scroll position
    /// 0x2006 - ppu read/write address
    /// 0x2007 - ppu data read/write
    /// 0x4014 - oam direct memory access (dma)
    ///
    /// NOTE: The control, mask, oam addr, scroll, ppu address, and oam dma
    /// are write only. However reading these will return the value of
    /// the ppu's internal data buffer (self.latch).
    pub fn read_reg(&mut self, addr: u16, mapper: &dyn Mapper) -> u8 {
	match addr % 0x3fff {
	    0x2000 => self.latch,
	    0x2001 => self.latch,
	    0x2002 => self.read_status(),
	    0x2003 => self.latch,
	    0x2004 => self.read_oam_data(),
	    0x2005 => self.latch,
	    0x2006 => self.latch,
	    0x2007 => self.read_ppu_data(mapper),
	    0x4014 => self.latch,
	    _ => panic!("PPU register address out of bounds: 0x{:x}", addr),
	}
    }

    /// Write to the PPU.
    pub fn write(&mut self, addr: u16, data: u8) {
	let addr = addr % 0x3fff;
	match addr {
	    0x2006 => self.write_addr(data),
	    _ => panic!("PPU write invalid address 0x{:x}", addr),
	}
    }
    
    /// Writes the ppu address register. Alternating between hi and lo byte.
    /// Address range is [0x0000-0x3fff] and address above that range will
    /// be mirrored down.
    fn write_addr(&mut self, byte: u8) {
	if self.reg_addr_write_hi {
	    self.reg_addr = (byte as u16) << 8 | (self.reg_addr & 0xff);
	} else {
	    self.reg_addr = (self.reg_addr & 0xff00) | byte as u16;
	}
	self.reg_addr_write_hi = !self.reg_addr_write_hi;
	self.reg_addr %= 0x3fff;
    }

    /// Reads the PPU status register. Reading this register fills the ppu latch
    /// with the data read. Additional effects: bit 7 in the status register
    /// (vertical blank) is cleared after reading, and the address write 
    /// hi/lo latch is reset.
    fn read_status(&mut self) -> u8 {
	self.latch = self.reg_status;
	let status = self.reg_status;
	self.reg_status &= !(1 << 7);
	self.reg_addr_write_hi = true;
	status
    }

    /// Returns the byte of OAM data at location indicated by reg_oam_addr.
    fn read_oam_data(&mut self) -> u8 {
	let value = self.oam[self.reg_oam_addr as usize];
	self.latch = value;
	value
    }

    /// Read PPU memory.
    ///
    /// Memory Map:
    /// [0x0000,0x0fff] - pattern table 0
    /// [0x1000,0x1fff] - pattern table 1
    /// [0x2000,0x23ff] - name table 0
    /// [0x2400,0x27ff] - name table 1
    /// [0x2800,0x2bff] - name table 2
    /// [0x2c00,0x2fff] - name table 3
    /// [0x3000,0x3eff] - mirrors of [0x2000,0x2efff]
    /// [0x3f00,0x3f1f] - pallete ram indexes
    /// [0x3f20,0x3fff] - mirrors of [0x3f00,0x3f1f]
    fn read_ppu_data(&mut self, mapper: &dyn Mapper) -> u8 {
	let addr = self.reg_addr;
	self.inc_vram_addr();

	match addr {
	    0x0000..=0x1fff => {
		let l = self.latch;
		self.latch = mapper.read_chr(addr);
		l
	    },

	    0x2000..=0x2fff => {
		let l = self.latch;
		self.latch = self.vram[addr as usize];
		l
	    },

	    0x3000..=0x3eff => unimplemented!("reading [0x3000,0x3eff] is unimplemented."),

	    0x3f00..=0x3fff => {
		self.palletes[(addr - 0x3f00) as usize]
	    },

	    _ => panic!("invalid ppu address {:x}", addr),
	}
    }

    /// Increments reg_addr by 32 or 1 if bit 2 in reg_ctrl is set
    /// or unset respectively.
    fn inc_vram_addr(&mut self) {
	if (1 << 2) & self.reg_ctrl > 0 {
	    self.reg_addr = self.reg_addr.wrapping_add(32);	    
	} else {
	    self.reg_addr = self.reg_addr.wrapping_add(1);
	}
    }
}
