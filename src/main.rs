mod cpu;
mod err;
mod memory;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::default();
    cpu.reset();
    cpu.run().unwrap();
}
