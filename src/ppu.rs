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
    _palletes: [u8;32],
    // name tables
    _vram: [u8; 2048],
    _oam: [u8;256],
    _interal_buf: u8,

    latch: u8,

    reg_addr: u16,
    reg_addr_write_hi: bool,
}

impl std::default::Default for Ppu {
    fn default() -> Self {
	Ppu {
	    _palletes: [0;32],
	    _vram: [0; 2048],
	    _oam: [0;256],
	    _interal_buf: 0,

	    latch: 0,

	    reg_addr: 0,
	    reg_addr_write_hi: true,
	}
    }
}

impl Ppu {
    pub fn step(&mut self, _mapper: &dyn Mapper) -> Result<(), EmuErr> {
	Ok(())
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
    pub fn read_reg(&mut self, addr: u16) -> u8 {
	match addr % 0x3fff {
	    0x2000 => self.latch,
	    0x2001 => self.latch,
	    0x2003 => self.latch,
	    0x2004 => todo!("read oam data"),
	    0x2005 => self.latch,
	    0x2006 => self.latch,
	    0x2007 => todo!("read ppu data"),
	    0x4014 => self.latch,
	    _ => panic!("PPU register address out of bounds: 0x{:x}", addr),
	}
    }

    pub fn write(&mut self, addr: u16, data: u8) {
	let addr = addr % 0x3fff;
	match addr {
	    0x2006 => self.write_addr(data),
	    _ => panic!("PPU write invalid address 0x{:x}", addr),
	}
    }
}
