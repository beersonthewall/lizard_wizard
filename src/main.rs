mod bus;
mod cpu;
mod emulator;
mod err;
mod opcodes;
mod ppu;

use emulator::Emulator;

fn main() {
    let mut emu = Emulator::default();
    emu.run("./testrom.nes").unwrap();
}
