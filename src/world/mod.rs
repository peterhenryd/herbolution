mod pipeline;
mod vertex;

use wgpu::RenderPass;
use crate::app::gpu::Gpu;
use crate::app::surface::Surface;
use crate::world::pipeline::WorldPipeline;

/// The entire rendering state for the world of the game.
pub struct World {
    pipeline: WorldPipeline,
}

impl World {
    pub fn create(gpu: &Gpu, surface: &Surface) -> Self {
        Self {
            pipeline: WorldPipeline::create(gpu, surface.get_texture_format()),
        }
    }

    pub fn render(&self, _render_pass: &mut RenderPass<'_>) {
        todo!()
    }
}