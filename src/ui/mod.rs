use std::slice;
use crate::app::gpu::Gpu;
use crate::app::surface::Surface;
use crate::ui::pipeline::UiPipeline;
use crate::ui::text::UiText;
use wgpu::RenderPass;
use winit::dpi::PhysicalSize;

pub mod pipeline;
pub mod text;
pub mod vertex;

pub struct Ui {
    pipeline: UiPipeline,
    pub(crate) text: UiText,
    projection: Projection,
}

impl Ui {
    pub fn create(gpu: &Gpu, surface: &Surface) -> Self {
        let projection = Projection::from(surface.get_size());
        Self {
            pipeline: UiPipeline::create(gpu, surface, &projection),
            text: UiText::create(gpu, surface),
            projection
        }
    }

    pub fn bind_to_render_pass(&self, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.pipeline.uniform_bind_group, &[]);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.projection = Projection::from(size);
        self.pipeline.update_uniforms(&self.projection);
        self.text.resize(size);
    }
}

pub struct Projection([f32; 16]);

impl From<PhysicalSize<u32>> for Projection {
    fn from(PhysicalSize { width, height }: PhysicalSize<u32>) -> Self {
        Self([
            2.0 / width as f32, 0.0, 0.0, 0.0,
            0.0, -2.0 / height as f32, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -1.0, 1.0, 0.0, 1.0,
        ])
    }
}

impl AsRef<[u8]> for Projection {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.0.as_ptr() as *const u8, size_of::<Self>())
        }
    }
}
