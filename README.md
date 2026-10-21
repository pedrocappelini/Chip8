# SCHIP Emulator

An emulator originally built for CHIP-8 and now upgraded to support Super-CHIP (SCHIP) specifications. The implementation handles standard ROMs and high-resolution SCHIP titles, with a few edge-case quirks currently being refined.

---

## Technical Stack

- **Language:** Rust (chosen for strict memory safety, performance, and predictable resource management)
- **Graphics & Windowing:** Macroquad (used for handling the display pipeline, screen scaling, and input polling)

---

## Compatibility and Features

- Upgraded from base CHIP-8 to SCHIP, supporting extended instruction sets and the 128x64 high-resolution display mode.
- Functional timing, keypad input mapping, and sound/delay timers.
- Broad compatibility with most standard test suites and games.

---

## Bundled ROMs & Where to Get More

A selection of sample ROMs is already included in this repo. All of them came from the CHIP-8 Archive. For more, visit:

- https://johnearnest.github.io/chip8Archive/?sort=platform

---

## How to Change the ROM

Currently, the ROM path is set directly in the source code. To change the active game, open `main.rs` and update the ROM path inside the load function:

```rust
chip8.load_rom("roms/ROM_NAME.ch8");
```

Replace `ROM_NAME.ch8` with the filename of the ROM you want to run.

---

## Getting Started

### Prerequisites

- Rust and Cargo installed via rustup.

### Running the Emulator

Clone the repository and run:

```bash
cargo run --release
```

---

## Roadmap

- **File Organization & Optimization:** Refactor project structure for cleaner separation of concerns, improve code layout, and apply performance optimizations across the emulation loop.
- **Color Palette Support:** Implement custom foreground/background colors and multi-color pixel rendering modes (such as MegaChip/XO-CHIP style extensions).
- **Built-in Assembler:** Develop an integrated assembler tool to write, compile, and test custom ROMs directly within the project workflow.
- **Edge-case Accuracy:** Fix remaining instruction timing and flag-handling quirks in specific edge cases.
