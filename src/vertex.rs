use crate::{Color, Vec2};

/// An individual piece of vertex data.
#[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vertex {
    /// The on-screen position of the vertex.
    ///
    /// Note that the transform matrix will be applied to this value.
    pub position: Vec2,

    /// The texture coordinates to sample for this vertex.
    ///
    /// Both *x* and *y* should have a minimum value of `0.0` and a maximum value of `1.0`.
    pub uv: Vec2,

    /// The color of the vertex.
    ///
    /// Ignored if the
    pub color: Color,
}

impl Vertex {
    /// Creates a [Vertex] with the provided *position*, *uv* and *color*.
    #[inline]
    pub fn new(position: Vec2, uv: Vec2, color: Color) -> Self {
        Self {
            position,
            uv,
            color,
        }
    }
}
