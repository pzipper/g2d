use crate::Dimension;

/// The pixels of a texture.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pixels {
    /// The size of the texture.
    size: Dimension,

    /// The pixels of the texture.
    buffer: Vec<u8>,
}

impl Pixels {
    /// Creates a [Pixels] instance from its raw parts.
    #[inline]
    pub fn from_raw_parts(size: Dimension, buffer: Vec<u8>) -> Self {
        Self { size, buffer }
    }

    /// Creates a [`Vec<u8>`] from this [Pixels] buffer.
    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    /// Creates a [`Vec<u8>`] from this [Pixels] buffer.
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }
}

impl std::ops::Deref for Pixels {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl std::ops::DerefMut for Pixels {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}
