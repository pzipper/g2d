use crate::{Handle, Texture};

/// A [Texture] on the GPU owned by the CPU.
pub struct OwnedTexture<'a, H: Handle> {
    handle: &'a H,
    wgpu_texture: wgpu::Texture,
}

impl<'a, H: Handle> OwnedTexture<'a, H> {
    /// Creates an [OwnedTexture] from its raw parts.
    ///
    /// NOTE: the provided [`wgpu::Texture`] should have been created from the provided [Handle].
    #[inline]
    pub fn from_raw_parts(handle: &'a H, wgpu_texture: wgpu::Texture) -> Self {
        Self {
            handle,
            wgpu_texture,
        }
    }
}

impl<'a, H: Handle> Texture<H> for OwnedTexture<'a, H> {
    fn handle(&self) -> &H {
        self.handle
    }

    fn wgpu_texture(&self) -> &wgpu::Texture {
        &self.wgpu_texture
    }
}
