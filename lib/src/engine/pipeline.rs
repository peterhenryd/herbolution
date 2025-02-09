use crate::engine::binding::{Binding, BindingBuilder};
use crate::engine::gpu::Gpu;
use wgpu::{BlendState, ColorTargetState, ColorWrites, DepthStencilState, Face, FragmentState, FrontFace, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, TextureFormat, VertexBufferLayout, VertexState};

pub struct Pipeline {
    render_pipeline: RenderPipeline,
    bindings: Vec<Binding>,
}

impl Pipeline {
    pub fn create(
        gpu: &Gpu,
        name: &str,
        bindings: Vec<Binding>,
        shader: ShaderModuleDescriptor,
        buffers: &[VertexBufferLayout<'_>],
        format: TextureFormat,
        depth_stencil: Option<DepthStencilState>,
    ) -> Self {
        let bind_group_layouts = bindings.iter()
            .map(Binding::layout)
            .collect::<Vec<_>>();
        let pipeline_layout = gpu.device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some(&format!("herbolution_{}_render_pipeline_layout", name)),
                bind_group_layouts: &bind_group_layouts,
                push_constant_ranges: &[],
            });
        let shader_module = gpu.device.create_shader_module(shader);
        let render_pipeline = gpu.device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(&format!("herbolution_{}_render_pipeline", name)),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader_module,
                    entry_point: Some("vs"),
                    compilation_options: PipelineCompilationOptions::default(),
                    buffers,
                },
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(Face::Front),
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil,
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &shader_module,
                    entry_point: Some("fs"),
                    compilation_options: PipelineCompilationOptions::default(),
                    targets: &[Some(ColorTargetState {
                        format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::all(),
                    })],
                }),
                multiview: None,
                cache: None,
            });

        Self {
            render_pipeline,
            bindings,
        }
    }

    pub fn enable(&self, render_pass: &mut RenderPass<'_>) {
        render_pass.set_pipeline(&self.render_pipeline);
        for (i, binding) in self.bindings.iter().enumerate() {
            render_pass.set_bind_group(i as u32, binding.group(), &[]);
        }
    }
}

pub struct PipelineBuilder<'g, 'a> {
    gpu: &'g Gpu,
    name: String,
    bindings: Vec<Binding>,
    shader: Option<ShaderModuleDescriptor<'a>>,
    buffers: Vec<VertexBufferLayout<'a>>,
    format: TextureFormat,
    depth_stencil: Option<DepthStencilState>,
}

impl<'a> PipelineBuilder<'_, 'a> {
    pub fn with_binding(mut self, binding: Binding) -> Self {
        self.bindings.push(binding);
        self
    }

    pub fn build_binding(self, name: impl Into<String>, f: impl FnOnce(BindingBuilder) -> Binding) -> Self {
        let binding = f(self.gpu.build_binding(name));
        self.with_binding(binding)
    }

    pub fn with_shader(mut self, shader: ShaderModuleDescriptor<'a>) -> Self {
        self.shader = Some(shader);
        self
    }

    pub fn with_buffers(mut self, buffers: impl IntoIterator<Item=VertexBufferLayout<'a>>) -> Self {
        self.buffers.extend(buffers);
        self
    }

    pub fn with_buffer(mut self, buffer: VertexBufferLayout<'a>) -> Self {
        self.buffers.push(buffer);
        self
    }

    pub fn with_depth_stencil(mut self, depth_stencil: DepthStencilState) -> Self {
        self.depth_stencil = Some(depth_stencil);
        self
    }

    pub fn finish(self) -> Pipeline {
        Pipeline::create(
            self.gpu,
            &self.name,
            self.bindings,
            self.shader.expect(&format!("{} did not set a shader when building its pipeline", self.name)),
            &self.buffers,
            self.format,
            self.depth_stencil,
        )
    }
}

impl Gpu {
    pub fn build_pipeline(&self, name: impl Into<String>, format: TextureFormat) -> PipelineBuilder {
        PipelineBuilder {
            gpu: self,
            name: name.into(),
            bindings: vec![],
            shader: None,
            buffers: vec![],
            format,
            depth_stencil: None,
        }
    }
}