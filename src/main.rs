use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

// Import from our library
use raster_rust::{
    Color, Display,
    display::{draw_pixel, draw_rect},
};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut display = Display::new(&sdl_context)?;

    'mainloop: loop {
        // Handle input - moved back to main
        for event in display.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape) | Some(Keycode::Q),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }

        // Get direct access to screen pixels - NO COLOR BUFFER NEEDED!
        let pixel_buffer = display.get_pixel_buffer()?;

        // Clear screen to black by writing directly to screen memory
        pixel_buffer.fill(0);

        // Draw directly to screen memory - NO COPYING!
        draw_rect(pixel_buffer, 200, 150, 400, 300, Color::ORANGE);

        // Draw some individual pixels to demonstrate pixel-level control
        for i in 0..100 {
            draw_pixel(pixel_buffer, 100 + i, 100, Color::RED);
            draw_pixel(pixel_buffer, 100, 100 + i, Color::GREEN);
            draw_pixel(pixel_buffer, 100 + i, 200, Color::BLUE);
        }

        // Draw a simple diagonal line with pixels
        for i in 0..50 {
            draw_pixel(pixel_buffer, 50 + i, 50 + i, Color::WHITE);
        }

        // Present the surface to screen
        display.present()?;

        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS idle loop
    }

    Ok(())
}
