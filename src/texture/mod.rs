mod owned;

pub use owned::*;

use crate::{Canvas, Dimension, Handle};

/// A G2d texture.
///
/// A texture can be any object which has a G2d [Handle] and a [`wgpu::Texture`].
pub trait Texture<H: Handle> {
    /// Returns the [Handle] used to create this [Texture].
    fn handle(&self) -> &H;

    /// Returns the [`wgpu::Texture`] that this [Texture] wraps.
    fn wgpu_texture(&self) -> &wgpu::Texture;

    /// Returns the usages of this [Texture] allowed for the GPU.  Any operations on this [Texture]
    /// not contained in this set will fail.
    #[inline]
    fn wgpu_texture_usage(&self) -> wgpu::TextureUsages {
        self.wgpu_texture().usage()
    }

    /// Returns the size of this [Texture], in pixels.
    #[inline]
    fn size(&self) -> Dimension {
        Dimension::new(self.wgpu_texture().width(), self.wgpu_texture().height())
    }

    /// Creates a [Canvas] for drawing to this [Texture].
    #[inline]
    fn canvas(&self) -> Canvas<'_, H> {
        Canvas::from_raw_parts(self.handle(), self.wgpu_texture())
    }
}
