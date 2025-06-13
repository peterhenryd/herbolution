use crate::gpu::Handle;
use crate::mem::bind_group::BindGroup;
use std::collections::HashMap;
use std::hash::Hash;
use wgpu::{BindGroupLayout, BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthStencilState, Face, FragmentState, FrontFace, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule, TextureFormat, VertexBufferLayout, VertexState};

#[derive(Debug)]
pub struct Pipelines<R, const N: usize>{
    map: HashMap<R, RenderPipeline>,
    bind_groups: [BindGroup; N],
}

impl<R: RenderType<N>, const N: usize> Pipelines<R, N> {
    pub fn create<'a>(handle: &Handle, state: &R::Options<'_>) -> Self {
        let bind_groups = R::create_bind_groups(handle, state);

        let mut map = HashMap::with_capacity(N);
        for entry in R::ENTRIES {
            let bind_group_included = entry.bind_groups();
            let bind_group_layouts = bind_groups.iter()
                .map(|x| &x.layout)
                .enumerate()
                .filter(|(i, _)| bind_group_included[*i])
                .map(|(_, x)| x)
                .collect::<Vec<_>>();

            let options = entry.pipeline_options(handle, state);
            let render_pipeline = create(handle, &bind_group_layouts, options);
            
            map.insert(*entry, render_pipeline);
        }
        
        Self { map, bind_groups, }
    }
    
    pub fn load_into_render_pass(&self, render_type: R, render_pass: &mut RenderPass<'_>) {
        let render_pipeline = self.map.get(&render_type).expect("Unknown render type");
        render_pass.set_pipeline(render_pipeline);
        
        let bind_group_enabled = render_type.bind_groups();
        for (i, bind_group) in self.bind_groups.iter().enumerate() {
            if bind_group_enabled[i] {
                render_pass.set_bind_group(i as u32, &bind_group.inner, &[]);
            }
        }
    }
}

pub trait RenderType<const N: usize>: Copy + Eq + Hash + 'static {
    type Options<'a>;

    const ENTRIES: &'static [Self];
    
    fn create_bind_groups(handle: &Handle, options: &Self::Options<'_>) -> [BindGroup; N];
    
    fn pipeline_options<'a>(&self, handle: &Handle, options: &Self::Options<'a>) -> PipelineOptions<'a>;
    
    fn bind_groups(&self) -> [bool; N] {
        [true; N]
    }
}

pub struct PipelineOptions<'a> {
    pub shader_module: &'a ShaderModule,
    pub vertex_buffer_layouts: &'a [VertexBufferLayout<'a>],
    pub cull_mode: Option<Face>,
    pub depth_write_enabled: bool,
}

pub fn create(handle: &Handle, bind_group_layouts: &[&BindGroupLayout], options: PipelineOptions) -> RenderPipeline {
    let pipeline_layout = handle.device()
        .create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts,
            push_constant_ranges: &[],
        });
    
    handle.device()
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
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        })
}