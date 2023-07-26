use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use super::err::EmuErr;
use super::mapper::MapperType;

pub struct Cartridge {
    header: [u8;16],
    mapper: MapperType,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl std::default::Default for Cartridge {
    fn default() -> Self {
	Self {
	    header: [0;16],
	    mapper: MapperType::NROM,
	    prg_rom: Vec::new(),
	    chr_rom: Vec::new(),
	}
    }
}

impl Cartridge {
    /// Loads an iNES ROM
    ///
    /// Header format:
    /// bytes - what's in it
    /// [0,3] - String literal "NES^Z"
    /// [4]   - Number of 16KiB ROM banks (prg rom)
    /// [5]   - Number of 8KiB VROM banks (chr rom)
    /// [6]   - control byte 1
    /// [7]   - control byte 2
    /// [8]   - size of prg ram in 8KiB units
    /// [9]   - ?
    /// [A,F] - reserved. must be zero.
    pub fn load_rom<P: AsRef<Path>>(rom_path: P) -> Result<Self, EmuErr> {
	let mut file = OpenOptions::new().read(true).open(rom_path).map_err(EmuErr::ReadRom)?;
	let mut header = [0;16];

	file.read_exact(&mut header).map_err(EmuErr::ReadRom)?;

	// check that 'NES' literal is the first four bytes
	let literal = [0x4e, 0x45, 0x53, 0x1a];
	if literal != header[0..4] {
	    return Err(EmuErr::InvalidRom);
	}

	let control_byte_1 = header[6];
	let control_byte_2 = header[7];

	// mapper
	let mapper_lo_nibble = control_byte_1 >> 4;
	let mapper_hi_nibble = control_byte_2 >> 4;
	let mapper_byte = (mapper_hi_nibble << 4) | mapper_lo_nibble;
	let mapper = MapperType::try_from(mapper_byte)?;
	println!("mapper: {:?}", mapper);
	// Find sizes of prg_rom and chr_rom in the header
	// pg rom_sz is the number of 16KB ROM Banks
	let prg_rom_sz = header[4] as usize;
	// chr_rom_sz is the number of 8KB VROM Banks
	let chr_rom_sz = header[5] as usize;

	let prg_rom_sz = prg_rom_sz * 16 * 1024;
	let chr_rom_sz = chr_rom_sz * 8 * 1024;
	let mut prg_rom = vec![0;prg_rom_sz];
	let mut chr_rom = vec![0;chr_rom_sz];

	file.read_exact(&mut prg_rom).map_err(EmuErr::ReadRom)?;
	let addr = (0xFFFC - 0x8000) % 0x4000;
	println!("What's at the reset vec? 0x{:x}, 0x{:x}", prg_rom[addr], prg_rom[addr+1]);
	file.read_exact(&mut chr_rom).map_err(EmuErr::ReadRom)?;

	Ok(Self {
	    header,
	    prg_rom,
	    chr_rom,
	    mapper,
	})
    }

    pub fn read_prg_rom(&self, addr: u16) -> u8 {
	self.prg_rom[addr as usize]
    }

    pub fn read_chr_rom(&self, addr: u16) -> u8 {
	self.chr_rom[addr as usize]
    }

    pub fn mapper(&self) -> MapperType {
	self.mapper
    }

    pub fn prg_rom_sz(&self) -> usize {
	self.prg_rom.len()
    }

    pub fn uses_chr_ram(&self) -> bool {
	self.header[5] == 0
    }
}
