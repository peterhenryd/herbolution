use std::borrow::Cow;
use std::marker::PhantomData;
use std::num::NonZeroU32;

use crate::{Buffer, Gpu, Texture};

pub struct PipelineLayoutBuilder<'a> {
    gpu: &'a Gpu,
    label: Option<Cow<'a, str>>,
    bind_group_layouts: Vec<&'a wgpu::BindGroupLayout>,
    push_constant_ranges: Vec<wgpu::PushConstantRange>,
}

impl<'a> PipelineLayoutBuilder<'a> {
    #[inline]
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[inline]
    pub fn add_bind_group_layout(mut self, layout: &'a wgpu::BindGroupLayout) -> Self {
        self.bind_group_layouts.push(layout);
        self
    }

    #[inline]
    pub fn add_push_constant_range(mut self, range: wgpu::PushConstantRange) -> Self {
        self.push_constant_ranges.push(range);
        self
    }

    #[inline]
    pub fn create(self) -> wgpu::PipelineLayout {
        self.gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: self.label.as_deref(),
                bind_group_layouts: &self.bind_group_layouts,
                push_constant_ranges: &self.push_constant_ranges,
            })
    }
}

impl Gpu {
    #[inline]
    pub fn build_pipeline_layout(&self) -> PipelineLayoutBuilder<'_> {
        PipelineLayoutBuilder {
            gpu: self,
            label: None,
            bind_group_layouts: Vec::new(),
            push_constant_ranges: Vec::new(),
        }
    }
}

pub struct RenderPipelineBuilder<'a> {
    gpu: &'a Gpu,
    label: Option<Cow<'a, str>>,
    layout: Option<&'a wgpu::PipelineLayout>,
    vertex_module: Option<&'a wgpu::ShaderModule>,
    vertex_entry_point: Option<Cow<'a, str>>,
    vertex_buffers: Vec<wgpu::VertexBufferLayout<'a>>,
    primitive: wgpu::PrimitiveState,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    fragment_module: Option<&'a wgpu::ShaderModule>,
    fragment_entry_point: Option<Cow<'a, str>>,
    color_targets: Vec<Option<wgpu::ColorTargetState>>,
    multiview: Option<NonZeroU32>,
    compilation_options: wgpu::PipelineCompilationOptions<'a>,
}

impl<'a> RenderPipelineBuilder<'a> {
    #[inline]
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[inline]
    pub fn layout(mut self, layout: &'a wgpu::PipelineLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    #[inline]
    pub fn vertex_shader(mut self, module: &'a wgpu::ShaderModule, entry_point: impl Into<Cow<'a, str>>) -> Self {
        self.vertex_module = Some(module);
        self.vertex_entry_point = Some(entry_point.into());
        self
    }

    #[inline]
    pub fn add_vertex_buffer_layout(mut self, layout: wgpu::VertexBufferLayout<'a>) -> Self {
        self.vertex_buffers.push(layout);
        self
    }

    #[inline]
    pub fn topology(mut self, topology: wgpu::PrimitiveTopology) -> Self {
        self.primitive.topology = topology;
        self
    }

    #[inline]
    pub fn cull_mode(mut self, face: Option<wgpu::Face>) -> Self {
        self.primitive.cull_mode = face;
        self
    }

    #[inline]
    pub fn polygon_mode(mut self, polygon_mode: wgpu::PolygonMode) -> Self {
        self.primitive.polygon_mode = polygon_mode;
        self
    }

    #[inline]
    pub fn depth_stencil(mut self, state: wgpu::DepthStencilState) -> Self {
        self.depth_stencil = Some(state);
        self
    }

    #[inline]
    pub fn multisample_count(mut self, count: u32) -> Self {
        self.multisample.count = count;
        self
    }

    #[inline]
    pub fn alpha_to_coverage_enabled(mut self, enabled: bool) -> Self {
        self.multisample.alpha_to_coverage_enabled = enabled;
        self
    }

    #[inline]
    pub fn fragment_shader(mut self, module: &'a wgpu::ShaderModule, entry_point: impl Into<Cow<'a, str>>) -> Self {
        self.fragment_module = Some(module);
        self.fragment_entry_point = Some(entry_point.into());
        self
    }

    #[inline]
    pub fn add_color_target(mut self, target: impl Into<Option<wgpu::ColorTargetState>>) -> Self {
        self.color_targets.push(target.into());
        self
    }

    #[inline]
    pub fn multiview(mut self, view_count: NonZeroU32) -> Self {
        self.multiview = Some(view_count);
        self
    }

    #[inline]
    pub fn shader(self, module: &'a wgpu::ShaderModule, vertex_entry_point: impl Into<Cow<'a, str>>, fragment_entry_point: impl Into<Cow<'a, str>>) -> Self {
        self.vertex_shader(module, vertex_entry_point)
            .fragment_shader(module, fragment_entry_point)
    }

    #[inline]
    pub fn create(self) -> wgpu::RenderPipeline {
        let vertex_state = wgpu::VertexState {
            module: self
                .vertex_module
                .expect("A vertex shader module must be provided."),
            entry_point: self.vertex_entry_point.as_deref(),
            compilation_options: self.compilation_options.clone(),
            buffers: &self.vertex_buffers,
        };

        let fragment_state = self
            .fragment_module
            .map(|module| wgpu::FragmentState {
                module,
                entry_point: self.fragment_entry_point.as_deref(),
                compilation_options: self.compilation_options.clone(),
                targets: &self.color_targets,
            });

        self.gpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: self.label.as_deref(),
                layout: self.layout,
                vertex: vertex_state,
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                fragment: fragment_state,
                multiview: self.multiview,
                cache: None,
            })
    }
}

