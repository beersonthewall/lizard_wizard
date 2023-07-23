use super::bus::Bus;
use super::cpu::Cpu;
use super::err::EmuErr;
use super::ppu::Ppu;
use std::convert::AsRef;
use std::path::Path;

pub struct Emulator {
    cpu: Cpu,
    bus: Bus,
    ppu: Ppu,
}

impl std::default::Default for Emulator {
    fn default() -> Self {
	Self {
	    cpu: Cpu::default(),
	    bus: Bus::default(),
	    ppu: Ppu::default(),
	}
    }
}

impl Emulator {
    pub fn run<P: AsRef<Path>>(&mut self, rom_path: P) -> Result<(), EmuErr> {
	self.bus.load();
	self.cpu.reset(&mut self.bus);
	let mut exit = false;
	while !exit {
	    // 3 ppu ticks per cpu cycle
/*	    self.ppu.step(&mut self.bus)?;
	    self.ppu.step(&mut self.bus)?;
	    self.ppu.step(&mut self.bus)?;*/

	    exit = self.cpu.step(&mut self.bus)?;
	}
	Ok(())
    }
}
