use rand::prelude::*;

pub const KEY_COUNT: usize = 16;
pub const MEMORY_SIZE: usize = 4096;
pub const REGISTER_COUNT: usize = 16;
pub const STACK_LEVELS: usize = 16;
pub const VIDEO_HEIGHT: usize = 32;
pub const VIDEO_WIDTH: usize = 64;

type Chip8Fn = fn(&mut Chip8);

pub struct Chip8 {
    pub keypad: [u8; KEY_COUNT],
    pub video: [u32; VIDEO_WIDTH * VIDEO_HEIGHT],

    memory: [u8; MEMORY_SIZE],
    registers: [u8; REGISTER_COUNT],
    index: u16,
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; STACK_LEVELS],
    sp: u8,
    opcode: u16,

    rng: StdRng,
    rand_dist: Uniform<u8>,

    table: [Chip8Fn; 0xF + 1],
    table0: [Chip8Fn; 0xE + 1],
    table8: [Chip8Fn; 0xE + 1],
    tableE: [Chip8Fn; 0xE + 1],
    tableF: [Chip8Fn; 0x65 + 1],
}

const FONTSET_SIZE: usize = 80;
const FONTSET_START_ADDRESS: usize = 0x50;
const START_ADDRESS: usize = 0x200;

const FONTSET: [u8; FONTSET_SIZE] = [
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

impl Chip8 {
    pub fn new() -> Self {
        let mut seed = [0u8; 32];
        rand::rng().fill_bytes(&mut seed); // Generate a random seed
        Self {
            pc: START_ADDRESS as u16,

            memory: {
                let mut memory = [0; MEMORY_SIZE];
                memory[FONTSET_START_ADDRESS..FONTSET_START_ADDRESS + FONTSET_SIZE]
                    .copy_from_slice(&FONTSET);
                memory
            },

            rng: StdRng::from_seed(seed),
            rand_dist: Uniform::new_inclusive(0, 255),
        }
    }

    pub fn load_rom(&mut self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::{self, Read};

        let mut file = File::open(filename)?;
        let metadata = file.metadata()?;
        let file_size = metadata.len() as usize;

        if file_size > MEMORY_SIZE - START_ADDRESS as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ROM is too large to fit in memory",
            ));
        }

        let mut buffer = vec![0; file_size];
        file.read_exact(&mut buffer)?;

        self.memory[START_ADDRESS as usize..START_ADDRESS as usize + file_size]
            .copy_from_slice(&buffer);

        Ok(())
    }

    pub fn cycle(&mut self) {
        // implementación pendiente
    }

    // Tables
    fn table0(&mut self) {}
    fn table8(&mut self) {}
    fn tableE(&mut self) {}
    fn tableF(&mut self) {}

    // Opcodes (nombres adaptados a snake_case)
    fn op_null(&mut self) {}

    fn op_00e0(&mut self) {} // CLS
    fn op_00ee(&mut self) {} // RET
    fn op_1nnn(&mut self) {} // JP address
    fn op_2nnn(&mut self) {} // CALL address
    fn op_3xkk(&mut self) {} // SE Vx, byte
    fn op_4xkk(&mut self) {} // SNE Vx, byte
    fn op_5xy0(&mut self) {} // SE Vx, Vy
    fn op_6xkk(&mut self) {} // LD Vx, byte
    fn op_7xkk(&mut self) {} // ADD Vx, byte
    fn op_8xy0(&mut self) {} // LD Vx, Vy
    fn op_8xy1(&mut self) {} // OR Vx, Vy
    fn op_8xy2(&mut self) {} // AND Vx, Vy
    fn op_8xy3(&mut self) {} // XOR Vx, Vy
    fn op_8xy4(&mut self) {} // ADD Vx, Vy
    fn op_8xy5(&mut self) {} // SUB Vx, Vy
    fn op_8xy6(&mut self) {} // SHR Vx
    fn op_8xy7(&mut self) {} // SUBN Vx, Vy
    fn op_8xye(&mut self) {} // SHL Vx
    fn op_9xy0(&mut self) {} // SNE Vx, Vy
    fn op_annn(&mut self) {} // LD I, address
    fn op_bnnn(&mut self) {} // JP V0, address
    fn op_cxkk(&mut self) {} // RND Vx, byte
    fn op_dxyn(&mut self) {} // DRW Vx, Vy, height
    fn op_ex9e(&mut self) {} // SKP Vx
    fn op_exa1(&mut self) {} // SKNP Vx
    fn op_fx07(&mut self) {} // LD Vx, DT
    fn op_fx0a(&mut self) {} // LD Vx, K
    fn op_fx15(&mut self) {} // LD DT, Vx
    fn op_fx18(&mut self) {} // LD ST, Vx
    fn op_fx1e(&mut self) {} // ADD I, Vx
    fn op_fx29(&mut self) {} // LD F, Vx
    fn op_fx33(&mut self) {} // LD B, Vx
    fn op_fx55(&mut self) {} // LD [I], Vx
    fn op_fx65(&mut self) {} // LD Vx, [I]
}