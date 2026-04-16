mod chip8;

use chip8::Chip8;
use macroquad::prelude::*;

#[macroquad::main(window_config)]
async fn main() {
    let mut chip8 = Chip8::new();

    chip8.load_rom("roms/IBM Logo.ch8");
    display_render(chip8).await;
}

async fn display_render(mut chip8: Chip8) {
    loop {
        chip8.cycles();

        clear_background(BLACK);

        dbg!(&chip8.display.len());

        for y in 0..32 {
            for x in 0..64 {
                draw_rectangle(
                    (x * 10) as f32,
                    (y * 10) as f32,
                    10.0,
                    10.0,
                    if chip8.display[y * 64 + x] == 1 {
                        WHITE
                    } else {
                        BLACK
                    },
                );
            }
        }

        next_frame().await
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: "Chip-8".to_string(),
        window_width: 640,
        window_height: 320,
        ..Default::default()
    }
}
