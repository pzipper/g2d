/// Represents the width and height of an object in integer precision.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
}

impl Dimension {
    /// Creates a new [Dimension] with the provided width and height.
    #[inline]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Returns the surface area of the [Dimension].
    ///
    /// Equivalent to `width * height`.
    #[inline]
    pub const fn area(&self) -> u32 {
        self.width * self.height
    }
}
