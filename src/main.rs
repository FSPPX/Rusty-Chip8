mod chip8;
mod platform;

use chip8::Chip8;
use platform::Platform;
use clap::Parser;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Escala del video
    #[arg(short, long, default_value_t = 10)]
    scale: u32,

    /// Retardo por ciclo en milisegundos
    #[arg(short, long, default_value_t = 1)]
    delay: u64,

    /// Ruta al archivo ROM
    #[arg(index = 1)]
    rom_path: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let window_width = (chip8::VIDEO_WIDTH as u32) * args.scale;
    let window_height = (chip8::VIDEO_HEIGHT as u32) * args.scale;

    let mut platform = Platform::new(
        "CHIP-8 Emulator (Rust)", 
        window_width, 
        window_height, 
        chip8::VIDEO_WIDTH as u32, 
        chip8::VIDEO_HEIGHT as u32
    )?;

    let mut chip8 = Chip8::new();
    chip8.load_rom(&args.rom_path);

    let mut last_cycle_time = Instant::now();
    let cycle_delay = Duration::from_millis(args.delay);
    let video_pitch = chip8::VIDEO_WIDTH;

    'running: loop {
        let quit = platform.process_input(&mut chip8.keypad);
        if quit {
            break 'running;
        }

        let current_time = Instant::now();
        if current_time.duration_since(last_cycle_time) > cycle_delay {
            last_cycle_time = current_time;
            
            chip8.cycle();
            platform.update(&chip8.video, video_pitch)?;
        }
    }

    Ok(())
}
