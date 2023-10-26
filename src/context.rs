use crate::{math::Dimension, Texture};

/// Stores the state required for rendering, such as handles to the GPU.
///
/// Generally, only one context should exist at a time.
#[derive(Debug)]
pub struct Context {
    pub(crate) wgpu_queue: wgpu::Queue,
    pub(crate) wgpu_device: wgpu::Device,
}

impl Context {
    /// Initializes a [Context].
    pub async fn new() -> Self {
        // Initialize WGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            // TODO: support browser/WASM?
            backends: wgpu::Backends::all().difference(wgpu::Backends::BROWSER_WEBGPU),
            dx12_shader_compiler: Default::default(),
            flags: wgpu::InstanceFlags::debugging(),
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .expect("should be able to get adapter");

        // Request a device
        let (wgpu_device, wgpu_queue) = adapter
            .request_device(&Default::default(), None)
            .await
            .expect("should be able to get device");

        Self {
            wgpu_device,
            wgpu_queue,
        }
    }

    /// Returns a reference to the [`wgpu::Device`] used by the [Context].
    #[inline]
    pub fn wgpu_device(&self) -> &wgpu::Device {
        &self.wgpu_device
    }

    /// Returns a reference to the [`wgpu::Queue`] used by the [Context].
    #[inline]
    pub fn wgpu_queue(&self) -> &wgpu::Queue {
        &self.wgpu_queue
    }

    /// Creates a blank texture with the provided size.
    ///
    /// note: Textures default to the [`wgpu::TextureFormat::Rgba8UnormSrgb`] format.
    ///
    /// # Panics
    /// Panics if the width or height of the provided dimension are zero.
    pub fn make_blank_texture(&self, size: Dimension) -> Texture {
        assert!(
            size.width > 0 && size.height > 0,
            "width & height should be > 0"
        );

        // Create an empty texture
        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            label: None,
            view_formats: &[],
        };

        Texture::from_raw_parts(self, self.wgpu_device().create_texture(&texture_desc))
    }

    /// Creates a texture pre-initialized with the provided data.
    pub fn make_texture(&self, size: Dimension, data: &[u8]) -> Texture {
        let texture = self.make_blank_texture(size);
        texture.graphics().overwrite_pixel_data(data);
        texture
    }
}
