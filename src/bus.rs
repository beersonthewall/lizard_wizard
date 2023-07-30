use std::path::Path;
use super::cartridge::Cartridge;
use super::err::EmuErr;
use super::mapper::{Mapper, build_mapper};
use super::ppu::Ppu;

pub struct Bus {
    ram: Vec<u8>,
    mapper: Option<Box<dyn Mapper>>,
    ppu: Ppu,
}

impl std::default::Default for Bus {
    fn default() -> Self {
	Self {
	    // allocate 65K addressable ram
	    ram: [0;u16::MAX as usize].to_vec(),
	    mapper: None,
	    ppu: Ppu::default(),
	}
    }
}

impl Bus {

    /// Ticks ppu once
    pub fn step(&mut self) -> Result<(), EmuErr> {
	if let Some(m) = &self.mapper {
	    return self.ppu.step(m.as_ref());
	}
	Ok(())
    }

    /// Loads an iNES rom file, constructing the appropriate mapper based on
    /// parsed header information.
    pub fn load_rom<P: AsRef<Path>>(&mut self, rom_path: P) -> Result<(), EmuErr> {
	let cartridge = Cartridge::load_rom(rom_path)?;
	let mapper = build_mapper(cartridge);
	self.mapper = Some(mapper);
	Ok(())
    }

    const MEMORY_START: u16 = 0x0;
    const MEMORY_END: u16 = 0x1fff;

    const PPU_START: u16 = 0x2000;
    const PPU_END: u16 = 0x3fff;
    const OAM_DMA: u16 = 0x4014;

    const APU_START: u16 = 0x4000;
    const APU_END: u16 = 0x4017;
    const EXPANSION_START: u16 = 0x4020;
    const EXPANSION_END: u16 = 0x5fff;
    const PRG_ROM_START: u16 = 0x8000;
    const PRG_ROM_END: u16 = 0xffff;
    
    /// Matches the address range and reads from the appropriate memory source.
    ///
    /// [0x0000,0x07ff] - CPU internal ram
    /// [0x0800,0x1fff] - ram mirrors
    /// [0x2000,0x2007] - ppu registers
    /// [0x2008,0x3fff] - ppu register mirrors
    /// [0x4000,0x4017] - apu & I/O registers
    /// [0x4018,0x401f] - apu & I/O functionality which is normally disabled
    /// [0x4020,0xffff] - catridge space: prg rom, prg ram, and mapper regsiters
    pub fn read(&mut self, addr: u16) -> u8 {
	match addr {
	    // addr & 0x07ff (2kib) to implement mirroring
	    // effectively addr % 2KiB
	    Self::MEMORY_START..=Self::MEMORY_END => self.ram[(addr & 0x7ff) as usize],
	    // PPU memory-mapped registers are [0x2000,0x2007] and mirrored every 8 bytes
	    // [0x2008,0x3fff]
	    Self::PPU_START..=Self::PPU_END => self.ppu.read_reg(addr),
	    // TODO OAM DMA and APU range intersect. How to handle this better?
	    Self::OAM_DMA => self.ppu.read_reg(addr),
	    Self::APU_START..=Self::APU_END => todo!("apu mem"),
	    Self::EXPANSION_START..=Self::EXPANSION_END => todo!("cartridge expansion rom"),
	    Self::PRG_ROM_START..=Self::PRG_ROM_END => {
		if let Some(m) = &self.mapper {
		    m.read_prg_rom(addr)
		} else {
		    panic!("No mapper");
		}
	    },
	    _ => 0,
	}
    }

    pub fn read_u16(&mut self, addr: u16) -> u16 {
	let lo = self.read(addr) as u16;
	let hi = self.read(addr + 1) as u16;
	(hi << 8) | lo
    }

    /// Matches the address range and writes to the appropriate memory.
    ///
    /// [0x0000,0x07ff] - CPU internal ram
    /// [0x0800,0x1fff] - ram mirrors
    /// [0x2000,0x2007] - ppu registers
    /// [0x2008,0x3fff] - ppu register mirrors
    /// [0x4000,0x4017] - apu & I/O registers, notably the OAM direct memory access register is 0x4014
    /// [0x4018,0x401f] - apu & I/O functionality which is normally disabled
    /// [0x4020,0xffff] - catridge space: prg rom, prg ram, and mapper regsiters
    pub fn write(&mut self, addr: u16, data: u8) {
	match addr {
	    // addr & 0x07ff (2kib) to implement mirroring
	    // effectively addr % 2KiB
	    Self::MEMORY_START..=Self::MEMORY_END => self.ram[(addr & 0x7ff) as usize] = data,
	    Self::PPU_START..=Self::PPU_END => self.ppu.write(addr, data),
	    Self::PRG_ROM_START..=Self::PRG_ROM_END => panic!("prg rom write attempt"),
	    _ => (),
	}
    }
}
