pub struct Memory {
    bytes: Vec<u8>,
}

impl std::default::Default for Memory {
    fn default() -> Self {
	Self {
	    bytes: Vec::new(),
	}
    }
}

impl Memory {

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
