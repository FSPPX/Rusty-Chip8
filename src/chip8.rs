use std::fs::File;
use std::io::Read;

pub const VIDEO_WIDTH: usize = 64;
pub const VIDEO_HEIGHT: usize = 32;
const MEMORY_SIZE: usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_LEVELS: usize = 16;
const KEY_COUNT: usize = 16;
const START_ADDRESS: usize = 0x200;
const FONTSET_START_ADDRESS: usize = 0x50;

const FONTSET: [u8; 80] = [
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

pub struct Chip8 {
    registers: [u8; REGISTER_COUNT],
    memory: [u8; MEMORY_SIZE],
    index: u16,
    pc: u16,
    stack: [u16; STACK_LEVELS],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    pub keypad: [bool; KEY_COUNT],
    pub video: [u32; VIDEO_WIDTH * VIDEO_HEIGHT],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            registers: [0; REGISTER_COUNT],
            memory: [0; MEMORY_SIZE],
            index: 0,
            pc: START_ADDRESS as u16,
            stack: [0; STACK_LEVELS],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; KEY_COUNT],
            video: [0; VIDEO_WIDTH * VIDEO_HEIGHT],
        };

        // Cargar fuente en memoria
        chip8.memory[FONTSET_START_ADDRESS..FONTSET_START_ADDRESS + 80]
            .copy_from_slice(&FONTSET);

        chip8
    }

    pub fn load_rom(&mut self, filename: &str) {
        let mut file = File::open(filename).expect("No se pudo abrir el archivo ROM");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Error leyendo ROM");

        for (i, &byte) in buffer.iter().enumerate() {
            if START_ADDRESS + i < MEMORY_SIZE {
                self.memory[START_ADDRESS + i] = byte;
            }
        }
    }

    pub fn cycle(&mut self) {
        // Fetch
        let opcode = (self.memory[self.pc as usize] as u16) << 8 
                   | (self.memory[self.pc as usize + 1] as u16);

        self.pc += 2;

        // Decode & Execute
        self.execute_opcode(opcode);

        // Timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn execute_opcode(&mut self, opcode: u16) {
        let digit1 = (opcode & 0xF000) >> 12;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match (digit1, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => self.video = [0; VIDEO_WIDTH * VIDEO_HEIGHT], // CLS
            (0x0, 0x0, 0xE, 0xE) => { // RET
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            },
            (0x1, _, _, _) => self.pc = nnn, // JP addr
            (0x2, _, _, _) => { // CALL addr
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            },
            (0x3, _, _, _) => if self.registers[x] == nn { self.pc += 2 }, // SE Vx, byte
            (0x4, _, _, _) => if self.registers[x] != nn { self.pc += 2 }, // SNE Vx, byte
            (0x5, _, _, 0x0) => if self.registers[x] == self.registers[y] { self.pc += 2 }, // SE Vx, Vy
            (0x6, _, _, _) => self.registers[x] = nn, // LD Vx, byte
            (0x7, _, _, _) => self.registers[x] = self.registers[x].wrapping_add(nn), // ADD Vx, byte
            (0x8, _, _, 0x0) => self.registers[x] = self.registers[y], // LD Vx, Vy
            (0x8, _, _, 0x1) => self.registers[x] |= self.registers[y], // OR Vx, Vy
            (0x8, _, _, 0x2) => self.registers[x] &= self.registers[y], // AND Vx, Vy
            (0x8, _, _, 0x3) => self.registers[x] ^= self.registers[y], // XOR Vx, Vy
            (0x8, _, _, 0x4) => { // ADD Vx, Vy
                let (res, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[0xF] = if overflow { 1 } else { 0 };
                self.registers[x] = res;
            },
            (0x8, _, _, 0x5) => { // SUB Vx, Vy
                self.registers[0xF] = if self.registers[x] > self.registers[y] { 1 } else { 0 };
                self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]);
            },
            (0x8, _, _, 0x6) => { // SHR Vx
                self.registers[0xF] = self.registers[x] & 0x1;
                self.registers[x] >>= 1;
            },
            (0x8, _, _, 0x7) => { // SUBN Vx, Vy
                self.registers[0xF] = if self.registers[y] > self.registers[x] { 1 } else { 0 };
                self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);
            },
            (0x8, _, _, 0xE) => { // SHL Vx
                self.registers[0xF] = (self.registers[x] & 0x80) >> 7;
                self.registers[x] <<= 1;
            },
            (0x9, _, _, 0x0) => if self.registers[x] != self.registers[y] { self.pc += 2 }, // SNE Vx, Vy
            (0xA, _, _, _) => self.index = nnn, // LD I, addr
            (0xB, _, _, _) => self.pc = (self.registers[0] as u16).wrapping_add(nnn), // JP V0, addr
            (0xC, _, _, _) => { // RND Vx, byte
                let rng: u8 = rand::random(); 
                self.registers[x] = rng & nn;
            },
            (0xD, _, _, _) => { // DRW Vx, Vy, nibble
                let x_pos = (self.registers[x] as usize) % VIDEO_WIDTH;
                let y_pos = (self.registers[y] as usize) % VIDEO_HEIGHT;
                let height = n as usize;
                
                self.registers[0xF] = 0;

                for row in 0..height {
                    let sprite_byte = self.memory[self.index as usize + row];
                    for col in 0..8 {
                        let sprite_pixel = sprite_byte & (0x80 >> col);
                        if sprite_pixel != 0 {
                            let idx = (y_pos + row) * VIDEO_WIDTH + (x_pos + col);
                            if idx < self.video.len() {
                                if self.video[idx] == 0xFFFFFFFF {
                                    self.registers[0xF] = 1;
                                }
                                self.video[idx] ^= 0xFFFFFFFF;
                            }
                        }
                    }
                }
            },
            (0xE, _, _, 0xE) => if self.keypad[self.registers[x] as usize] { self.pc += 2 }, // SKP Vx
            (0xE, _, _, 0x1) => if !self.keypad[self.registers[x] as usize] { self.pc += 2 }, // SKNP Vx
            
            (0xF, _, _, 0x7) => self.registers[x] = self.delay_timer,
            
            // Fx0A es único
            (0xF, _, _, 0xA) => { // LD Vx, K
                let mut pressed = false;
                for i in 0..KEY_COUNT {
                    if self.keypad[i] {
                        self.registers[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.pc -= 2;
                }
            },

            // Fx15 (LD DT, Vx) - termina en 5, pero el segundo nibble es 1
            (0xF, _, 0x1, 0x5) => self.delay_timer = self.registers[x],
            
            // Fx18 (LD ST, Vx) - termina en 8
            (0xF, _, _, 0x8) => self.sound_timer = self.registers[x],
            
            // Fx1E (ADD I, Vx) - termina en E
            (0xF, _, _, 0xE) => self.index = self.index.wrapping_add(self.registers[x] as u16),
            
            // Fx29 (LD F, Vx) - termina en 9
            (0xF, _, _, 0x9) => self.index = FONTSET_START_ADDRESS as u16 + (self.registers[x] as u16 * 5),
            
            // Fx33 (LD B, Vx) - termina en 3
            (0xF, _, _, 0x3) => { 
                let mut value = self.registers[x];
                self.memory[self.index as usize + 2] = value % 10;
                value /= 10;
                self.memory[self.index as usize + 1] = value % 10;
                value /= 10;
                self.memory[self.index as usize] = value;
            },
            
            // Fx55 (LD [I], Vx) - termina en 5, segundo nibble es 5
            (0xF, _, 0x5, 0x5) => { 
                 for i in 0..=x {
                     self.memory[self.index as usize + i] = self.registers[i];
                 }
            },
            
            // Fx65 (LD Vx, [I]) - termina en 5, segundo nibble es 6
            (0xF, _, 0x6, 0x5) => { 
                 for i in 0..=x {
                     self.registers[i] = self.memory[self.index as usize + i];
                 }
            },

            _ => println!("Opcode desconocido: {:X}", opcode),
        }
    }
}
