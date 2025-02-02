use std::sync::Arc;
use wgpu::{
    BlendState, ColorTargetState, ColorWrites, DepthBiasState, Device, FragmentState,
    PipelineCompilationOptions, PipelineLayoutDescriptor, RenderPass, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, StencilState, TextureFormat,
    VertexBufferLayout, VertexState,
};
use crate::gpu::binding::BindingBuilder;
use crate::gpu::Gpu;
use crate::gpu::texture::DepthTexture;

pub struct Pipeline {
    render_pipeline: RenderPipeline,
    bindings: Vec<Binding>,
}

impl Pipeline {
    pub fn bind(&self, render_pass: &mut RenderPass<'_>) {
        render_pass.set_pipeline(&self.render_pipeline);
        for (i, binding) in self.bindings.iter().enumerate() {
            binding.bind(i as u32, render_pass);
        }
    }
}

pub struct PipelineBuilder<'a> {
    device: Arc<Device>,
    name: String,
    bindings: Vec<Binding>,
    shader_module_descriptor: Option<ShaderModuleDescriptor<'a>>,
    vertex_buffer_layouts: Vec<VertexBufferLayout<'a>>,
    format: TextureFormat,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(device: Arc<Device>, name: impl Into<String>, format: TextureFormat) -> Self {
        Self {
            name: name.into(),
            bindings: Vec::new(),
            shader_module_descriptor: None,
            vertex_buffer_layouts: vec![],
            format,
            device,
        }
    }

    pub fn with_shader(mut self, descriptor: ShaderModuleDescriptor<'a>) -> Self {
        self.shader_module_descriptor = Some(descriptor);
        self
    }

    pub fn with_vertex_buffer_layout(mut self, layout: VertexBufferLayout<'a>) -> Self {
        self.vertex_buffer_layouts.push(layout);
        self
    }

    pub fn finish(self) -> Pipeline {
        assert!(
            self.shader_module_descriptor.is_some(),
            "Pipeline shader module must be set"
        );

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some(&format!("herbolution_{}_render_pipeline_layout", self.name)),
                bind_group_layouts: &self
                    .bindings
                    .iter()
                    .map(Binding::layout)
                    .collect::<Vec<_>>(),
                push_constant_ranges: &[],
            });
        let shader_module = self
            .device
            .create_shader_module(self.shader_module_descriptor.unwrap());
        let render_pipeline = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(&format!("herbolution_{}_render_pipeline", self.name)),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader_module,
                    entry_point: Some("vs"),
                    compilation_options: PipelineCompilationOptions::default(),
                    buffers: &self.vertex_buffer_layouts,
                },
                primitive: Default::default(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DepthTexture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less, // 1.
                    stencil: StencilState::default(),           // 2.
                    bias: DepthBiasState::default(),
                }),
                multisample: Default::default(),
                fragment: Some(FragmentState {
                    module: &shader_module,
                    entry_point: Some("fs"),
                    compilation_options: PipelineCompilationOptions::default(),
                    targets: &[Some(ColorTargetState {
                        format: self.format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::all(),
                    })],
                }),
                multiview: None,
                cache: None,
            });

        Pipeline {
            render_pipeline,
            bindings: self.bindings,
        }
    }

    pub fn with_binding(mut self, binding: Binding) -> Self {
        self.bindings.push(binding);
        self
    }

    pub fn build_binding(
        mut self,
        name: impl Into<String>,
        f: impl FnOnce(BindingBuilder<'a>) -> BindingBuilder<'a>,
    ) -> Self {
        self.bindings
            .push(f(BindingBuilder::new(self.device.clone(), name.into())).finish());
        self
    }
}

impl Gpu {
    pub fn build_pipeline(
        &self,
        name: impl Into<String>,
        format: TextureFormat,
    ) -> PipelineBuilder {
        PipelineBuilder::new(self.device.clone(), name, format)
    }
}
