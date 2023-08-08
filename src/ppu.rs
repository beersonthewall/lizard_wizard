use std::cell::RefCell;
use std::rc::Rc;
use super::cartridge::Mirroring;
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
    pub nmi_signal: Rc<RefCell<bool>>,

    ctrl: CtrlReg,
    status: StatusReg,
    mask: MaskReg,
    mirror: Mirroring,
    buffer: u8,
    address_latch: bool,

    // background state
    name_tables: [u8;2*1024],
    bg_shift_h: u16,
    bg_shift_l: u16,
    at_shift_h: u8,
    at_shit_l: u8,
    at_latch_h: u8,
    at_latch_l: u8,
    fine_x: u8,

    // sprite state
    primary_oam: [u8;1],
    secondary_oam: [u8;1],
    shift_registers: [u8;16],
    sprite_latches: [u8;8],
    counters: [u8;8],

    // rendering state
    cycle: usize,
    scanline: usize,
    frame: usize,
}

impl Ppu {

    pub fn new(nmi_signal: Rc<RefCell<bool>>) -> Self {
	Self {
	    nmi_signal,

	    ctrl: CtrlReg::new(),
	    status: StatusReg::new(),
	    mask: MaskReg::new(),
	    mirror: Mirroring::Horizontal,
	    buffer: 0,
	    address_latch: false,

	    name_tables: [0;2*1024],
	    bg_shift_h: 0,
	    bg_shift_l: 0,
	    at_shift_h: 0,
	    at_shit_l: 0,
	    at_latch_h: 0,
	    at_latch_l: 0,
	    fine_x: 0,

	    primary_oam: [0;1],
	    secondary_oam: [0;1],
	    shift_registers: [0;16],
	    sprite_latches: [0;8],
	    counters: [0;8],

	    cycle: 0,
	    scanline: 0,
	    frame: 0,
	}
    }

    pub fn step(&mut self, _mapper: &dyn Mapper) -> Result<(), EmuErr> { Ok(()) }

    pub fn write(&mut self, addr: u16, data: u8) {
	match addr {
	    0x2000 => self.ctrl.write(data),
	    0x2001 => self.mask.write(data),
	    _ => (),
	}
    }

    pub fn read(&mut self, addr: u16) -> u8 {
	match addr {
	    0x2002 => {
		let res = self.status.read() | (self.buffer & 0b11_111);
		// reading status needs to clear the address latch used by
		// PPUSCROLL & PPUADDR.
		self.address_latch = false;
		res
	    },
	    _ => self.buffer,
	}
    }

    pub fn set_mirror(&mut self, m: Mirroring) { self.mirror = m; }

}

enum NTAddr {
    NT2000,
    NT2400,
    NT2800,
    NT2c00,
}

struct CtrlReg {
    base_nt_addr: NTAddr,
    vram_address_inc: bool,
    sprite_pattern_table_addr: bool,
    bg_pattern_table_addr: bool,
    sprite_sz: bool,
    select: bool,
    nmi: bool,
}

impl CtrlReg {
    fn new() -> Self {
	Self {
	    base_nt_addr: NTAddr::NT2000,
	    vram_address_inc: false,
	    sprite_pattern_table_addr: false,
	    bg_pattern_table_addr: false,
	    sprite_sz: false,
	    select: false,	    
	    nmi: false,
	}
    }

    fn write(&mut self, data: u8) {
	self.base_nt_addr = match data & 0b11 {
	    0 => NTAddr::NT2000,
	    1 => NTAddr::NT2400,
	    2 => NTAddr::NT2800,
	    _ => NTAddr::NT2c00,
	};
	self.vram_address_inc = (data >> 2) & 1 > 0;
	self.sprite_pattern_table_addr = (data >> 3) & 1 > 0;
	self.bg_pattern_table_addr = (data >> 4) & 1 > 0;
	self.sprite_sz = (data >> 5) & 1 > 0;
	self.select = (data >> 6) & 1 > 0;
	self.nmi = (data >> 7) & 1 > 0;
    }
}

struct StatusReg {
    overflow: bool,
    sprite_zero_hit: bool,
    vblank: bool
}

impl StatusReg {
    fn new() -> Self {
	Self {
	    overflow: false,
	    sprite_zero_hit: false,
	    vblank: false,
	}
    }

    fn read(&mut self)  -> u8 {
	let res = (self.vblank as u8) << 7 |
	(self.sprite_zero_hit as u8) << 6 |
	(self.overflow as u8) << 5;

	// reading status reg clears vblank
	self.vblank = false;

	res
    }
}

struct MaskReg {
    grayscale: bool,
    show_bg_left: bool,
    show_sp_left: bool,
    show_bg: bool,
    show_sp: bool,
    em_red: bool,
    em_green: bool,
    em_blue: bool,
}

impl MaskReg {
    fn new() -> Self {
	Self {
	    grayscale: false,
	    show_bg_left: false,
	    show_sp_left: false,
	    show_bg: false,
	    show_sp: false,
	    em_red: false,
	    em_green: false,
	    em_blue: false,
	}
    }

    fn write(&mut self, data: u8) {
	self.grayscale = data & 1 > 0;
	self.show_bg_left = (data >> 1) & 1 > 0;
	self.show_sp_left = (data >> 2) & 1 > 0;
	self.show_bg = (data >> 3) & 1 > 0;
	self.show_sp = (data >> 4) & 1 > 0;
	self.em_red = (data >> 5) & 1 > 0;
	self.em_green = (data >> 6) & 1 > 0;
	self.em_blue = (data >> 7) & 1 > 0;
    }
}
