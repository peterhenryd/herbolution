use std::num::NonZeroU32;
use std::sync::Arc;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType, Device, RenderPass,
    Sampler, SamplerBindingType, ShaderStages, TextureSampleType, TextureView,
    TextureViewDimension,
};
use crate::gpu::buffer::Buffer;
use crate::gpu::Gpu;

pub struct Binding {
    layout: BindGroupLayout,
    group: BindGroup,
}

impl Binding {
    pub fn bind(&self, slot: u32, render_pass: &mut RenderPass) {
        render_pass.set_bind_group(slot, &self.group, &[]);
    }

    pub fn layout(&self) -> &BindGroupLayout {
        &self.layout
    }
}

pub struct BindingBuilder<'a> {
    device: Arc<Device>,
    name: String,
    layout_entries: Vec<LayoutEntry>,
    resource_entries: Vec<BindingResource<'a>>,
}

impl BindingBuilder<'_> {
    pub fn new(device: Arc<Device>, name: String) -> Self {
        Self {
            device,
            name,
            layout_entries: vec![],
            resource_entries: vec![],
        }
    }
}

struct LayoutEntry {
    visibility: ShaderStages,
    ty: BindingType,
    count: Option<NonZeroU32>,
}

impl<'a> BindingBuilder<'a> {
    pub fn with_entry(
        mut self,
        visibility: ShaderStages,
        ty: BindingType,
        count: Option<NonZeroU32>,
        resource: BindingResource<'a>,
    ) -> Self {
        self.layout_entries.push(LayoutEntry {
            visibility,
            ty,
            count,
        });
        self.resource_entries.push(resource);
        self
    }

    pub fn with_sampler(self, binding_type: SamplerBindingType, sampler: &'a Sampler) -> Self {
        self.with_entry(
            ShaderStages::FRAGMENT,
            BindingType::Sampler(binding_type),
            None,
            BindingResource::Sampler(sampler),
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
            Some(NonZeroU32::new(views.len() as u32).unwrap()),
            BindingResource::TextureViewArray(views),
        )
    }

    pub fn with_uniform_buffer(self, buffer: &'a impl Buffer) -> Self {
        self.with_entry(
            buffer.visibility(),
            BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            None,
            buffer.as_ref().as_entire_binding(),
        )
    }

    pub fn finish(self) -> Binding {
        let layout_entries = self
            .layout_entries
            .into_iter()
            .enumerate()
            .map(
                |(
                     i,
                     LayoutEntry {
                         visibility,
                         ty,
                         count,
                     },
                 )| BindGroupLayoutEntry {
                    binding: i as u32,
                    visibility,
                    ty,
                    count,
                },
            )
            .collect::<Vec<_>>();
        let layout = self
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some(&format!("herbolution_{}_bind_group_layout", self.name)),
                entries: &layout_entries,
            });

        let entries = self
            .resource_entries
            .into_iter()
            .enumerate()
            .map(|(i, resource)| BindGroupEntry {
                binding: i as u32,
                resource,
            })
            .collect::<Vec<_>>();
        let group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some(&format!("herbolution_{}_bind_group", self.name)),
            layout: &layout,
            entries: &entries,
        });

        Binding { layout, group }
    }
}

impl Gpu {
    pub fn build_binding(&self, name: impl Into<String>) -> BindingBuilder {
        BindingBuilder::new(self.device.clone(), name.into())
    }
}
