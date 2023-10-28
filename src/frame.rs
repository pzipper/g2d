use crate::{Handle, Texture};

/// The current frame of a window.
pub struct Frame<'a, H: Handle> {
    handle: &'a H,
    wgpu_surface_texture: wgpu::SurfaceTexture,
}

impl<'a, H: Handle> Frame<'a, H> {
    /// Creates a [Frame] from its raw parts.
    #[inline]
    pub fn from_raw_parts(handle: &'a H, wgpu_surface_texture: wgpu::SurfaceTexture) -> Self {
        Self {
            handle,
            wgpu_surface_texture,
        }
    }

    /// Returns the raw [`wgpu::SurfaceTexture`] this [Frame] wraps.
    #[inline]
    pub fn wgpu_surface_texture(&self) -> &wgpu::SurfaceTexture {
        &self.wgpu_surface_texture
    }

    /// Consumes this [Frame] and returns the raw [`wgpu::SurfaceTexture`].
    #[inline]
    pub fn into_wgpu_surface_texture(self) -> wgpu::SurfaceTexture {
        self.wgpu_surface_texture
    }

    /// Sends this [Frame] to its window to be displayed.
    #[inline]
    pub fn present(self) {
        self.wgpu_surface_texture.present()
    }
}

impl<'a, H: Handle> Texture<H> for Frame<'a, H> {
    fn handle(&self) -> &H {
        self.handle
    }

    fn wgpu_texture(&self) -> &wgpu::Texture {
        &self.wgpu_surface_texture.texture
    }
}
