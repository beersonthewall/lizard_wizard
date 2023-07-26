mod nrom;

use super::cartridge::Cartridge;
use super::err::EmuErr;
use nrom::MapperNROM;

#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
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

pub fn build_mapper(cartridge: Cartridge) -> Box<dyn Mapper> {
    match cartridge.mapper() {
	MapperType::NROM => Box::new(MapperNROM::new(cartridge)),
    }
}
