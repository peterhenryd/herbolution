pub mod binding;
pub mod buffer;
pub mod pipeline;
pub mod surface;
pub mod texture;
pub mod uniform;

use std::sync::Arc;
use thiserror::Error;
use wgpu::{
    Adapter, AddressMode, BindGroupLayout, BlendState, ColorTargetState, ColorWrites,
    CommandEncoder, Device, DeviceDescriptor, Face, Features, FilterMode, FragmentState, FrontFace,
    Instance, Limits, MemoryHints, MultisampleState, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PolygonMode, PowerPreference, PrimitiveState, PrimitiveTopology,
    Queue, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, RequestDeviceError,
    Sampler, SamplerDescriptor, ShaderModuleDescriptor, Surface, TextureFormat, VertexBufferLayout,
    VertexState,
};

#[derive(Debug, Clone)]
pub struct Gpu {
    pub adapter: Arc<Adapter>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

#[derive(Debug, Error)]
pub enum CreateGpuError {
    #[error("Failed to find an appropriate GPU adapter")]
    RequestAdapter,
    #[error("Failed to create a GPU device: {0}")]
    RequestDevice(#[from] RequestDeviceError),
}

impl Gpu {
    pub async fn create(instance: &Instance, surface: &Surface<'_>) -> Result<Self, CreateGpuError> {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .await
            .ok_or(CreateGpuError::RequestAdapter)?;
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("herbolution_device"),
                    required_features: Features::TEXTURE_BINDING_ARRAY,
                    required_limits: Limits::default(),
                    memory_hints: MemoryHints::Performance,
                },
                None,
            )
            .await?;

        Ok(Self {
            adapter: Arc::new(adapter),
            device: Arc::new(device),
            queue: Arc::new(queue),
        })
    }

    pub fn create_render_pipeline(
        &self,
        name: &str,
        bind_group_layouts: &[&BindGroupLayout],
        shader_module_descriptor: ShaderModuleDescriptor,
        buffers: &[VertexBufferLayout<'static>],
        format: TextureFormat,
    ) -> RenderPipeline {
        let pipeline_layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some(&format!("herbolution_{name}_render_pipeline_layout")),
                bind_group_layouts,
                push_constant_ranges: &[],
            });
        let shader_module = self.device.create_shader_module(shader_module_descriptor);

        self.device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(&format!("herbolution_{name}_render_pipeline")),
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
                    cull_mode: Some(Face::Back),
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
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
            })
    }

    pub fn create_command_encoder(&self, name: &str) -> CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(&format!("herbolution_{name}_command_encoder")),
            })
    }

    pub fn create_sampler(&self, filter_mode: FilterMode) -> Sampler {
        self.device.create_sampler(&SamplerDescriptor {
            label: None,
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: filter_mode,
            min_filter: FilterMode::Linear,
            mipmap_filter: filter_mode,
            ..Default::default()
        })
    }
}
