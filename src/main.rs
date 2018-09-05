use std::io;
use std::io::prelude::*;
use std::fs::File;

mod cpu;
use cpu::Cpu;

fn main() {
    println!("yo, chip-8");
    let mut file = File::open("./data/space_invaders.ch8").unwrap();
    let mut program_data = Vec::<u8>::new();
    file.read_to_end(&mut program_data).unwrap();
    for mut b in program_data {
        println!("{:x}", b);
    }

    let cpu = Cpu::new();
}
