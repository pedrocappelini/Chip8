use std::fs::File;
use std::io::Read;

struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: [u16; 16],
    display: [u8; 64 * 32],
    keypad: [u8; 16],
    index: u16,
    s_timer: u8,
    d_timer: u8,
    pc: u16,
    sp: u8,
}

const START_ADRESS: u16 = 0x200;

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0; 4096],
            registers: [0; 16],
            stack: [0; 16],
            display: [0; 64 * 32],
            keypad: [0; 16],
            index: 0,
            s_timer: 0,
            d_timer: 0,
            pc: START_ADRESS,
            sp: 0,
        }
    }

    pub fn load_rom(&mut self, filename: &str) {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("error opening the file. {}", e);
                return;
            }
        };

        let mut buffer = Vec::new();

        match file.read_to_end(&mut buffer) {
            Ok(bytes_read) => {
                println!("size: {}", bytes_read);
            }
            Err(e) => {
                println!("error. {}", e);
                return;
            }
        }
        for i in 0..buffer.len() {
            self.memory[START_ADRESS as usize + i] = buffer[i];
        }
    }
}

fn main() {
    let mut chip_8 = Chip8::new();

    chip_8.load_rom("roms/IBM Logo.ch8");
}
