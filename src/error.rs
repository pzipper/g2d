/// An error from G2d.
#[derive(Clone, Debug)]
pub enum Error {
    /// When creating a [Handle](crate::Handle), G2d could not find a suitable adapter.
    FailedToAcquireAdapter,

    /// The G2d [Handle](crate::Handle) failed to acquire a [`wgpu::Device`].
    FailedToAcquireDevice(String),

    /// The data for a [Texture](crate::Texture) did not match the [Texture](crate::Texture)'s
    /// size.
    TextureDataSizeMismatch { expected: u32, got: u32 },

    /// The [Texture](crate::Texture) didn't have the correct usage(s) for an operation.
    LackingTextureUsage(wgpu::TextureUsages),

    /// The G2d [Handle](crate::Handle) failed to create a [`wgpu::Surface`] for its window.
    FailedToCreateSurface(String),
}
