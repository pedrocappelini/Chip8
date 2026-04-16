mod chip8;

use chip8::Chip8;

fn main() {
    let mut chip_8 = Chip8::new();

    chip_8.load_rom("roms/IBM Logo.ch8");
}
