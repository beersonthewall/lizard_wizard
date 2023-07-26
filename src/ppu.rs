use super::err::EmuErr;
use super::bus::Bus;

/// the ppu has its own 2KB address space
const PPU_ADDR_SPACE_SZ: usize = 2_000;

pub struct Ppu {
    addr_space: Vec<u8>,
}

#[allow(dead_code)]
impl Ppu {
    // the ppu exposes 8 memory mapped registers in the cpu's address space
    const PPU_CTRL: u16 = 0x2000;
    const PPU_MASK: u16 = 0x2001;
    const PPU_STATUS: u16 = 0x2002;
    const OAM_ADDR: u16 = 0x2003;
    const OAM_DATA: u16 = 0x2004;
    const PPU_SCROLL: u16 = 0x2005;
    const PPU_ADDR: u16 = 0x2006;
    const PPU_DATA: u16 = 0x2007;
    const OAM_DMA: u16 = 0x4014;

    // pattern tables store sprite images
    const PAT_TABLE_SZ: u16 = 0x1000;
    const PAT_TABLE_0: u16 = 0x0000;
    const PAT_TABLE_1: u16 = 0x1000;

    // name tables store the background graphics
    const NAME_TABLE_SZ: u16 = 0x400;
    const NAME_TABLE_0: u16 = 0x2000;
    const NAME_TABLE_1: u16 = 0x2400;
    const NAME_TABLE_2: u16 = 0x2800;
    const NAME_TABLE_3: u16 = 0x2C00;

    pub fn step(&mut self, _bus: &mut Bus) -> Result<(), EmuErr> {
	Ok(())
    }
}

impl std::default::Default for Ppu {
    fn default() -> Self {
	Ppu {
	    addr_space: vec![0;PPU_ADDR_SPACE_SZ],
	}
    }
}
