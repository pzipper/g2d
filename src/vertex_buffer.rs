use crate::{Handle, Vertex};

/// The attributes for `wgpu_desc`.
const WGPU_ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
    0 => Float32x2,
    1 => Float32x2,
    2 => Float64x4
];

/// A buffer of vertices, stored on the GPU.
#[derive(Debug)]
pub struct VertexBuffer<'a, H: Handle> {
    handle: &'a H,
    wgpu_buffer: wgpu::Buffer,
}

impl<'a, H: Handle> VertexBuffer<'a, H> {
    /// Returns the [`wgpu::VertexBufferLayout`] [VertexBuffer]s use.
    pub const fn wgpu_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &WGPU_ATTRIBS,
        }
    }

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
