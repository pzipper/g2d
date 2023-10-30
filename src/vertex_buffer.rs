use crate::{Handle, Vertex};

/// The attributes for `wgpu_desc`.
const WGPU_ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
    0 => Float32x2, // position
    1 => Float32x2, // uv
    2 => Float32x4, // color
];

/// A buffer of vertices, stored on the GPU.
#[derive(Debug)]
pub struct VertexBuffer<'a, H: Handle> {
    handle: &'a H,
    wgpu_buffer: wgpu::Buffer,
    len: wgpu::BufferAddress,
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
    pub fn from_raw_parts(
        handle: &'a H,
        wgpu_buffer: wgpu::Buffer,
        len: wgpu::BufferAddress,
    ) -> Self {
        Self {
            handle,
            wgpu_buffer,
            len,
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

    /// Returns the number of [Vertices](Vertex) in this [VertexBuffer]
    #[inline]
    pub fn len(&self) -> wgpu::BufferAddress {
        self.len
    }
}
