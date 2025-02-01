use wgpu::{include_wgsl, RenderPass, RenderPipeline, TextureFormat};
use crate::app::gpu::Gpu;
use crate::world::vertex::WorldVertex;

pub struct WorldPipeline {
    render_pipeline: RenderPipeline,
}

impl WorldPipeline {
    pub fn create(gpu: &Gpu, format: TextureFormat) -> Self {
        let bind_group_layouts = [];
        let render_pipeline = gpu.create_render_pipeline(
            "world",
            &bind_group_layouts,
            include_wgsl!("shader.wgsl"),
            &[WorldVertex::layout()],
            format
        );

        Self {
            render_pipeline,
        }
    }

    pub fn bind_to_render_pass(&self, render_pass: &mut RenderPass<'_>) {
        render_pass.set_pipeline(&self.render_pipeline);
    }
}