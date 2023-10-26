use std::fmt::Debug;

/// A color with 4 channels: red, green, blue and alpha.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Rgba {
    /// The size of an [Rgba] in bytes.
    ///
    /// ```
    /// # use g2d::math::Rgba;
    /// // Sanity test:
    /// assert_eq!(std::mem::size_of::<Rgba>(), Rgba::SIZE);
    /// assert_eq!(Rgba::SIZE, 4);
    /// ```
    pub const SIZE: usize = 4;

    pub const BLACK: Rgba = Rgba::new(0, 0, 0, 255);
    pub const WHITE: Rgba = Rgba::new(255, 255, 255, 255);

    /// Creates a new [Rgba] color.
    #[inline]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Creates an [Rgba] from the provided slice.
    pub const fn from_slice(slice: &[u8]) -> Option<Self> {
        if slice.len() < 4 {
            return None;
        }

        // Efficiently read the bytes from memory
        unsafe { Some(*(slice as *const [u8] as *const Rgba)) }
    }

    /// Creates a slice of this [Rgba]'s bytes.
    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, 4) }
    }

    /// Creates a mutable slice of this [Rgba]'s bytes.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self as *mut Self as *mut u8, 4) }
    }
}

impl Debug for Rgba {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rgba({:02}, {:02}, {:02}, {:02})",
            self.red, self.green, self.blue, self.alpha
        )
    }
}
