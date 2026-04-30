mod chip8;

use chip8::Chip8;
use macroquad::prelude::*;

#[macroquad::main(window_config)]
async fn main() {
    let mut chip8 = Chip8::new();

    chip8.load_rom("roms/Space Invaders [David Winter].ch8");
    display_render(chip8).await;
}

async fn display_render(mut chip8: Chip8) {
    loop {
        key_listener(&mut chip8);

        for _ in 0..10 {
            chip8.cycles();
        }

        if chip8.d_timer > 0 {
            chip8.d_timer -= 1;
        }

        if chip8.s_timer > 0 {
            chip8.s_timer -= 1;
        }

        clear_background(BLACK);

        let sq_w = screen_width() / 64.0;
        let sq_h = screen_height() / 32.0;

        for y in 0..32 {
            for x in 0..64 {
                if chip8.display[y * 64 + x] == 1 {
                    draw_rectangle(x as f32 * sq_w, y as f32 * sq_h, sq_w, sq_h, WHITE);
                }
            }
        }

        next_frame().await
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: "Chip-8".to_string(),
        window_width: 1280,
        window_height: 640,
        ..Default::default()
    }
}

fn key_listener(chip8: &mut Chip8) {
    chip8.keypad.fill(0);

    // 1 2 3 4
    if is_key_down(KeyCode::Key1) {
        chip8.keypad[0x1] = 1;
    }
    if is_key_down(KeyCode::Key2) {
        chip8.keypad[0x2] = 1;
    }
    if is_key_down(KeyCode::Key3) {
        chip8.keypad[0x3] = 1;
    }
    if is_key_down(KeyCode::Key4) {
        chip8.keypad[0xC] = 1;
    }
    // Q W E R
    if is_key_down(KeyCode::Q) {
        chip8.keypad[0x4] = 1;
    }
    if is_key_down(KeyCode::W) {
        chip8.keypad[0x5] = 1;
    }
    if is_key_down(KeyCode::E) {
        chip8.keypad[0x6] = 1;
    }
    if is_key_down(KeyCode::R) {
        chip8.keypad[0xD] = 1;
    }
    // A S D F
    if is_key_down(KeyCode::A) {
        chip8.keypad[0x7] = 1;
    }
    if is_key_down(KeyCode::S) {
        chip8.keypad[0x8] = 1;
    }
    if is_key_down(KeyCode::D) {
        chip8.keypad[0x9] = 1;
    }
    if is_key_down(KeyCode::F) {
        chip8.keypad[0xE] = 1;
    }
    // Z X C V
    if is_key_down(KeyCode::Z) {
        chip8.keypad[0xA] = 1;
    }
    if is_key_down(KeyCode::X) {
        chip8.keypad[0x0] = 1;
    }
    if is_key_down(KeyCode::C) {
        chip8.keypad[0xB] = 1;
    }
    if is_key_down(KeyCode::V) {
        chip8.keypad[0xF] = 1;
    }
}