impl Gpu {
    #[inline]
    pub fn build_render_pipeline(&self) -> RenderPipelineBuilder<'_> {
        RenderPipelineBuilder {
            gpu: self,
            label: None,
            layout: None,
            vertex_module: None,
            vertex_entry_point: None,
            vertex_buffers: Vec::new(),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment_module: None,
            fragment_entry_point: None,
            color_targets: Vec::new(),
            multiview: None,
            compilation_options: Default::default(),
        }
    }
}

pub struct BindGroupBuilder<'gpu, 'temp, 'out> {
    gpu: &'gpu Gpu,
    label: Option<Cow<'temp, str>>,
    entries: Vec<BindGroupBuilderEntry<'temp>>,
    layout: Option<&'out wgpu::BindGroupLayout>,
}

impl<'gpu, 'out> BindGroupBuilder<'gpu, '_, 'out> {
    #[inline]
    pub fn layout(mut self, layout: &'out wgpu::BindGroupLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    #[inline]
    pub fn create_with_layout(self) -> (wgpu::BindGroup, Cow<'out, wgpu::BindGroupLayout>) {
        let bind_group_layout = match self.layout {
            None => {
                let layout_entries = self
                    .entries
                    .iter()
                    .enumerate()
                    .map(|(i, entry)| wgpu::BindGroupLayoutEntry {
                        binding: i as u32,
                        visibility: entry.visibility,
                        ty: entry.binding_type,
                        count: entry.count,
                    })
                    .collect::<Vec<_>>();

                Cow::Owned(
                    self.gpu
                        .device
                        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                            label: None,
                            entries: &layout_entries,
                        }),
                )
            }
            Some(x) => Cow::Borrowed(x),
        };

        let entries = self
            .entries
            .into_iter()
            .enumerate()
            .map(|(i, entry)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: entry.resource,
            })
            .collect::<Vec<_>>();

        let bind_group = self
            .gpu
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: self.label.as_deref(),
                layout: bind_group_layout.as_ref(),
                entries: &entries,
            });

        (bind_group, bind_group_layout)
    }

    #[inline]
    pub fn create(self) -> wgpu::BindGroup {
        self.create_with_layout().0
    }
}

impl<'temp> BindGroupBuilder<'_, 'temp, '_> {
    #[inline]
    pub fn label(mut self, label: impl Into<Cow<'temp, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[inline]
    pub fn add_entry(mut self, entry: impl Into<BindGroupBuilderEntry<'temp>>) -> Self {
        self.entries.push(entry.into());
        self
    }
}

impl Gpu {
    #[inline]
    pub fn build_bind_group(&self) -> BindGroupBuilder {
        BindGroupBuilder {
            gpu: self,
            label: None,
            entries: vec![],
            layout: None,
        }
    }
}

struct BindGroupBuilderEntry<'a> {
    visibility: wgpu::ShaderStages,
    binding_type: wgpu::BindingType,
    count: Option<NonZeroU32>,
    resource: wgpu::BindingResource<'a>,
}

pub trait ShaderStagesBuilderExt: Sized {
    fn visibility(self, visibility: wgpu::ShaderStages) -> Self;

    #[inline]
    fn use_vert(self) -> Self {
        self.visibility(wgpu::ShaderStages::VERTEX)
    }

    #[inline]
    fn use_frag(self) -> Self {
        self.visibility(wgpu::ShaderStages::FRAGMENT)
    }

