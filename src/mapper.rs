use super::cartridge::Cartridge;
use super::err::EmuErr;

#[derive(Debug, Clone, Copy)]
pub enum MapperType {
    NROM = 0,
}

impl std::convert::TryFrom<u8> for MapperType {
    type Error = EmuErr;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
	match value {
	    0 => Ok(MapperType::NROM),
	    _ => Err(EmuErr::UnsupportedMapperType),
	}
    }
}

pub trait Mapper {
    fn read_prg_rom(&self, addr: u16) -> u8;
    fn write_prg_rom(&self, addr: u16, data: u8);
    fn read_chr(&self, addr: u16) -> u8;
    fn write_chr(&self, addr: u16, data: u8);
}

pub struct MapperNROM {
    cartridge: Cartridge,
    nrom_128: bool,
}

impl Mapper for MapperNROM {
    fn read_prg_rom(&self, addr: u16) -> u8 {
	let mut addr = addr - 0x8000;
	if self.nrom_128 {
	    addr &= 0x3fff;
	}
	self.cartridge.read_prg_rom(addr)
    }

    fn write_prg_rom(&self, addr: u16, data: u8) {
	println!("PRG ROM memory write: addr {:x} data {:x}", addr, data);
    }

    fn read_chr(&self, addr: u16) -> u8 {
	if self.cartridge.uses_chr_ram() {
	    std::todo!("NROM chr ram unimplemented");
	} else {
	    self.cartridge.read_chr_rom(addr)
	}
    }

    fn write_chr(&self, addr: u16, data: u8) {
	if self.cartridge.uses_chr_ram() {
	    std::todo!("NROM chr ram unimplemented");
	} else {
	    println!("CHR ROM memory write: addr {:x} data {:x}", addr, data);
	}
    }
}

impl MapperNROM {
    pub fn new(cartridge: Cartridge) -> Self {
	let nrom_128 = cartridge.prg_rom_sz() == 0x4000;
	Self {
	    cartridge,
	    nrom_128,
	}
    }
}

pub fn build_mapper(cartridge: Cartridge) -> Box<dyn Mapper> {
    match cartridge.mapper() {
	MapperType::NROM => Box::new(MapperNROM::new(cartridge)),
    }
}
