/// A 2D value with direction and magnitude.
#[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Creates a new [Vec2] from the provided *x* and *y* values.
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
