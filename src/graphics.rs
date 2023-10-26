use wgpu::Extent3d;

use crate::{
    math::{Dimension, Rgba},
    Context, Pixels, PixelsMut, Texture,
};

/// The main struct for Graphics in G2d.
#[derive(Debug)]
pub struct Graphics<'a> {
    texture: &'a Texture<'a>,
}

impl<'a> Graphics<'a> {
    /// Creates a [Graphics] instance for the provided [Texture].
    #[inline]
    pub fn from_raw_parts(texture: &'a Texture) -> Self {
        Self { texture }
    }

    /// Returns the texture that this [Graphics] instance will draw to.
    #[inline]
    pub fn texture(&self) -> &Texture {
        self.texture
    }

    /// Returns the [Context] used by the [Graphics].
    #[inline]
    pub fn context(&self) -> &Context {
        self.texture().context()
    }

    /// Returns the WGPU texture that is being written to.
    #[inline]
    pub fn wgpu_texture(&self) -> &wgpu::Texture {
        self.texture().wgpu_texture()
    }

    /// Returns an immutable view into the pixels of the underlying texture.
    pub async fn pixels(&self) -> Pixels {
        let buffer_row_width_bytes =
            self.wgpu_texture()
                .width()
                .next_multiple_of(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT) as usize
                * Rgba::SIZE;

        // Transfer the texture's pixels to a buffer on the GPU
        let buffer = Texture::texture_to_buffer(self.context(), self.wgpu_texture()).await;

        let mut data = Vec::with_capacity(
            Rgba::SIZE * (self.wgpu_texture().width() * self.wgpu_texture().height()) as usize,
        );

        buffer
            .slice(..)
            .get_mapped_range()
            .chunks(buffer_row_width_bytes)
            .for_each(|row| {
                data.extend_from_slice(&row[0..self.wgpu_texture().width() as usize * Rgba::SIZE])
            });

        // dbg!(&data);

        Pixels::new(
            data,
            Dimension::new(self.wgpu_texture().width(), self.wgpu_texture().height()),
        )
    }

    /// Accesses the raw pixels of the [Graphics], allowing the provided function to modify the
    /// texture on the GPU.
    ///
    /// NOTE: this method should be used sparingly as it moves data to and from the GPU, which can
    ///       be computationally expensive.
    pub async fn pixels_mut<T>(&self, mut callback: impl FnMut(PixelsMut) -> T) -> T {
        let buffer_row_width_bytes =
            self.wgpu_texture()
                .width()
                .next_multiple_of(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT) as usize
                * Rgba::SIZE;

        // Transfer the texture's pixels to a buffer on the GPU
        let input_buffer = Texture::texture_to_buffer(self.context(), self.wgpu_texture()).await;

        // Get the data once mapped on the CPU.
        let mut input_data = input_buffer.slice(..).get_mapped_range_mut();

        // Call the callback
        let size = Dimension::new(self.wgpu_texture().width(), self.wgpu_texture().height());
        let return_value = callback(PixelsMut::new(
            &mut input_data,
            buffer_row_width_bytes,
            size,
        ));

        // Remove the row padding and get the final texture data.
        let mut output_data = Vec::with_capacity(
            Rgba::SIZE * (self.wgpu_texture().width() * self.wgpu_texture().height()) as usize,
        );

        input_data.chunks(buffer_row_width_bytes).for_each(|row| {
            output_data
                .extend_from_slice(&row[0..self.wgpu_texture().width() as usize * Rgba::SIZE])
        });

        // Send the updated data back to the texture
        self.context().wgpu_queue().write_texture(
            self.wgpu_texture().as_image_copy(),
            &output_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(Rgba::SIZE as u32 * self.wgpu_texture().width()),
                rows_per_image: Some(self.wgpu_texture().height()),
            },
            Extent3d {
                width: self.wgpu_texture().width(),
                height: self.wgpu_texture().height(),
                depth_or_array_layers: 1,
            },
        );

        return_value
    }
}
