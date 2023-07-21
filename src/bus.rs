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
}
