use pollster::FutureExt;
use wgpu::*;
use crate::gpu::GpuError;
use crate::gpu::mem::bind_group::BindGroupSet;

#[derive(Debug)]
pub struct Handle {
    pub(crate) adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

impl Handle {
    const FEATURES: Features = Features::TEXTURE_BINDING_ARRAY;

    pub fn create(instance: &Instance, surface: &Surface) -> Result<Self, GpuError> {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .block_on()
            .ok_or(GpuError::RequestAdapter)?;
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Herbolution GPU Device"),
                    required_features: Self::FEATURES,
                    required_limits: Limits::default(),
                    memory_hints: MemoryHints::MemoryUsage,
                },
                None,
            )
            .block_on()?;

        Ok(Self {
            adapter,
            device,
            queue,
        })
    }

    pub fn create_render_pipeline(&self, bind_group_set: &BindGroupSet, shader: ShaderModuleDescriptor, vertex_buffers: &[VertexBufferLayout], format: TextureFormat) -> RenderPipeline {
        let bind_group_layouts = bind_group_set.layouts().collect::<Vec<_>>();
        let layout = self.device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &bind_group_layouts,
                push_constant_ranges: &[],
            });
        let shader_module = self.device.create_shader_module(shader);

        self.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vs"),
                compilation_options: Default::default(),
                buffers: vertex_buffers,
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
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some("fs"),
                compilation_options: Default::default(),
                targets: &[
                    Some(ColorTargetState {
                        format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })
                ],
            }),
            multiview: None,
            cache: None,
        })
    }
}