use rand::Rng;
use std::fs::File;
use std::io::Read;

pub struct Chip8 {
    memory: [u8; 4096],
    register: [u8; 16],
    stack: [u16; 16],
    pub display: [u8; 64 * 32],
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
            register: [0; 16],
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

        //dbg!(&self.memory[0x200..0x2F0]);
    }

    pub fn cycles(&mut self) {
        //FETCH
        let left_byte = self.memory[self.pc as usize];
        let right_byte = self.memory[(self.pc + 1) as usize];
        let opcode = u16::from_be_bytes([left_byte, right_byte]);
        //println!("{:04X}", opcode);
        self.pc += 2;

        //DECODE
        let n1 = (opcode & 0xF000) >> 12;
        let n2 = (opcode & 0x0F00) >> 8;
        let n3 = (opcode & 0x00F0) >> 4;
        let n4 = opcode & 0x000F;

        //EXECUTIONS
        match (n1, n2, n3, n4) {
            //0x00E0
            (0x0, 0x0, 0xE, 0x0) => {
                self.display.fill(0);
            }

            //0x00EE
            (0x0, _, _, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }

            //0x1nnn
            (0x1, _, _, _) => {
                self.pc = (n2 << 8) | (n3 << 4) | n4;
            }

            //0x2nnn
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp = self.sp + 1;
                self.pc = (n2 << 8) | (n3 << 4) | n4;
            }

            //0x3xkk
            (0x3, _, _, _) => {
                if self.register[n2 as usize] as u16 == (n3 << 4) | n4 {
                    self.pc = self.pc + 2;
                }
            }

            //0x4kk
            (0x4, _, _, _) => {
                if self.register[n2 as usize] as u16 != (n3 << 4) | n4 {
                    self.pc = self.pc + 2;
                }
            }

            //0x5xy0
            (0x5, _, _, 0x0) => {
                if self.register[n2 as usize] == self.register[n3 as usize] {
                    self.pc = self.pc + 2;
                }
            }

            //0x6xkk
            (0x6, _, _, _) => {
                self.register[n2 as usize] = (n3 << 4) as u8 | n4 as u8;
            }

            //0x7xkk
            (0x7, _, _, _) => {
                let res = (n3 << 4) as u8 | n4 as u8;
                self.register[n2 as usize] = self.register[n2 as usize].wrapping_add(res);
            }

            //0x8xy0 (vx = vy)
            (0x8, _, _, 0x0) => {
                self.register[n2 as usize] = self.register[n3 as usize];
            }

            //0x8xy1
            (0x8, _, _, 0x1) => {
                self.register[n2 as usize] =
                    self.register[n2 as usize] | self.register[n3 as usize];
            }

            //0x8xy2
            (0x8, _, _, 0x2) => {
                self.register[n2 as usize] =
                    self.register[n2 as usize] & self.register[n3 as usize];
            }

            //0x8xy3
            (0x8, _, _, 0x3) => {
                self.register[n2 as usize] =
                    self.register[n2 as usize] ^ self.register[n3 as usize];
            }

            //0x8xy4
            (0x8, _, _, 0x4) => {
                let res: u16 =
                    self.register[n2 as usize] as u16 + self.register[n3 as usize] as u16;
                self.register[n2 as usize] = res as u8;
                self.register[0xF] = if res > 255 { 1 } else { 0 };
            }

            //0x8xy5
            (0x8, _, _, 0x5) => {
                self.register[0xF] = if self.register[n2 as usize] >= self.register[n3 as usize] {
                    1
                } else {
                    0
                };
                self.register[n2 as usize] =
                    self.register[n2 as usize].wrapping_sub(self.register[n3 as usize]);
            }

            //0x8xy6
            (0x8, _, _, 0x6) => {
                let lsb = self.register[n2 as usize] & 1;
                self.register[0xF] = lsb;
                self.register[n2 as usize] = self.register[n2 as usize] >> 1;
            }

            //0x8xy7
            (0x8, _, _, 0x7) => {
                self.register[0xF as usize] =
                    if self.register[n3 as usize] >= self.register[n2 as usize] {
                        1
                    } else {
                        0
                    };
                self.register[n2 as usize] =
                    self.register[n3 as usize].wrapping_sub(self.register[n2 as usize]);
            }

            //0x8xyE
            (0x8, _, _, 0xE) => {
                let msb = self.register[n2 as usize] & 128;
                self.register[0xF] = if msb == 128 { 1 } else { 0 };
                self.register[n2 as usize] = self.register[n2 as usize] << 1;
            }

            //0x9xy0
            (0x9, _, _, _) => {
                if self.register[n2 as usize] != self.register[n3 as usize] {
                    self.pc += 2;
                }
            }

            //0xAnnn
            (0xA, _, _, _) => {
                self.index = (n2 << 8) | (n3 << 4) | n4;
            }

            //0xBnnn
            (0xB, _, _, _) => {
                self.pc = ((n2 << 8) | (n3 << 4) | n4) + self.register[0x0] as u16;
            }

            //0xCxkk
            (0xC, _, _, _) => {
                let kk = ((n3 << 4) | n4) as u8;
                let random_byte: u8 = rand::random();
                self.register[n2 as usize] = random_byte & kk;
            }

            //0xDxyn
            (0xD, vx, vy, n) => {
                let x_pos = self.register[vx as usize] as usize % 64;
                let y_pos = self.register[vy as usize] as usize % 32;
                self.register[0xF] = 0; // Reset collision flag

                for row in 0..n {
                    let sprite_byte = self.memory[(self.index + row) as usize];
                    if y_pos + row as usize >= 32 {
                        break;
                    }
                    for col in 0..8 {
                        if x_pos + col >= 64 {
                            break;
                        }
                        let sprite_pixel = (sprite_byte >> (7 - col)) & 1;
                        if sprite_pixel == 1 {
                            let screen_idx = (x_pos + col) + ((y_pos + row as usize) * 64);
                            if self.display[screen_idx] == 1 {
                                self.register[0xF] = 1;
                            }
                            self.display[screen_idx] ^= 1;
                        }
                    }
                }
            }

            //0xEx9E
            (0xE, _, 0x9, _) => {}

            //0xExA1
            (0xE, _, 0xA, _) => {}

            //0xFx07
            (0xF, _, 0x0, 0x7) => {}

            //0xFx0A
            (0xF, _, 0x0, 0xA) => {}

            //0xFx15
            (0xF, _, 0x1, 0x5) => {}

            //0xFx18
            (0xF, _, 0x1, 0x8) => {}

            //0xFx1E
            (0xF, _, 0x1, 0xE) => {}

            //0xFx29
            (0xF, _, 0x2, 0x9) => {}

            //0xFx33
            (0xF, _, 0x3, 0x3) => {}

            //0xFx55
            (0xF, _, 0x5, 0x5) => {}

            //0xFx65
            (0xF, _, 0x6, 0x5) => {}

            //0xFFFF (doesn't exist on the chip8 architecture).
            (0xF, 0xF, 0xF, 0xF) => {
                self.display.fill(1);
            }
            (_, _, _, _) => {
                print!(" . ");
            }
        }
    }
}
