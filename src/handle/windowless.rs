use crate::{Error, Handle, Paint};

/// A [Handle] to the G2d API which doesn't require a window.
#[derive(Debug)]
pub struct WindowlessHandle {
    wgpu_device: wgpu::Device,
    wgpu_queue: wgpu::Queue,

    // Render pipelines for different paints
    paint_fill_pipeline: wgpu::RenderPipeline,
}

impl WindowlessHandle {
    /// Attempts to create a [WindowlessHandle].
    pub async fn new() -> Result<Self, Error> {
        let wgpu_instance = super::create_wgpu_instance();
        let wgpu_adapter = super::request_wgpu_adapter(&wgpu_instance, None).await?;
        let (wgpu_device, wgpu_queue) = super::request_wgpu_device(&wgpu_adapter).await?;

        Ok(Self {
            paint_fill_pipeline: super::paint_fill_pipeline(&wgpu_device),

            wgpu_device,
            wgpu_queue,
        })
    }
}

impl Handle for WindowlessHandle {
    fn wgpu_device(&self) -> &wgpu::Device {
        &self.wgpu_device
    }

    fn wgpu_queue(&self) -> &wgpu::Queue {
        &self.wgpu_queue
    }

    fn wgpu_render_pipeline_for_paint(&self, paint: &Paint) -> &wgpu::RenderPipeline {
        match paint {
            Paint::Fill => &self.paint_fill_pipeline,
        }
    }
}
