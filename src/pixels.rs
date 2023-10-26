use std::marker::PhantomData;

use crate::math::{Dimension, Rgba};

/// An immutable view into the pixels of a texture.
#[derive(Debug)]
pub struct Pixels {
    /// The raw buffer containing the pixels.
    buffer: Vec<u8>,

    /// The size of the underlying image.
    size: Dimension,
}

impl Pixels {
    /// Creates a [Pixels] from its raw parts.
    ///
    /// NOTE: The width and height of the texture must be greater than zero.
    #[inline]
    pub(crate) fn new(buffer: Vec<u8>, size: Dimension) -> Self {
        Self { buffer, size }
    }

    /// Returns an iterator over each row of pixels.
    #[inline]
    pub fn rows(&self) -> impl Iterator<Item = &[u8]> {
        self.buffer.chunks(self.size.width as usize)
    }

    /// Collects the pixel data into a [`Vec<u8>`].
    pub fn to_vec(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    /// Collects the pixel data into a [`Vec<u8>`].
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }
}

impl std::ops::Deref for Pixels {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

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

    /// Calculates the index of the provided X and Y coordinates, accounting for the
    /// `buffer_row_width`.  Assumes the provided coordinates are in the buffer's bounds.
    fn index_of(&self, x: u32, y: u32) -> usize {
        self.buffer_row_width * y as usize // calculate the row index
            + x as usize // and the offset from the row, in bytes
            * Rgba::SIZE // then multiply by the size of each pixel, in bytes.
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

    /// Gets the color of a pixel in this buffer.
    pub fn get_color(&self, x: u32, y: u32) -> Option<Rgba> {
        // Ensure the pixel is in bounds
        if x > self.size.width || y > self.size.height {
            return None;
        }

        // We know the pixel is in bounds, so this operation is safe.  `Rgba` has the same
        // in-memory representation as the pixels in this buffer.
        unsafe { Some(*(&self.buffer[self.index_of(x, y)] as *const u8 as *mut Rgba)) }
    }

    /// Returns a mutable reference to the color of the provided pixel, if the coordinates are in
    /// bounds of the pixel buffer.
    pub fn get_color_mut(&mut self, x: u32, y: u32) -> Option<&mut Rgba> {
        // Ensure the pixel is in bounds
        if x > self.size.width || y > self.size.height {
            return None;
        }

        // We know the pixel is in bounds, so this operation is safe.  `Rgba` has the same
        // in-memory representation as the pixels in this buffer.
        unsafe { Some(&mut *(&mut self.buffer[self.index_of(x, y)] as *mut u8 as *mut Rgba)) }
    }

    /// Sets the color of a pixel in the buffer, returning the old color (if it was in bounds).
    pub fn set_color(&mut self, x: u32, y: u32, new_color: Rgba) -> Option<Rgba> {
        let pixel_ref = self.get_color_mut(x, y)?;
        let old_color = *pixel_ref;
        *pixel_ref = new_color;
        Some(old_color)
    }

    /// Collects the pixel data into a [`Vec<u8>`].
    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(Rgba::SIZE * self.size.area() as usize);

        for row in self.rows() {
            vec.extend_from_slice(row);
        }

        vec
    }

    /// Returns an iterator through the bytes in this [PixelsMut] instance.
    ///
    /// NOTE: if you are trying to collect all of these pixels into a [Vec],
    /// [to_vec](#method.to_vec) is most likely faster.
    #[inline]
    pub fn bytes(&self) -> impl Iterator<Item = u8> + '_ {
        self.rows().flat_map(|row| row.iter().copied())
    }

    /// Returns an iterator over the pixels in this [PixelsMut] instance.
    pub fn pixels(&self) -> impl Iterator<Item = Rgba> + '_ {
        struct Iter<'a> {
            pixels: &'a PixelsMut<'a>,
            idx: u32,
        }

        impl<'a> Iterator for Iter<'a> {
            type Item = Rgba;

            fn next(&mut self) -> Option<Self::Item> {
                // Check if the index is in bounds.
                if self.idx >= self.pixels.size.area() {
                    return None;
                }

                let x = self.idx % self.pixels.size.width;
                let y = self.idx / self.pixels.size.width;

                self.idx += 1;

                // We know the pixel is in bounds.
                unsafe {
                    Some(
                        *(&self.pixels.buffer[self.pixels.index_of(x, y)] as *const u8
                            as *mut Rgba),
                    )
                }
            }
        }

        return Iter {
            pixels: self,
            idx: 0,
        };
    }

    /// Returns an iterator over the pixels in this [PixelsMut] instance.
    pub fn pixels_mut(&mut self) -> impl Iterator<Item = &mut Rgba> + '_ {
        struct Iter<'a> {
            marker: PhantomData<&'a mut ()>,
            next_pixel: *mut u8,
            idx: u32,

            /// The width and height of the pixel buffer.
            size: Dimension,

            /// The width of a row in the pixel buffer.
            buffer_row_width: usize,
        }

        impl<'a> Iterator for Iter<'a> {
            type Item = &'a mut Rgba;

            fn next(&mut self) -> Option<Self::Item> {
                // Check if the index is in bounds.
                if self.idx >= self.size.area() {
                    return None;
                }

                let x = self.idx % self.size.width;
                let y = self.idx / self.size.width;

                self.idx += 1;

                // We know the pixel is in bounds. (same algorithm as `index_of`)
                unsafe {
                    Some(
                        &mut *(self
                            .next_pixel
                            .add(self.buffer_row_width * y as usize + x as usize * Rgba::SIZE)
                            as *mut Rgba),
                    )
                }
            }
        }

        return Iter {
            marker: PhantomData,
            next_pixel: self.buffer.as_mut_ptr(),
            idx: 0,
            size: self.size,
            buffer_row_width: self.buffer_row_width,
        };
    }
}
