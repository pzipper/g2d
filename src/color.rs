/// Represents an RGBA color.
#[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl Color {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    /// Creates a new [Color] from the provided red, green, blue and alpha channels.
    #[inline]
    pub const fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Converts a [Color] to its associated RGBA bytes.
    #[inline]
    pub fn to_rgba_bytes(&self) -> [u8; 4] {
        [
            (self.red.min(1.0) * 255.0) as u8,
            (self.green.min(1.0) * 255.0) as u8,
            (self.blue.min(1.0) * 255.0) as u8,
            (self.alpha.min(1.0) * 255.0) as u8,
        ]
    }

    /// Converts this [Color] to a [`wgpu::Color`].
    #[inline]
    pub const fn to_wgpu_color(&self) -> wgpu::Color {
        wgpu::Color {
            r: self.red,
            g: self.green,
            b: self.blue,
            a: self.alpha,
        }
    }
}
