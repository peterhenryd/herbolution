use std::sync::Arc;
use wgpu::{BindGroup, BindGroupLayout};

pub use builder::BindingBuilder;

#[derive(Debug, Clone)]
pub struct Binding {
    layout: Arc<BindGroupLayout>,
    group: Arc<BindGroup>,
}

impl Binding {
    pub fn layout(&self) -> &BindGroupLayout {
        &self.layout
    }

    pub fn group(&self) -> &BindGroup {
        &self.group
    }
}

impl crate::engine::gpu::Gpu {
    pub fn build_binding<'a>(&self, name: impl Into<String>) -> BindingBuilder<'a> {
        BindingBuilder {
            gpu: self.clone(),
            name: name.into(),
            entries: vec![],
            layout: None,
        }
    }
}

mod builder {
    use crate::engine::binding::Binding;
    use crate::engine::gpu::Gpu;
    use crate::engine::storage::Storage;
    use crate::engine::uniform::Uniform;
    use std::num::NonZeroU32;
    use std::sync::Arc;
    use wgpu::{BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, Sampler, SamplerBindingType, ShaderStages, TextureSampleType, TextureView, TextureViewDimension};

    pub struct BindingBuilder<'a> {
        pub(super) gpu: Gpu,
        pub(super) name: String,
        pub(super) entries: Vec<BindingEntry<'a>>,
        pub(super) layout: Option<BindGroupLayout>,
    }

    pub struct BindingEntry<'a> {
        visibility: ShaderStages,
        ty: BindingType,
        count: Option<NonZeroU32>,
        resource: BindingResource<'a>,
    }

    impl<'a> BindingBuilder<'a> {
        pub fn with_entry(
            mut self,
            visibility: ShaderStages,
            ty: BindingType,
            count: Option<NonZeroU32>,
            resource: BindingResource<'a>,
        ) -> Self {
            self.entries.push(BindingEntry { visibility, ty, count, resource });
            self
        }

        pub fn with_storage<T>(self, visibility: ShaderStages, buffer: &'a Storage<T>) -> Self {
            self.with_entry(
                visibility,
                BindingType::Buffer {
                    ty: BufferBindingType::Storage {
                        read_only: true,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                None,
                buffer.buffer.as_entire_binding(),
            )
        }

        pub fn with_uniform<T>(self, visibility: ShaderStages, buffer: &'a Uniform<T>) -> Self {
            self.with_entry(
                visibility,
                BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                None,
                buffer.buffer.as_entire_binding(),
            )
        }

        pub fn with_layout(mut self, layout: BindGroupLayout) -> Self {
            self.layout = Some(layout);
            self
        }

        pub fn with_sampler(self, sampler_type: SamplerBindingType, sampler: &'a Sampler) -> Self {
            self.with_entry(
                ShaderStages::FRAGMENT,
                BindingType::Sampler(sampler_type),
                None,
                BindingResource::Sampler(sampler),
            )
        }

        pub fn with_texture(self, view: &'a TextureView) -> Self {
            self.with_entry(
                ShaderStages::FRAGMENT,
                BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                None,
                BindingResource::TextureView(view),
            )
        }

        pub fn with_texture_array(self, views: &'a [&'a TextureView]) -> Self {
            self.with_entry(
                ShaderStages::FRAGMENT,
                BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2Array,
                    multisampled: false,
                },
                NonZeroU32::new(views.len() as u32),
                BindingResource::TextureViewArray(views),
            )
        }

        pub fn finish(self) -> Binding {
            let layout;
            if let Some(x) = self.layout {
                layout = x;
            } else {
                let layout_entries = self.entries.iter()
                    .enumerate()
                    .map(|(i, BindingEntry { visibility, ty, count, .. })| BindGroupLayoutEntry {
                        binding: i as u32,
                        visibility: *visibility,
                        ty: *ty,
                        count: *count,
                    })
                    .collect::<Vec<_>>();
                layout = self.gpu.device
                    .create_bind_group_layout(&BindGroupLayoutDescriptor {
                        label: Some(&format!("herbolution_{}_bind_group_layout", self.name)),
                        entries: &layout_entries,
                    });
            }

            let group_entries = self.entries.into_iter()
                .enumerate()
                .map(|(i, BindingEntry { resource, .. })| BindGroupEntry { binding: i as u32, resource })
                .collect::<Vec<_>>();
            let group = self.gpu.device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some(&format!("herbolution_{}_bind_group", self.name)),
                    layout: &layout,
                    entries: &group_entries,
                });

            Binding { layout: Arc::new(layout), group: Arc::new(group) }
        }
    }
}