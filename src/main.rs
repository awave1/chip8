use std::io;
use std::io::prelude::*;
use std::fs::File;

mod macros;

mod cpu;
use cpu::Cpu;


fn get_program_data(path: &str) -> Result<Vec<u8>, io::Error> {
    debug!("> reading program data");
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut program_data = Vec::<u8>::new();
    match f.read_to_end(&mut program_data) {
        Ok(_) => Ok(program_data),
        Err(e) => Err(e),
    }
}

fn start_with_data(data: Vec<u8>) {
    println!("yo, chip-8");
    debug!("> starting");

    let mut cpu = Cpu::new();

    match cpu.load_prog(data) {
        Ok(_) => cpu.start(),
        Err(e) => panic!("{:?}", e)
    }
}

fn main() {
    match get_program_data("./data/space_invaders.ch8") {
        Ok(data) => start_with_data(data),
        Err(e) => panic!("Oops, error opening the file {:?}", e),
    }

    debug!("> stop");
    println!("bye!");
}
