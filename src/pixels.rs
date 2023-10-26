use crate::math::{Dimension, Rgba};

/// A view into the pixels of a texture.
#[derive(Debug)]
pub struct PixelsMut<'a> {
    /// A view into the raw buffer containing the pixels.
    buffer: &'a mut [u8],

    /// The width of a row in the buffer (in BYTES, not pixels), not necessarily the width of an
    /// image.  Generally a multiple of 256.
    buffer_row_width: usize,

    /// The size of the underlying image.
    size: Dimension,
}

impl<'a> PixelsMut<'a> {
    /// Creates a [PixelsMut] from its raw parts.
    ///
    /// NOTE: `buffer_row_width` and the width and height of the texture must be greater than zero.
    #[inline]
    pub(crate) fn new(buffer: &'a mut [u8], buffer_row_width: usize, size: Dimension) -> Self {
        Self {
            buffer,
            buffer_row_width,
            size,
        }
    }

    /// Returns an iterator over each row of pixels.
    pub fn rows(&self) -> impl Iterator<Item = &[u8]> {
        self.buffer
            .chunks(self.buffer_row_width)
            .map(|chunk| &chunk[0..self.size.width as usize * Rgba::SIZE])
    }

    /// Returns a mutable iterator over each row of pixels.
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [u8]> {
        self.buffer
            .chunks_mut(self.buffer_row_width)
            .map(|chunk| &mut chunk[0..self.size.width as usize * Rgba::SIZE])
    }

    /// Sets the color of a pixel in the buffer, returning the old color (if it was in bounds).
    pub fn set_color(&mut self, x: u32, y: u32, color: Rgba) -> Option<Rgba> {
        // Ensure the pixel is in bounds
        if x > self.size.width || y > self.size.height {
            return None;
        }

        // Find the pixel
        let pixel_index = self.buffer_row_width * y as usize // calculate the row index
            + x as usize * Rgba::SIZE; // and the offset from the row, in bytes

        // Unsafe code that is safe, we know the pixel is in bounds, so we can cast the pointer to
        // an Rgba instance, which has the same in-memory representation.
        let pixel = unsafe { &mut *(&mut self.buffer[pixel_index] as *mut u8 as *mut Rgba) };
        let old_color = *pixel;

        // Update the pixel
        *pixel = color;

        Some(old_color)
    }

    /// Collects the pixels into a [`Vec<u8>`].
    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.size.area() as usize);

        for (idx, row) in self.rows().enumerate() {
            vec.extend_from_slice(row);
        }

        vec
    }
}
