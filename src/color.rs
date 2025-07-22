#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const ORANGE: Color = Color::new(255, 100, 50);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Write color to an RGB buffer at the specified offset
    pub fn write_to_buffer(&self, buffer: &mut [u8], offset: usize) {
        buffer[offset] = self.r;
        buffer[offset + 1] = self.g;
        buffer[offset + 2] = self.b;
    }

    /// Get the RGB components as a tuple
    pub fn as_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Create a color from RGB components in the range 0.0..=1.0
    pub fn from_float(r: f32, g: f32, b: f32) -> Self {
        Color {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
        }
    }
}
