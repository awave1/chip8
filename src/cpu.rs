pub struct Cpu {
    // index register
    pub i: u16,
    // program counter
    pub pc: u16,
    pub memory: [u8; 4096],
    // data registers
    pub v: [u8; 16],
    pub stack: [u16; 16],
    // stack pointer
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0x200,
            pc: 0x200,
            memory: [0; 4096],
            v: [0; 16],
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn start(self) {
        loop {
            println!("started");
        }
    }
}
