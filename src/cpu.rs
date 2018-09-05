use std::io::{Error, ErrorKind};

const FONTSET: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Cpu {
    // index register
    i: u16,
    // program counter
    pc: usize,
    memory: [u8; 4096],
    // data registers
    v: [u8; 16],
    stack: [u16; 16],
    // stack pointer
    sp: u8,
    delay_timer: u8,
    sound_timer: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            i: 0x200,
            pc: 0x200,
            memory: [0; 4096],
            v: [0; 16],
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        };

        // Load fonts in first 512 bytes
        cpu.load_fonts();

        cpu
    }

    pub fn load_prog(&mut self, data: Vec<u8>) -> Result<bool, Error> {
        let mut byte_count = 0;
        for (i, byte) in data.iter().enumerate() {
            self.memory[self.pc] = *byte;
            self.pc += 1;
            byte_count = i;
        }

        // Make sure all data have been loaded successfully
        let loaded = (self.pc - (byte_count + 1)) == 0x200;

        if loaded {
            Ok(loaded)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Failed to load the program"))
        }
    }

    pub fn start(self) {
        println!("started");
    }

    fn load_fonts(&mut self) {
        for (i, &byte) in FONTSET.iter().enumerate() {
            self.memory[i] = byte;
        }
    }
}
