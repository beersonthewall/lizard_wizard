use super::cpu::Cpu;
use std::convert::AsRef;
use std::path::Path;

struct Emulator {
    cpu: Cpu
}

impl std::default::Default for Emulator {
    fn default() -> Self {
	Self {
	    cpu: Cpu::default(),
	}
    }
}

impl Emulator {
    pub fn run<P: AsRef<Path>>(_rom_path: P) -> Result<(), EmuErr> {
	self.cpu.reset();

	loop {
	    self.cpu.step()?;
	}
    }
}
