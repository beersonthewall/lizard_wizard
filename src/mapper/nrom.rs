use crate::mapper::Mapper;
use crate::cartridge::Cartridge;

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
