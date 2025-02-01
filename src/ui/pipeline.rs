use crate::app::gpu::Gpu;
use crate::app::surface::Surface;
use crate::ui::vertex::UiVertex;
use crate::ui::Projection;
use std::ops::Deref;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{include_wgsl, BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Queue, RenderPipeline, ShaderStages};

pub struct UiPipeline {
    queue: Queue,
    render_pipeline: RenderPipeline,
    uniform_buffer: Buffer,
    pub(crate) uniform_bind_group: BindGroup,
}

impl UiPipeline {
    pub fn create(gpu: &Gpu, surface: &Surface, projection: &Projection) -> Self {
        let (uniform_buffer, layout, bind_group) = create_uniforms(gpu, projection);
        let bind_group_layouts = [&layout];
        let render_pipeline = gpu.create_render_pipeline(
            "ui",
            &bind_group_layouts,
            include_wgsl!("shader.wgsl"),
            &[UiVertex::layout()],
            surface.get_texture_format()
        );

        Self {
            queue: gpu.queue.clone(),
            render_pipeline,
            uniform_buffer,
            uniform_bind_group: bind_group,
        }
    }

    pub fn update_uniforms(&self, projection: &Projection) {
        self.queue.write_buffer(&self.uniform_buffer, 0, projection.as_ref());
    }
}

impl Deref for UiPipeline {
    type Target = RenderPipeline;

    fn deref(&self) -> &RenderPipeline {
        &self.render_pipeline
    }
}

fn create_uniforms(gpu: &Gpu, projection: &Projection) -> (Buffer, BindGroupLayout, BindGroup) {
    let buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("herbolution_ui_uniform_buffer"),
        contents: projection.as_ref(),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });
    let layout = gpu.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("herbolution_ui_uniform_bind_group_layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }
        ],
    });
    let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("herbolution_ui_uniform_bind_group"),
        layout: &layout,
        entries: &[BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() }],
    });

    (buffer, layout, bind_group)
}