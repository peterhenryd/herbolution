pub mod map;

pub use map::Key;
use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthStencilState, FragmentState, FrontFace, PipelineLayoutDescriptor,
    PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, TextureFormat, VertexState,
};
pub use wgpu::{Face, VertexBufferLayout, VertexStepMode, vertex_attr_array};

use crate::handle::Handle;
use crate::shader;

pub type Map<K, const N: usize> = map::PipelineMap<K, N>;

pub struct PipelineOptions<'a> {
    pub shader_module: &'a shader::Module,
    pub vertex_buffer_layouts: &'a [VertexBufferLayout<'a>],
    pub cull_mode: Option<Face>,
    pub depth_write_enabled: bool,
}

impl Handle {
    pub fn create_pipeline(&self, bind_group_layouts: &[&BindGroupLayout], options: PipelineOptions) -> RenderPipeline {
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
                    entry_point: None,
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
                    depth_compare: CompareFunction::Less,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: Default::default(),
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
