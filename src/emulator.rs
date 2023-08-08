use std::cell::RefCell;
use std::convert::AsRef;
use std::path::Path;
use std::rc::Rc;
use super::bus::Bus;
use super::controller::Controller;
use super::cpu::Cpu;
use super::err::EmuErr;
use super::ppu::Ppu;

pub struct Emulator {
    cpu: Cpu,
    bus: Bus,
}

impl Emulator {
    pub fn new<F>(_update_game: Box<F>) -> Self
    where F: FnMut (&Ppu, &mut Controller) {
	let nmi_signal: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
	Self {
	    cpu: Cpu::new(nmi_signal.clone()),
	    bus: Bus::new(nmi_signal),
	}
    }

    pub fn init<P: AsRef<Path>>(&mut self, rom_path: P) -> Result<(), EmuErr> {
	self.bus.load_rom(rom_path)?;
	self.cpu.power_on();
	self.cpu.reset(&mut self.bus);

	Ok(())
    }

    pub fn step(&mut self) -> Result<bool, EmuErr> {
	let exit = self.cpu.step(&mut self.bus)?;
	self.bus.step()?;
	Ok(exit)
    }
}
