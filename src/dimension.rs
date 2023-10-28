/// The width and height of an object in integer precision.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
}

impl Dimension {
    /// Creates a new [Dimension] with the provided *width* and *height*.
    #[inline]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Converts this [Dimension] to a [`wgpu::Extent3d`] with `1` depth.
    pub const fn to_extent_3d(&self) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: self.width,
            height: self.height,
            depth_or_array_layers: 1,
        }
    }

    /// Returns the total surface area of this [Dimension].
    ///
    /// Equivalent to `width * height`.
    #[inline]
    pub const fn area(&self) -> u32 {
        self.width * self.height
    }
}
