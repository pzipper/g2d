use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use crate::{Dimension, Error, Frame, Handle};

/// A [Handle] to the G2d API which is initialized for a specific window.
#[derive(Debug)]
pub struct WindowHandle {
    wgpu_surface: wgpu::Surface,
    wgpu_surface_config: wgpu::SurfaceConfiguration,
    surface_size: Dimension,
    wgpu_device: wgpu::Device,
    wgpu_queue: wgpu::Queue,
}

impl WindowHandle {
    /// Attempts to create a [WindowHandle] for the provided window.
    ///
    /// The provided size should be the current size of the window's surface.
    pub async fn new<W: HasRawDisplayHandle + HasRawWindowHandle>(
        window: &W,
        surface_size: Dimension,
    ) -> Result<Self, Error> {
        // Initialize WGPU
        let wgpu_instance = super::create_wgpu_instance();
        let wgpu_surface = unsafe {
            wgpu_instance
                .create_surface(window)
                .map_err(|err| Error::FailedToCreateSurface(err.to_string()))?
        };
        let wgpu_adapter = super::request_wgpu_adapter(&wgpu_instance, Some(&wgpu_surface)).await?;
        let (wgpu_device, wgpu_queue) = super::request_wgpu_device(&wgpu_adapter).await?;

        // Configure surface
        let wgpu_surface_caps = wgpu_surface.get_capabilities(&wgpu_adapter);
        // Assumes an sRGB surface texture. Using a different one will result all the colors coming
        // out darker.
        let surface_format = wgpu_surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(wgpu_surface_caps.formats[0]);
        let wgpu_surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: surface_size.width,
            height: surface_size.height,
            present_mode: wgpu_surface_caps.present_modes[0],
            alpha_mode: wgpu_surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        wgpu_surface.configure(&wgpu_device, &wgpu_surface_config);

        Ok(Self {
            wgpu_surface,
            wgpu_surface_config,
            surface_size,
            wgpu_device,
            wgpu_queue,
        })
    }

    /// Returns the [`wgpu::Surface`] this [WindowHandle] uses.
    #[inline]
    pub fn wgpu_surface(&self) -> &wgpu::Surface {
        &self.wgpu_surface
    }

    /// Resizes the surface of this [WindowHandle].
    ///
    /// Should be called when the window's manager emits a resize event.  Does nothing if the new
    /// size is zero.
    pub fn resize_surface(&mut self, new_size: Dimension) {
        if new_size.area() > 0 {
            self.surface_size = new_size;
            self.wgpu_surface_config.width = new_size.width;
            self.wgpu_surface_config.height = new_size.height;
            self.wgpu_surface
                .configure(&self.wgpu_device(), &self.wgpu_surface_config);
        }
    }

    /// Creates a [Frame] that can be used to draw to the window.
    ///
    /// # Fails
    /// - Fails if a [Frame] is already alive.  TODO: verify: is true?
    #[inline]
    pub fn frame(&self) -> Result<Frame<'_, Self>, Error> {
        Ok(Frame::from_raw_parts(
            self,
            self.wgpu_surface()
                .get_current_texture()
                .map_err(|err| Error::FailedToGetSurfaceTexture(err.to_string()))?,
        ))
    }
}

impl Handle for WindowHandle {
    fn wgpu_device(&self) -> &wgpu::Device {
        &self.wgpu_device
    }

    fn wgpu_queue(&self) -> &wgpu::Queue {
        &self.wgpu_queue
    }
}
