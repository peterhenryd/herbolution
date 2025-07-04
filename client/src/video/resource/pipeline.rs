use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthStencilState, Face, FragmentState, FrontFace, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
    TextureFormat, VertexBufferLayout, VertexState,
};

use crate::video::gpu::Handle;
use crate::video::resource::bind_group::BindGroup;
use crate::video::resource::texture::SampleCount;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct PipelineMap<K> {
    map: HashMap<K, RenderPipeline>,
    bind_groups: Vec<BindGroup>,
    sample_count: SampleCount,
}

impl<R: PipelineType> PipelineMap<R> {
    pub fn create<'a>(gpu: &Handle, state: &R::Options<'_>, sample_count: SampleCount) -> Self {
        let bind_groups = R::create_bind_groups(gpu, state);

        let mut map = HashMap::with_capacity(R::ENTRIES.len());
        for key in R::ENTRIES {
            let bind_group_layouts = bind_groups
                .iter()
                .map(|x| &x.layout)
                .enumerate()
                .filter(|(i, _)| key.is_bind_group_enabled(*i))
                .map(|(_, x)| x)
                .collect::<Vec<_>>();

            let options = key.pipeline_options(gpu, state);
            let render_pipeline = gpu.create_pipeline(&bind_group_layouts, sample_count, options);

            map.insert(*key, render_pipeline);
        }

        Self {
            map,
            bind_groups,
            sample_count,
        }
    }

    pub fn set_sample_count(&mut self, gpu: &Handle, sample_count: SampleCount, state: &R::Options<'_>) {
        if self.sample_count == sample_count {
            return;
        }

        for (key, render_pipeline) in self.map.iter_mut() {
            let bind_group_layouts = self
                .bind_groups
                .iter()
                .map(|x| &x.layout)
                .enumerate()
                .filter(|(i, _)| key.is_bind_group_enabled(*i))
                .map(|(_, x)| x)
                .collect::<Vec<_>>();

            let options = key.pipeline_options(gpu, state);
            *render_pipeline = gpu.create_pipeline(&bind_group_layouts, sample_count, options);
        }

        self.sample_count = sample_count;
    }

    pub fn load_by_type(&self, render_type: R, render_pass: &mut RenderPass<'_>) {
        let render_pipeline = self
            .map
            .get(&render_type)
            .expect("Unknown video type");
        render_pass.set_pipeline(render_pipeline);

        for (i, bind_group) in self.bind_groups.iter().enumerate() {
            if render_type.is_bind_group_enabled(i) {
                render_pass.set_bind_group(i as u32, &bind_group.inner, &[]);
            }
        }
    }
}

pub trait PipelineType: Copy + Eq + Hash + 'static {
    type Options<'a>;

    const ENTRIES: &'static [Self];

    fn create_bind_groups(gpu: &Handle, options: &Self::Options<'_>) -> Vec<BindGroup>;

    fn pipeline_options<'a>(&self, gpu: &Handle, options: &Self::Options<'a>) -> PipelineOptions<'a>;

    fn is_bind_group_enabled(&self, _index: usize) -> bool {
        true
    }
}

pub struct PipelineOptions<'a> {
    pub shader_module: &'a ShaderModule,
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
