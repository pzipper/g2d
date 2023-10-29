use crate::{Handle, Vertex};

/// A buffer of vertices, stored on the GPU.
#[derive(Debug)]
pub struct VertexBuffer<'a, H: Handle> {
    handle: &'a H,
    wgpu_buffer: wgpu::Buffer,
}

impl<'a, H: Handle> VertexBuffer<'a, H> {
    /// Returns the [`wgpu::VertexBufferLayout`] [VertexBuffer]s use.
    pub const WGPU_VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> =
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // UV
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float64x4,
                },
            ],
        };

    /// Creates a [VertexBuffer] from its raw parts.
    ///
    /// The provided buffer should have been created with the provided [Handle].
    #[inline]
    pub fn from_raw_parts(handle: &'a H, wgpu_buffer: wgpu::Buffer) -> Self {
        Self {
            handle,
            wgpu_buffer,
        }
    }

    /// Returns the [Handle] used to create this [VertexBuffer].
    #[inline]
    pub fn handle(&self) -> &H {
        self.handle
    }

    /// Returns the [`wgpu::Buffer`] this [VertexBuffer] represents.
    #[inline]
    pub fn wgpu_buffer(&self) -> &wgpu::Buffer {
        &self.wgpu_buffer
    }
}
