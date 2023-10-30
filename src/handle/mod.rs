mod window;
mod windowless;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    TextureUsages,
};
pub use window::*;
pub use windowless::*;

use crate::{Dimension, Error, OwnedTexture, Paint, Vertex, VertexBuffer};

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

/// Creates a [`wgpu::RenderPipeline`] for with the provided options.
pub(crate) fn create_wgpu_render_pipeline(
    wgpu_device: &wgpu::Device,
    wgpu_render_pipeline_layout: &wgpu::PipelineLayout,
    wgpu_shader: &wgpu::ShaderModule,
) -> wgpu::RenderPipeline {
    wgpu_device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(wgpu_render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &wgpu_shader,
            entry_point: "vs_main",
            // NOTE: the kind of [Handle] does not affect the layout of a [VertexBuffer].  It is
            //       simply needed to access the `wgpu_desc` method.
            buffers: &[VertexBuffer::<WindowlessHandle>::wgpu_desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &wgpu_shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}

/// Creates a [`wgpu::RenderPipeline`] for rendering basic color-filled shapes.
pub(crate) fn paint_fill_pipeline(wgpu_device: &wgpu::Device) -> wgpu::RenderPipeline {
    let wgpu_shader =
        wgpu_device.create_shader_module(wgpu::include_wgsl!("../shaders/paint_fill.wgsl"));

    let wgpu_render_pipeline_layout =
        wgpu_device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

    create_wgpu_render_pipeline(wgpu_device, &wgpu_render_pipeline_layout, &wgpu_shader)
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

        VertexBuffer::from_raw_parts(self, wgpu_buffer, data.len() as wgpu::BufferAddress)
    }

    /// Returns the [`wgpu::RenderPipeline`] for the provided [Paint] type.
    fn wgpu_render_pipeline_for_paint(&self, paint: &Paint) -> &wgpu::RenderPipeline;
}
