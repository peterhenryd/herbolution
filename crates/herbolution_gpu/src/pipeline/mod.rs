pub mod map;

pub use map::Key;
pub use wgpu::{vertex_attr_array, Face, VertexBufferLayout, VertexStepMode};
use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthStencilState, FragmentState, FrontFace, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, TextureFormat, VertexState,
};

use crate::handle::Handle;
use crate::shader;
use crate::texture::SampleCount;

pub struct PipelineOptions<'a> {
    pub shader_module: &'a shader::Module,
    pub vertex_buffer_layouts: &'a [VertexBufferLayout<'a>],
    pub cull_mode: Option<Face>,
    pub depth_write_enabled: bool,
}

impl Handle {
    pub fn create_pipeline(&self, bind_group_layouts: &[&BindGroupLayout], sample_count: SampleCount, options: PipelineOptions) -> RenderPipeline {
        let pipeline_layout = self
            .device()
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts,
                push_constant_ranges: &[],
            });

        self.device()
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: options.shader_module,
                    entry_point: Some("vs"),
                    compilation_options: Default::default(),
                    buffers: options.vertex_buffer_layouts,
                },
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: options.cull_mode,
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(DepthStencilState {
                    format: TextureFormat::Depth32Float,
                    depth_write_enabled: options.depth_write_enabled,
                    depth_compare: CompareFunction::LessEqual,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: MultisampleState {
                    count: sample_count.get(),
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(FragmentState {
                    module: options.shader_module,
                    entry_point: Some("fs"),
                    compilation_options: Default::default(),
                    targets: &[Some(ColorTargetState {
                        format: TextureFormat::Bgra8UnormSrgb,
                        blend: Some(BlendState::ALPHA_BLENDING),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                multiview: None,
                cache: None,
            })
    }
}
