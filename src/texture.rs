use wgpu::Extent3d;

use crate::{
    math::{Dimension, Rgba},
    Context, Graphics,
};

/// A G2d texture.
#[derive(Debug)]
pub struct Texture<'cx> {
    /// The context used to create this [Texture].
    context: &'cx Context,

    /// The WGPU texture that this [Texture] represents.
    wgpu_texture: wgpu::Texture,
}

impl<'cx> Texture<'cx> {
    /// Creates a new [Texture] from its WGPU counterpart.
    #[inline]
    pub fn from_raw_parts(context: &'cx Context, wgpu_texture: wgpu::Texture) -> Self {
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
        &self.wgpu_texture
    }

    /// Returns the [Dimension] which represents the size of this [Texture].
    #[inline]
    pub fn dimension(&self) -> Dimension {
        Dimension::new(self.wgpu_texture.width(), self.wgpu_texture.height())
    }

    /// Returns the width of this [Texture].
    #[inline]
    pub fn width(&self) -> u32 {
        self.dimension().width
    }

    /// Returns the height of this [Texture].
    #[inline]
    pub fn height(&self) -> u32 {
        self.dimension().height
    }

    /// Creates a [Graphics] instance for this texture.
    #[inline]
    pub fn graphics(&self) -> Graphics {
        Graphics::from_raw_parts(self.context(), &self.wgpu_texture)
    }

    /// Transfers the pixels of this [Texture] to a buffer on the GPU.
    ///
    /// Returns the buffer and the width of a row in it.  The resulting buffer is already mapped.
    pub(crate) async fn texture_to_buffer(
        context: &'cx Context,
        wgpu_texture: &wgpu::Texture,
    ) -> wgpu::Buffer {
        let mut encoder = context
            .wgpu_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // Create the output buffer

        // The width of the output buffer must be a multiple of 256
        let padded_width = wgpu_texture
            .width()
            .next_multiple_of(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);

        let output_buffer_size =
            ((Rgba::SIZE as u32 * padded_width) * wgpu_texture.height()) as wgpu::BufferAddress;
        let output_buffer = context
            .wgpu_device()
            .create_buffer(&wgpu::BufferDescriptor {
                size: output_buffer_size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                label: None,
                mapped_at_creation: false,
            });

        // Write the data of the texture to the output buffer
        encoder.copy_texture_to_buffer(
            wgpu_texture.as_image_copy(),
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(Rgba::SIZE as u32 * padded_width),
                    rows_per_image: Some(wgpu_texture.height()),
                },
            },
            Extent3d {
                width: wgpu_texture.width(),
                height: wgpu_texture.height(),
                depth_or_array_layers: 1,
            },
        );

        // Submit the commands to the queue.
        context.wgpu_queue().submit(Some(encoder.finish()));

        // Map the buffer.
        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        output_buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
        context.wgpu_device().poll(wgpu::Maintain::Wait);
        rx.receive().await.unwrap().unwrap();

        // Buffer is mapped and ready to go.
        output_buffer
    }
}
