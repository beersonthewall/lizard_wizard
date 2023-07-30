use std::convert::AsRef;
use std::path::Path;
use super::bus::Bus;
use super::cpu::Cpu;
use super::err::EmuErr;

#[derive(Default)]
pub struct Emulator {
    cpu: Cpu,
    bus: Bus,
}

impl Emulator {
    pub fn run<P: AsRef<Path>>(&mut self, rom_path: P) -> Result<(), EmuErr> {
	self.bus.load_rom(rom_path)?;
	self.cpu.power_on();
	self.cpu.reset(&mut self.bus);

	let mut exit = false;
	while !exit {
	    // 3 ppu ticks per cpu cycle
	    exit = self.cpu.step(&mut self.bus)?;
	    self.bus.step()?;
	}
	Ok(())
    }
}
