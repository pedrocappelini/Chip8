use std::fs::File;
use std::io::Read;

pub struct Chip8 {
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
const START_FONT_ADRESS: u16 = 0x00;
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x70, 0x20, 0x20, 0x70, // 1
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Chip8 {
    //constructor
    pub fn new() -> Self {
        let mut memory = [0u8; 4096];
        memory[START_FONT_ADRESS as usize..START_FONT_ADRESS as usize + FONTSET.len()]
            .copy_from_slice(&FONTSET);
        Self {
            memory,
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

    //loading the ROM into the memory (starting at 0x200).
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
        //Loading the ROM into the memory (todo: check to see if there is enough memory for the ROM).
        self.memory[START_ADRESS as usize..START_ADRESS as usize + buffer.len()]
            .copy_from_slice(&buffer);

        dbg!(&self.memory[0x00..81]);
        dbg!(&self.memory[0x200..0x20A]);
    }
}
