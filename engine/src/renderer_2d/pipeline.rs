use wgpu::{include_wgsl, Face, RenderPass, RenderPipeline, TextureFormat};
use math::proj::Orthographic;
use crate::camera::Camera;
use crate::gpu::handle::Handle;
use crate::gpu::mem::bind_group::BindGroupSet;
use crate::gpu::mem::buffer::UnaryBuffer;

pub struct Pipeline2D {
    render_pipeline: RenderPipeline,
    bind_group_set: BindGroupSet,
}

impl Pipeline2D {
    pub fn create(handle: &Handle, camera: &UnaryBuffer<Camera<Orthographic>>, format: TextureFormat) -> Self {
        let bind_group_set = BindGroupSet::build(handle)
            .build_group(|builder| builder.with_entries(camera))
            .finish();
        let render_pipeline = handle.create_render_pipeline(
            Face::Back,
            &bind_group_set,
            include_wgsl!("shader.wgsl"),
            &super::vertex::buffer_layouts(),
            format,
            true
        );

        Self {
            render_pipeline,
            bind_group_set,
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        self.bind_group_set.bind_consecutive(render_pass, 0);
    }
}