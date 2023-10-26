use wgpu::Extent3d;

use crate::{
    math::{Dimension, Rgba},
    Context, PixelsMut, Texture,
};

/// The main struct for Graphics in G2d.
#[derive(Debug)]
pub struct Graphics<'cx, 'tex> {
    /// The context used to create the [`wgpu::Texture`] for this [Graphics].
    context: &'cx Context,

    /// The WGPU texture to draw to.
    wgpu_texture: &'tex wgpu::Texture,
}

impl<'cx, 'tex> Graphics<'cx, 'tex> {
    /// Creates a [Graphics] instance for the provided WGPU texture.
    #[inline]
    pub fn from_raw_parts(context: &'cx Context, wgpu_texture: &'tex wgpu::Texture) -> Self {
        Self {
            context,
            wgpu_texture,
        }
    }

    /// Returns the [Context] used to create this [Texture].
    #[inline]
    pub fn context(&self) -> &Context {
        self.context
    }

    /// Returns the WGPU texture that this [Texture] represents.
    #[inline]
    pub fn wgpu_texture(&self) -> &wgpu::Texture {
        self.wgpu_texture
    }

    /// Accesses the raw pixels of the [Graphics], allowing the provided function to modify the
    /// texture on the GPU.
    pub async fn pixels_mut<T>(&self, mut callback: impl FnMut(PixelsMut) -> T) -> T {
        // TODO: ensure the queues are executed before retrieving the texture's pixels so that the
        //       pixels sent to the callback are updated.

        let buffer_row_width_bytes =
            self.wgpu_texture()
                .width()
                .next_multiple_of(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT) as usize
                * Rgba::SIZE;

        // Transfer the texture's pixels to a buffer on the GPU
        let input_buffer = Texture::texture_to_buffer(self.context(), self.wgpu_texture()).await;

        let return_value = {
            // Transfer the pixels to a buffer on the GPU
            let mut input_data = input_buffer.slice(..).get_mapped_range_mut();

            // Call the callback
            let return_value = callback(PixelsMut::new(
                &mut input_data,
                buffer_row_width_bytes,
                Dimension::new(self.wgpu_texture().width(), self.wgpu_texture().height()),
            ));

            // Send the updated data back to the texture

            // Create the output buffer
            let output_buffer_size = (buffer_row_width_bytes
                * self.wgpu_texture().height() as usize)
                as wgpu::BufferAddress;
            let output_buffer =
                self.context()
                    .wgpu_device()
                    .create_buffer(&wgpu::BufferDescriptor {
                        size: output_buffer_size,
                        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
                        label: None,
                        mapped_at_creation: false,
                    });

            // Write the new data to the buffer
            self.context()
                .wgpu_queue()
                .write_buffer(&output_buffer, 0, &input_data);

            // Send the buffer data to the texture
            let mut encoder = self
                .context()
                .wgpu_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            encoder.copy_buffer_to_texture(
                wgpu::ImageCopyBuffer {
                    buffer: &output_buffer,
                    layout: wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: Some(buffer_row_width_bytes as u32),
                        rows_per_image: Some(self.wgpu_texture().height()),
                    },
                },
                self.wgpu_texture().as_image_copy(),
                Extent3d {
                    width: self.wgpu_texture().width(),
                    height: self.wgpu_texture().height(),
                    depth_or_array_layers: 1,
                },
            );

            // Submit the commands to the queue.
            self.context().wgpu_queue().submit(Some(encoder.finish()));

            return_value
        };

        input_buffer.unmap(); // unmap now that we're done with it.

        return_value
    }
}
