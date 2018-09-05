mod cpu;

use cpu::Cpu;

fn main() {
    println!("yo, chip-8");
    let cpu = Cpu::new();
    cpu.start();
}
