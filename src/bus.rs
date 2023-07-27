use std::path::Path;
use super::cartridge::Cartridge;
use super::err::EmuErr;
use super::mapper::{Mapper, build_mapper};

pub struct Bus {
    ram: Vec<u8>,
    mapper: Option<Box<dyn Mapper>>,
}

impl std::default::Default for Bus {
    fn default() -> Self {
	Self {
	    // allocate 65K addressable ram
	    ram: [0;u16::MAX as usize].to_vec(),
	    mapper: None,
	}
    }
}

impl Bus {

    pub fn load_rom<P: AsRef<Path>>(&mut self, rom_path: P) -> Result<(), EmuErr> {
	let cartridge = Cartridge::load_rom(rom_path)?;
	let mapper = build_mapper(cartridge);
	self.mapper = Some(mapper);
	Ok(())
    }

    const MEMORY_START: u16 = 0x0;
    const MEMORY_END: u16 = 0x1fff;
    const PPU_CTRL_START: u16 = 0x2000;
    const PPU_CTRL_END: u16 = 0x2007;
    const APU_START: u16 = 0x4000;
    const APU_END: u16 = 0x4017;
    const EXPANSION_START: u16 = 0x4020;
    const EXPANSION_END: u16 = 0x5fff;
    const PRG_ROM_START: u16 = 0x8000;
    const PRG_ROM_END: u16 = 0xffff;
    
    /// matches the address range and reads from the appropriate memory source.
    ///
    /// RAM     - 0x0000 - 0x1fff
    /// APU     - 0x4000 - 0x4017
    /// PRG ROM - 0x4020 - 0xffff
    pub fn read(&self, addr: u16) -> u8 {
	match addr {
	    // addr & 0x07ff (2kib) to implement mirroring
	    // effectively addr % 2KiB
	    Self::MEMORY_START..=Self::MEMORY_END => self.ram[(addr & 0x7ff) as usize],
	    Self::PPU_CTRL_START..=Self::PPU_CTRL_END => todo!("ppu ctrl reg"),
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

    pub fn read_u16(&self, addr: u16) -> u16 {
	let lo = self.read(addr) as u16;
	let hi = self.read(addr + 1) as u16;
	(hi << 8) | lo
    }

    pub fn write(&mut self, addr: u16, data: u8) {
	match addr {
	    // addr & 0x07ff (2kib) to implement mirroring
	    // effectively addr % 2KiB
	    Self::MEMORY_START..=Self::MEMORY_END => self.ram[(addr & 0x7ff) as usize] = data,
	    Self::PRG_ROM_START..=Self::PRG_ROM_END => {
		if let Some(m) = &self.mapper {
		    m.write_prg_rom(addr, data);
		} else {
		    panic!("No mapper");
		}
	    },
	    _ => (),
	}
    }
}
