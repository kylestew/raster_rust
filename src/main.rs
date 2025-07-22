use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

// Import from our library
use raster_rust::{
    Color, Display,
    display::{HEIGHT, PITCH, draw_rect},
};

fn update_color_buffer(buffer: &mut [u8]) {
    // Clear buffer to black
    buffer.fill(0);

    draw_rect(buffer, 200, 150, 400, 300, Color::ORANGE);
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut display = Display::new(&sdl_context)?;

    // Create color buffer - this is now managed by main
    let mut color_buffer = vec![0u8; PITCH * HEIGHT];

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

        // Update what to draw - application logic in main
        update_color_buffer(&mut color_buffer);

        // Render the buffer - display module just handles rendering
        display.render(&color_buffer)?;

        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS idle loop
    }

    Ok(())
}