    #[inline]
    fn use_comp(self) -> Self {
        self.visibility(wgpu::ShaderStages::COMPUTE)
    }

    #[inline]
    fn use_task(self) -> Self {
        self.visibility(wgpu::ShaderStages::TASK)
    }

    #[inline]
    fn use_mesh(self) -> Self {
        self.visibility(wgpu::ShaderStages::MESH)
    }
}

pub trait ToBinding<'a> {
    type Output: Into<BindGroupBuilderEntry<'a>>;

    fn to_binding(&'a self) -> Self::Output;
}

pub struct BufferBindingBuilder<'a, T> {
    resource: wgpu::BindingResource<'a>,
    binding_type: wgpu::BufferBindingType,
    visibility: wgpu::ShaderStages,
    count: Option<NonZeroU32>,
    _marker: PhantomData<T>,
}

impl<'a, T> ToBinding<'a> for Buffer<T> {
    type Output = BufferBindingBuilder<'a, T>;

    #[inline]
    fn to_binding(&'a self) -> Self::Output {
        BufferBindingBuilder {
            resource: self.inner().as_entire_binding(),
            binding_type: wgpu::BufferBindingType::Uniform,
            visibility: wgpu::ShaderStages::empty(),
            count: None,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> BufferBindingBuilder<'a, T> {
    #[inline]
    pub fn uniform(mut self) -> Self {
        self.binding_type = wgpu::BufferBindingType::Uniform;
        self
    }

    #[inline]
    pub fn storage(mut self) -> Self {
        self.binding_type = wgpu::BufferBindingType::Storage { read_only: true };
        self
    }

    #[inline]
    pub fn mut_storage(mut self) -> Self {
        self.binding_type = wgpu::BufferBindingType::Storage { read_only: false };
        self
    }
}

impl<'a, T> ShaderStagesBuilderExt for BufferBindingBuilder<'a, T> {
    #[inline]
    fn visibility(mut self, visibility: wgpu::ShaderStages) -> Self {
        self.visibility.insert(visibility);
        self
    }
}

impl<'a, T> From<BufferBindingBuilder<'a, T>> for BindGroupBuilderEntry<'a> {
    fn from(builder: BufferBindingBuilder<'a, T>) -> Self {
        BindGroupBuilderEntry {
            visibility: builder.visibility,
            binding_type: wgpu::BindingType::Buffer {
                ty: builder.binding_type,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: builder.count,
            resource: builder.resource,
        }
    }
}

pub trait DepthStencilStateExt {
    fn f32_less() -> Self;
}

impl DepthStencilStateExt for wgpu::DepthStencilState {
    fn f32_less() -> Self {
        Self {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: Default::default(),
            bias: Default::default(),
        }
    }
}

pub trait ColorTargetStateExt {
    fn bgra_all() -> Self;
}

impl ColorTargetStateExt for wgpu::ColorTargetState {
    fn bgra_all() -> Self {
        Self {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        }
    }
}

pub struct SamplerBindingBuilder<'a> {
    resource: wgpu::BindingResource<'a>,
    binding_type: wgpu::SamplerBindingType,
    visibility: wgpu::ShaderStages,
}

impl<'a> SamplerBindingBuilder<'a> {}

impl ShaderStagesBuilderExt for SamplerBindingBuilder<'_> {
    fn visibility(mut self, visibility: wgpu::ShaderStages) -> Self {
        self.visibility.insert(visibility);
        self
    }
}

impl<'a> ToBinding<'a> for wgpu::Sampler {
    type Output = SamplerBindingBuilder<'a>;

    fn to_binding(&'a self) -> Self::Output {
        SamplerBindingBuilder {
            resource: wgpu::BindingResource::Sampler(self),
            binding_type: wgpu::SamplerBindingType::Filtering,
            visibility: wgpu::ShaderStages::empty(),
        }
    }
}

impl<'a> From<SamplerBindingBuilder<'a>> for BindGroupBuilderEntry<'a> {
    fn from(value: SamplerBindingBuilder<'a>) -> Self {
        Self {
            visibility: value.visibility,
            binding_type: wgpu::BindingType::Sampler(value.binding_type),
            count: None,
            resource: value.resource,
        }
    }
}

pub struct TextureBindingBuilder<'a> {
    resource: wgpu::BindingResource<'a>,
    sample_type: wgpu::TextureSampleType,
    view_dimension: wgpu::TextureViewDimension,
    multisampled: bool,
    visibility: wgpu::ShaderStages,
    count: Option<NonZeroU32>,
}

