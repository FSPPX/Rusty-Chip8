# CHIP-8 Emulator

A CHIP-8 interpreter/emulator implemented in Rust with SDL2 for graphics and input handling.

## About CHIP-8

CHIP-8 is an interpreted programming language developed in the mid-1970s. It was initially used on 8-bit microcomputers to make game programming easier. CHIP-8 programs are run on a virtual machine with its own instruction set, memory, registers, and display.

## Features

- Full CHIP-8 instruction set implementation
- 64x32 monochrome display
- SDL2-based rendering and input handling
- Keyboard input mapping
- Sound timer support

## Building and Running

```bash
# Build the project
cargo build --release

# Run with a ROM file
cargo run --release <path-to-rom-file>
```

## Controls

CHIP-8 uses a 16-key hexadecimal keypad (0-F). The keyboard mapping is:

```
CHIP-8 Keypad:          Keyboard Mapping:
+---+---+---+---+       +---+---+---+---+
| 1 | 2 | 3 | C |       | 1 | 2 | 3 | 4 |
+---+---+---+---+       +---+---+---+---+
| 4 | 5 | 6 | D |       | Q | W | E | R |
+---+---+---+---+       +---+---+---+---+
| 7 | 8 | 9 | E |       | A | S | D | F |
+---+---+---+---+       +---+---+---+---+
| A | 0 | B | F |       | Z | X | C | V |
+---+---+---+---+       +---+---+---+---+
```

## Project Structure

- `src/chip8.rs` - Core CHIP-8 CPU and instruction implementation
- `src/platform.rs` - SDL2 platform/graphics layer
- `src/main.rs` - Entry point and main loop

## Resources and Credits

This project was inspired by Austin Morlan's excellent CHIP-8 emulator tutorial in C++:
- [Building a CHIP-8 Emulator](https://austinmorlan.com/posts/chip8_emulator/#source-code)

Additional resources:
- [CHIP-8 Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
