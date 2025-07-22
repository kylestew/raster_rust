use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::Color;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;
pub const PITCH: usize = WIDTH * 3; // RGB24: 3 bytes per pixel

pub struct Display {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    pub event_pump: sdl2::EventPump,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String> {
        let video_subsystem = sdl_context.video()?;
        let (canvas, texture_creator) = create_canvas(&video_subsystem)?;

        let event_pump = sdl_context.event_pump()?;

        Ok(Display {
            canvas,
            texture_creator,
            event_pump,
        })
    }

    pub fn render(&mut self, color_buffer: &[u8]) -> Result<(), String> {
        let mut texture = self
            .texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32)
            .map_err(|e| e.to_string())?;

        // Copy buffer into texture
        texture
            .update(None, color_buffer, PITCH)
            .map_err(|e| e.to_string())?;

        // Display
        self.canvas.copy(&texture, None, None)?;
        self.canvas.present();

        Ok(())
    }
}

fn create_canvas(
    video_subsystem: &sdl2::VideoSubsystem,
) -> Result<(Canvas<Window>, TextureCreator<WindowContext>), String> {
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
    let texture_creator = canvas.texture_creator();

    Ok((canvas, texture_creator))
}

pub fn draw_rect(buffer: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: Color) {
    for curr_y in y..y + height {
        for curr_x in x..x + width {
            if curr_y >= 0 && curr_y < HEIGHT as i32 && curr_x >= 0 && curr_x < WIDTH as i32 {
                let offset = (curr_y as usize) * PITCH + (curr_x as usize) * 3;
                color.write_to_buffer(buffer, offset);
            }
        }
    }
}
