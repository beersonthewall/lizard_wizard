use super::bus::Bus;
use super::cpu::Cpu;
use super::err::EmuErr;
use super::ppu::Ppu;
use std::convert::AsRef;
use std::path::Path;

pub struct Emulator {
    cpu: Cpu,
    memory: Bus,
    ppu: Ppu,
}

impl std::default::Default for Emulator {
    fn default() -> Self {
	Self {
	    cpu: Cpu::default(),
	    memory: Bus::default(),
	    ppu: Ppu::default(),
	}
    }
}

impl Emulator {
    pub fn run<P: AsRef<Path>>(&mut self, _rom_path: P) -> Result<(), EmuErr> {
	self.cpu.reset();

	loop {
	    self.cpu.step(&mut self.memory)?;
	}
    }
}
