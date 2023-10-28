/// Represents an RGBA color.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);

    /// Creates a new [Color] from the provided red, green, blue and alpha channels.
    #[inline]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Converts the red, green, blue and alpha channels of this [Color] to [f64]s.
    #[inline]
    pub fn to_floats(&self) -> (f64, f64, f64, f64) {
        (
            self.red as f64 / 255f64,
            self.green as f64 / 255f64,
            self.blue as f64 / 255f64,
            self.alpha as f64 / 255f64,
        )
    }

    /// Converts this [Color] to a [`wgpu::Color`].
    #[inline]
    pub fn to_wgpu_color(&self) -> wgpu::Color {
        let floats = self.to_floats();

        wgpu::Color {
            r: floats.0,
            g: floats.1,
            b: floats.2,
            a: floats.3,
        }
    }
}
