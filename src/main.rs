mod cpu;
mod err;
mod emulator;
mod memory;
mod ppu;

use emulator::Emulator;

fn main() {
    let mut emu = Emulator::default();
    emu.run("").unwrap();
}
