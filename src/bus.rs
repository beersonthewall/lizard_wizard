use super::err::EmuErr;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

pub struct Bus {
    bytes: Vec<u8>,
}

impl std::default::Default for Bus {
    fn default() -> Self {
	Self {
	    // allocate 65K addressable bytes
	    bytes: [0;u16::MAX as usize].to_vec(),
	}
    }
}

impl Bus {

    pub fn load(&mut self) {
	let game_code = vec![
	    0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02, 0x85,
	    0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9, 0x0f, 0x85,
	    0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85, 0x00, 0xa5, 0xfe,
	    0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20, 0x8d, 0x06, 0x20, 0xc3,
	    0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c, 0x38, 0x06, 0xa5, 0xff, 0xc9,
	    0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0, 0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60,
	    0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85, 0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0,
	    0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01, 0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02,
	    0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05, 0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06,
	    0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00, 0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07,
	    0xe6, 0x03, 0xe6, 0x03, 0x20, 0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06,
	    0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c,
	    0x35, 0x07, 0x60, 0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02,
	    0x4a, 0xb0, 0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9,
	    0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
	    0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10, 0xb0,
	    0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5, 0x10, 0x29,
	    0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe, 0x91, 0x00, 0x60,
	    0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10, 0x60, 0xa2, 0x00, 0xea,
	    0xea, 0xca, 0xd0, 0xfb, 0x60
	];

        self.bytes[0x0600..(0x0600 + game_code.len())].copy_from_slice(&game_code[..]);
	self.write_u16(0xFFFC, 0x600);
	println!("reset vec 0xFFFC {:X}", self.read_u16(0xFFFC));
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, rom_path: P) -> Result<(), EmuErr> {
	let mut opts = OpenOptions::new().read(true).open(rom_path).map_err(EmuErr::ReadRom)?;
	opts.read_to_end(&mut self.bytes).map_err(EmuErr::ReadRom)?;
	Ok(())
    }

    fn reset(&mut self) {
	// Frame IRQ enabled
	self.bytes[0x4017] = 0x00;
	// all channels disabled
	self.bytes[0x4015] = 0x00;
	for i in 0x4010..=0x4013 {
	    self.bytes[i] = 0x00;
	}
	for i in 0x4000..=0x400F {
	    self.bytes[i] = 0x00;
	}
    }

    /// Read a byte from memory at `addr`.
    pub fn read(&self, addr: u16) -> u8 {
	self.bytes[addr as usize]
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
	let bytes: [u8;2] = [self.bytes[addr as usize], self.bytes[addr as usize + 1]];
	u16::from_le_bytes(bytes)
    }

    pub fn write(&mut self, addr: u16, data: u8) {
	self.bytes[addr as usize] = data;
    }

    pub fn write_u16(&mut self, addr: u16, data: u16) {
	let hi = (data >> 8) as u8;
	let lo = (data & 0xFF) as u8;
	self.write(addr, lo);
	self.write(addr+1, hi);
    }
}
