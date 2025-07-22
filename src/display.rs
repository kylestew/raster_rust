use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;

use crate::Color;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;
pub const PITCH: usize = WIDTH * 3; // RGB24: 3 bytes per pixel

pub struct Display {
    canvas: Canvas<Window>,
    surface: Surface<'static>,
    pub event_pump: sdl2::EventPump,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String> {
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("rust-sdl2 demo: Video", WIDTH as u32, HEIGHT as u32)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())?;

        // Create a surface that matches our screen format for direct pixel access
        let surface = Surface::new(WIDTH as u32, HEIGHT as u32, PixelFormatEnum::RGB24)
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;

        Ok(Display {
            canvas,
            surface,
            event_pump,
        })
    }

    pub fn get_pixel_buffer(&mut self) -> Result<&mut [u8], String> {
        // Get direct access to surface pixels - NO TEXTURE NEEDED!
        self.surface
            .without_lock_mut()
            .ok_or_else(|| "Surface requires locking for pixel access".to_string())
    }

    pub fn present(&mut self) -> Result<(), String> {
        // Convert surface to texture only once per frame, then display
        let texture_creator = self.canvas.texture_creator();
        let texture = self
            .surface
            .as_texture(&texture_creator)
            .map_err(|e| format!("Failed to create texture from surface: {:?}", e))?;

        self.canvas.copy(&texture, None, None)?;
        self.canvas.present();
        Ok(())
    }
}

/// Draw a single pixel at the specified coordinates
pub fn draw_pixel(buffer: &mut [u8], x: i32, y: i32, color: Color) {
    // Bounds check - ensure we don't write outside the buffer
    if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
        let offset = (y as usize) * PITCH + (x as usize) * 3;
        color.write_to_buffer(buffer, offset);
    }
}

pub fn draw_rect(buffer: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: Color) {
    for curr_y in y..y + height {
        for curr_x in x..x + width {
            draw_pixel(buffer, curr_x, curr_y, color);
        }
    }
}