impl<'a> ToBinding<'a> for Texture {
    type Output = TextureBindingBuilder<'a>;

    #[inline]
    fn to_binding(&'a self) -> Self::Output {
        TextureBindingBuilder {
            resource: wgpu::BindingResource::TextureView(&self.view),
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: match self.view.texture().dimension() {
                wgpu::TextureDimension::D1 => wgpu::TextureViewDimension::D1,
                wgpu::TextureDimension::D2 => wgpu::TextureViewDimension::D2,
                wgpu::TextureDimension::D3 => wgpu::TextureViewDimension::D3,
            },
            multisampled: false,
            visibility: wgpu::ShaderStages::empty(),
            count: None,
        }
    }
}

impl<'a> TextureBindingBuilder<'a> {
    #[inline]
    pub fn with_sample_type(mut self, sample_type: wgpu::TextureSampleType) -> Self {
        self.sample_type = sample_type;
        self
    }

    #[inline]
    pub fn with_view_dimension(mut self, dimension: wgpu::TextureViewDimension) -> Self {
        self.view_dimension = dimension;
        self
    }
}

impl<'a> ShaderStagesBuilderExt for TextureBindingBuilder<'a> {
    #[inline]
    fn visibility(mut self, visibility: wgpu::ShaderStages) -> Self {
        self.visibility.insert(visibility);
        self
    }
}

impl<'a> From<TextureBindingBuilder<'a>> for BindGroupBuilderEntry<'a> {
    fn from(builder: TextureBindingBuilder<'a>) -> Self {
        BindGroupBuilderEntry {
            visibility: builder.visibility,
            binding_type: wgpu::BindingType::Texture {
                sample_type: builder.sample_type,
                view_dimension: builder.view_dimension,
                multisampled: builder.multisampled,
            },
            count: builder.count,
            resource: builder.resource,
        }
    }
}

pub struct BindGroupLayoutEntryBuilder {
    entry: wgpu::BindGroupLayoutEntry,
}

impl BindGroupLayoutEntryBuilder {
    pub fn buffer(binding: u32, visibility: wgpu::ShaderStages) -> Self {
        Self {
            entry: wgpu::BindGroupLayoutEntry {
                binding,
                visibility,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        }
    }

    pub fn texture(binding: u32, visibility: wgpu::ShaderStages) -> Self {
        Self {
            entry: wgpu::BindGroupLayoutEntry {
                binding,
                visibility,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
        }
    }

    pub fn sampler(binding: u32, visibility: wgpu::ShaderStages) -> Self {
        Self {
            entry: wgpu::BindGroupLayoutEntry {
                binding,
                visibility,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        }
    }

    pub fn storage_texture(binding: u32, visibility: wgpu::ShaderStages, format: wgpu::TextureFormat) -> Self {
        Self {
            entry: wgpu::BindGroupLayoutEntry {
                binding,
                visibility,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::WriteOnly,
                    format,
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            },
        }
    }

    #[inline]
    pub fn build(self) -> wgpu::BindGroupLayoutEntry {
        self.entry
    }

    #[inline]
    pub fn with_buffer_type(mut self, ty: wgpu::BufferBindingType) -> Self {
        if let wgpu::BindingType::Buffer { ty: buffer_ty, .. } = &mut self.entry.ty {
            *buffer_ty = ty;
        }
        self
    }

    #[inline]
    pub fn with_dynamic_offset(mut self, has_dynamic_offset: bool) -> Self {
        if let wgpu::BindingType::Buffer { has_dynamic_offset: d, .. } = &mut self.entry.ty {
            *d = has_dynamic_offset;
        }
        self
    }
}

pub struct BindGroupLayoutBuilder<'a> {
    gpu: &'a Gpu,
    label: Option<Cow<'a, str>>,
    entries: Vec<wgpu::BindGroupLayoutEntry>,
}

impl<'a> BindGroupLayoutBuilder<'a> {
    #[inline]
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[inline]
    pub fn add_entry(mut self, entry: wgpu::BindGroupLayoutEntry) -> Self {
        self.entries.push(entry);
        self
    }

    #[inline]
    pub fn create(self) -> wgpu::BindGroupLayout {
        self.gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: self.label.as_deref(),
                entries: &self.entries,
            })
    }
}

impl Gpu {
    #[inline]
    pub fn build_bind_group_layout(&self) -> BindGroupLayoutBuilder<'_> {
        BindGroupLayoutBuilder {
            gpu: self,
            label: None,
            entries: Vec::new(),
        }
    }
}
