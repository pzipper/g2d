mod window;
mod windowless;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    TextureUsages,
};
pub use window::*;
pub use windowless::*;

use crate::{Dimension, Error, OwnedTexture, Vertex, VertexBuffer};

/// Creates a [`wgpu::Instance`] with the default settings for G2d.
#[inline]
pub(crate) fn create_wgpu_instance() -> wgpu::Instance {
    wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        flags: wgpu::InstanceFlags::debugging(),
        ..Default::default()
    })
}

/// Requests a [`wgpu::Adapter`] from the provided [`wgpu::Instance`] with the default
/// configuration for G2d.
#[inline]
pub(crate) async fn request_wgpu_adapter(
    wgpu_instance: &wgpu::Instance,
    compatible_surface: Option<&wgpu::Surface>,
) -> Result<wgpu::Adapter, Error> {
    wgpu_instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface,
            force_fallback_adapter: false,
        })
        .await
        .ok_or(Error::FailedToAcquireAdapter)
}

/// Requests a [`wgpu::Device`] from the provided [`wgpu::Adapter`].
pub(crate) async fn request_wgpu_device(
    wgpu_adapter: &wgpu::Adapter,
) -> Result<(wgpu::Device, wgpu::Queue), Error> {
    wgpu_adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // TODO: support browser targets
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        )
        .await
        .map_err(|err| Error::FailedToAcquireDevice(err.to_string()))
}

/// A handle to the G2d API.
pub trait Handle: Sized {
    /// The [`wgpu::Device`] this [Handle] uses.
    fn wgpu_device(&self) -> &wgpu::Device;

    /// The [`wgpu::Queue`] this [Handle] uses.
    fn wgpu_queue(&self) -> &wgpu::Queue;

    /// Creates a new [Texture](crate::Texture) with the provided size.  Leaves the texture blank.
    ///
    /// NOTE: G2d textures default to the `Rgba8UnormSrgb` format.
    fn make_blank_texture(&self, size: Dimension) -> OwnedTexture<'_, Self> {
        let wgpu_texture = self.wgpu_device().create_texture(&wgpu::TextureDescriptor {
            size: size.to_extent_3d(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::all().difference(TextureUsages::STORAGE_BINDING), // TODO: all?
            label: None,
            view_formats: &[],
        });

        OwnedTexture::from_raw_parts(self, wgpu_texture)
    }

    /// Creates a [Texture](crate::Texture) with the provided size and data.
    ///
    /// Expects the data to be in `Rgba8UnormSrgb`.
    ///
    /// # Fails
    /// Fails if the data is too big or small for the provided size.
    fn make_texture(&self, size: Dimension, data: &[u8]) -> Result<OwnedTexture<'_, Self>, Error> {
        if data.len() as u32 != size.area() * 4 {
            return Err(Error::TextureDataSizeMismatch {
                expected: size.area() * 4,
                got: data.len() as u32,
            });
        }

        let wgpu_texture = self.wgpu_device().create_texture_with_data(
            self.wgpu_queue(),
            &wgpu::TextureDescriptor {
                size: size.to_extent_3d(),
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::all().difference(TextureUsages::STORAGE_BINDING), // TODO: all?
                label: None,
                view_formats: &[],
            },
            data,
        );

        Ok(OwnedTexture::from_raw_parts(self, wgpu_texture))
    }

    /// Creates a new [VertexBuffer], initialized with the provided data.
    fn make_vertex_buffer(&self, data: &[Vertex]) -> VertexBuffer<'_, Self> {
        let wgpu_buffer = self
            .wgpu_device()
            .create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::VERTEX,
            });

        VertexBuffer::from_raw_parts(self, wgpu_buffer)
    }
}
