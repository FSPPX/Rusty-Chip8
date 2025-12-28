use sdl2::pixels::PixelFormatEnum; // Quitados Color y Texture innecesarios
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct Platform {
    canvas: Canvas<Window>,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    event_pump: EventPump,
    width: u32,
    height: u32,
}

impl Platform {
    pub fn new(title: &str, window_width: u32, window_height: u32, texture_width: u32, texture_height: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window(title, window_width, window_height)
            .position_centered()
            .resizable()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        
        canvas.set_logical_size(texture_width, texture_height).map_err(|e| e.to_string())?;
        
        let texture_creator = canvas.texture_creator();
        let event_pump = sdl_context.event_pump()?;

        Ok(Platform {
            canvas,
            texture_creator,
            event_pump,
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

        // CORRECCIÓN: map_err para convertir UpdateTextureError a String
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
}

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
