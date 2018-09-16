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
    sp: usize,
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

        for (i, &byte) in data.iter().enumerate() {
            if self.memory[self.pc] == 0 {
                self.memory[self.pc] = byte;
                self.pc += 1;
                byte_count = i;
            } else {
                break;
            }
        }

        // Make sure all data have been loaded successfully
        let loaded = (self.pc - (byte_count + 1)) == 0x200;
        self.pc = 0x200; // reset pc back to start (0x200 in chip-8 case)

        if loaded {
            Ok(loaded)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Failed to load the program"))
        }
    }

    // fetches 16bit word opcode
    fn fetch(&mut self) -> u16 {
        let opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);

        opcode
    }

    fn execute(&mut self, opcode: u16) {
        let x = ((opcode >> 8) & 0x000f) as usize;
        let y = ((opcode >> 4) & 0x000f) as usize;
        let n = opcode & 0x000F; // the lowest 4 bits
        let nn = opcode & 0x00FF; // the lowest 8 bits
        let nnn = opcode & 0x0FFF; // the lowest 12 bits


        match opcode & 0xf000 {
            0x0000 => {
                match nn {
                    0x00e0 => debug!("0x00e0"),
                    0x00ee => debug!("0x00ee"),
                    _ => debug!("unknown opcode"),
                }
            },
            0x1000 => {
                debug!("0x1000");
                self.pc = nnn as usize;
            },
            0x2000 => {
                // calls subroutine at nnn
                debug!("0x2000");
                self.sp += 1;
                self.stack[self.sp] = (self.pc + 2) as u16;
                self.pc = nnn as usize;
            },
            0x3000 => {
                // conditional vx == nn
                if self.v[x] == nn as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x4000 => {
                // conditional vx != nn
                if self.v[x] != nn as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x5000 => {
                // conditional vx == vy
                if self.v[x] == self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6000 => {
                self.v[x] = nn as u8;
                self.pc += 2;
            },
            0x7000 => {
                self.v[x] += nn as u8;
                self.pc += 2;
            }
            0x8000 => debug!("0x8000"),
            0x9000 => debug!("0x9000"),
            0xa000 => debug!("0xa000"),
            0xb000 => debug!("0xb000"),
            0xc000 => debug!("0xc000"),
            0xd000 => debug!("0xd000"),
            0xe000 => debug!("0xe000"),
            0xd000 => debug!("0xd000"),
            _ => debug!(),
        }
    }

    pub fn start(mut self) {
        loop {
            let opcode = self.fetch();
            self.execute(opcode);
            //debug!("{:x}", opcode);
        };
    }

    fn load_fonts(&mut self) {
        for (i, &byte) in FONTSET.iter().enumerate() {
            self.memory[i] = byte;
        }
    }
}
