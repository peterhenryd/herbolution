use wgpu::{BindGroup, BindGroupLayout, BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Face, FragmentState, FrontFace, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule, StencilState, TextureFormat, VertexBufferLayout, VertexState};
use crate::gpu::binding::BindGroupBuilder;
use crate::gpu::Gpu;

pub struct RenderPipelineOptions<'a> {
    pub bind_group_layouts: &'a [&'a BindGroupLayout],
    pub shader_module: &'a ShaderModule,
    pub input: &'a [VertexBufferLayout<'a>],
    pub cull_mode: Face,
    pub depth_write_enabled: bool,
    pub texture_format: TextureFormat,
}

pub trait RenderPipelineExt {
    fn create(gpu: &Gpu, options: RenderPipelineOptions) -> Self;
}

impl RenderPipelineExt for RenderPipeline {
    fn create(gpu: &Gpu, options: RenderPipelineOptions) -> Self {
        let layout = gpu.device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: options.bind_group_layouts,
                push_constant_ranges: &[],
            });

        gpu.device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(&layout),
                vertex: VertexState {
                    module: options.shader_module,
                    entry_point: Some("vs"),
                    compilation_options: PipelineCompilationOptions::default(),
                    buffers: options.input,
                },
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(options.cull_mode),
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(DepthStencilState {
                    format: TextureFormat::Depth32Float,
                    depth_write_enabled: options.depth_write_enabled,
                    depth_compare: CompareFunction::Less,
                    stencil: StencilState::default(),
                    bias: DepthBiasState::default(),
                }),
                multisample: MultisampleState {
                    count: 1,
                    mask: 1,
                    alpha_to_coverage_enabled: true,
                },
                fragment: Some(FragmentState {
                    module: options.shader_module,
                    entry_point: Some("fs"),
                    compilation_options: PipelineCompilationOptions::default(),
                    targets: &[Some(ColorTargetState {
                        format: options.texture_format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                multiview: None,
                cache: None,
            })
    }
}

pub trait BindGroupExt {
    fn build<'a>() -> BindGroupBuilder<'a>;
}

impl BindGroupExt for BindGroup {
    fn build<'a>() -> BindGroupBuilder<'a> {
        BindGroupBuilder::new()
    }
}