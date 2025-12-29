use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};

pub struct Platform {
    canvas: Canvas<Window>,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    event_pump: EventPump,
    audio_device: AudioDevice<SquareWave>,
    width: u32,
    height: u32,
}

impl Platform {
    pub fn new(title: &str, window_width: u32, window_height: u32, texture_width: u32, texture_height: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let audio_subsystem = sdl_context.audio()?;

        let window = video_subsystem.window(title, window_width, window_height)
            .position_centered()
            .resizable()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        
        // Logical size allows automatic scaling while maintaining aspect ratio
        canvas.set_logical_size(texture_width, texture_height).map_err(|e| e.to_string())?;
        
        let texture_creator = canvas.texture_creator();
        let event_pump = sdl_context.event_pump()?;

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1), // Mono
            samples: None,
        };

        let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // 440.0 Hz is the frequency for the A4 note
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25, // 25% of max volume
            }
        }).map_err(|e| e.to_string())?;

        Ok(Platform {
            canvas,
            texture_creator,
            event_pump,
            audio_device,
            width: texture_width,
            height: texture_height,
        })
    }

    pub fn update(&mut self, buffer: &[u32], pitch: usize) -> Result<(), String> {
        let mut texture = self.texture_creator.create_texture_streaming(
            PixelFormatEnum::RGBA8888, 
            self.width, 
            self.height
        ).map_err(|e| e.to_string())?;

        let mut raw_buffer = Vec::with_capacity(buffer.len() * 4);
        for &pixel in buffer {
            let bytes = pixel.to_be_bytes();
            raw_buffer.extend_from_slice(&bytes);
        }

        texture.update(None, &raw_buffer, pitch * 4)
            .map_err(|e| e.to_string())?;
        
        self.canvas.clear();
        self.canvas.copy(&texture, None, None)?;
        self.canvas.present();
        Ok(())
    }

    pub fn process_input(&mut self, keys: &mut [bool; 16]) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return true,
                Event::KeyDown { keycode: Some(key), .. } => {
                    if let Some(idx) = key_map(key) { keys[idx] = true; }
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                    if let Some(idx) = key_map(key) { keys[idx] = false; }
                },
                _ => {}
            }
        }
        false
    }

    pub fn play_beep(&self, play: bool) {
        if play {
            self.audio_device.resume();
        } else {
            self.audio_device.pause();
        }
    }
}

// Maps keyboard keys to CHIP-8's 16-key hexadecimal keypad (0-F)
fn key_map(key: Keycode) -> Option<usize> {
    match key {
        Keycode::X => Some(0x0),
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::Z => Some(0xA),
        Keycode::C => Some(0xB),
        Keycode::Num4 => Some(0xC),
        Keycode::R => Some(0xD),
        Keycode::F => Some(0xE),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a simple square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
