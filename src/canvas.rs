use wgpu::TextureUsages;

use crate::{Color, Error, Handle, Pixels, Texture};

/// A view into a [Texture] used for reading or writing to it.
#[derive(Debug)]
pub struct Canvas<'a, H: Handle> {
    handle: &'a H,
    wgpu_texture: &'a wgpu::Texture,
}

impl<'a, H: Handle> Canvas<'a, H> {
    /// Creates an [Canvas] from its raw parts.
    ///
    /// NOTE: the provided [`wgpu::Texture`] should have been created from the provided [Handle].
    #[inline]
    pub fn from_raw_parts(handle: &'a H, wgpu_texture: &'a wgpu::Texture) -> Self {
        Self {
            handle,
            wgpu_texture,
        }
    }

    /// Writes data directly to a [Texture].
    ///
    /// # Fails
    /// - Fails if the underlying data is too big or small.
    /// - Fails if the [Texture] doesn't have the `COPY_DST` usage.
    pub fn write(&self, data: &[u8]) -> Result<(), Error> {
        if !self
            .wgpu_texture_usage()
            .contains(wgpu::TextureUsages::COPY_DST)
        {
            return Err(Error::LackingTextureUsage(wgpu::TextureUsages::COPY_DST));
        }

        if data.len() as u32 != self.size().area() * 4 {
            return Err(Error::TextureDataSizeMismatch {
                expected: self.size().area() * 4,
                got: data.len() as u32,
            });
        }

        self.handle().wgpu_queue().write_texture(
            self.wgpu_texture().as_image_copy(),
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(self.size().width * 4),
                rows_per_image: None,
            },
            self.size().to_extent_3d(),
        );

        Ok(())
    }

    /// Attempts to get the pixels of this [Texture].
    ///
    /// # Fails
    /// - Fails if the texture doesn't have the `COPY_SRC` usage.
    pub async fn pixels(&self) -> Result<Pixels, Error> {
        let mut wgpu_encoder = self
            .handle()
            .wgpu_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // The width of the output buffer must be a multiple of 256
        let padded_width = self
            .wgpu_texture()
            .width()
            .next_multiple_of(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);

        let output_buffer_size =
            ((4 * padded_width) * self.wgpu_texture().height()) as wgpu::BufferAddress;
        let output_buffer = self
            .handle()
            .wgpu_device()
            .create_buffer(&wgpu::BufferDescriptor {
                size: output_buffer_size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                label: None,
                mapped_at_creation: false,
            });

        // Write the data of the texture to the output buffer
        wgpu_encoder.copy_texture_to_buffer(
            self.wgpu_texture().as_image_copy(),
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * padded_width),
                    rows_per_image: None,
                },
            },
            self.size().to_extent_3d(),
        );

        // Submit the commands to the queue.
        self.handle()
            .wgpu_queue()
            .submit(Some(wgpu_encoder.finish()));

        // Map the buffer.
        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        output_buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
        self.handle().wgpu_device().poll(wgpu::Maintain::Wait);
        rx.receive().await.unwrap().unwrap();

        // Remove the row padding from the buffer
        let mut pixel_data = Vec::with_capacity(self.size().area() as usize * 4);

        output_buffer
            .slice(..)
            .get_mapped_range()
            .chunks(padded_width as usize * 4)
            .for_each(|row| pixel_data.extend_from_slice(&row[0..self.size().width as usize * 4]));

        Ok(Pixels::from_raw_parts(self.size(), pixel_data))
    }

    /// Clears this [Texture], filling it with the provided color.
    ///
    /// # Fails
    /// - Fails if this [Texture] does not have the `RENDER_ATTACHMENT` usage.
    pub fn clear(&self, color: Color) -> Result<(), Error> {
        if !self
            .wgpu_texture_usage()
            .contains(TextureUsages::RENDER_ATTACHMENT)
        {
            return Err(Error::LackingTextureUsage(TextureUsages::RENDER_ATTACHMENT));
        }

        let wgpu_texture_view = self
            .wgpu_texture()
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .handle()
            .wgpu_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // Begin the clear render pass.
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &wgpu_texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color.to_wgpu_color()),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Submit to be drawn.
        self.handle()
            .wgpu_queue()
            .submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

impl<'a, H: Handle> Texture<H> for Canvas<'a, H> {
    fn handle(&self) -> &H {
        self.handle
    }

    fn wgpu_texture(&self) -> &wgpu::Texture {
        self.wgpu_texture
    }
}
