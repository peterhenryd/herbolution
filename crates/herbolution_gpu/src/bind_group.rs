use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType,
    BufferUsages, SamplerBindingType, ShaderStages, TextureSampleType, TextureViewDimension,
};

use crate::buffer::Buffer;
use crate::handle::Handle;
use crate::texture::Texture;

#[derive(Debug)]
pub struct BindGroup {
    pub inner: wgpu::BindGroup,
    pub layout: BindGroupLayout,
}

impl BindGroup {
    #[inline]
    pub fn build<'r>() -> BindGroupBuilder<'r> {
        BindGroupBuilder::new()
    }
}

pub struct BindGroupBuilder<'r> {
    pairs: Vec<(BindGroupLayoutEntry, BindGroupEntry<'r>)>,
}

impl<'r> BindGroupBuilder<'r> {
    #[inline]
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    #[inline]
    pub fn add_item(&mut self, layout_entry: BindGroupLayoutEntry, entry: BindGroupEntry<'r>) {
        self.pairs.push((layout_entry, entry));
    }

    #[inline]
    pub fn with_item(mut self, layout_entry: BindGroupLayoutEntry, entry: BindGroupEntry<'r>) -> Self {
        self.add_item(layout_entry, entry);
        self
    }

    pub fn add_buffer<T>(&mut self, buffer: &'r Buffer<T>, visibility: ShaderStages) {
        let binding = self.pairs.len() as u32;
        self.pairs.push((
            BindGroupLayoutEntry {
                binding,
                visibility,
                ty: BindingType::Buffer {
                    ty: if buffer.usage().contains(BufferUsages::UNIFORM) {
                        BufferBindingType::Uniform
                    } else {
                        BufferBindingType::Storage { read_only: true }
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            BindGroupEntry {
                binding,
                resource: buffer.inner().as_entire_binding(),
            },
        ));
    }

    pub fn with_buffer<T>(mut self, buffer: &'r Buffer<T>, visibility: ShaderStages) -> Self {
        self.add_buffer(buffer, visibility);
        self
    }

    pub fn add_sampler(&mut self, sampler: &'r wgpu::Sampler) {
        let binding = self.pairs.len() as u32;
        self.pairs.push((
            BindGroupLayoutEntry {
                binding,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
            BindGroupEntry {
                binding,
                resource: BindingResource::Sampler(sampler),
            },
        ));
    }

    pub fn with_sampler(mut self, sampler: &'r wgpu::Sampler) -> Self {
        self.add_sampler(sampler);
        self
    }

    pub fn add_texture(&mut self, texture: &'r Texture) {
        let binding = self.pairs.len() as u32;
        self.pairs.push((
            BindGroupLayoutEntry {
                binding,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupEntry {
                binding,
                resource: BindingResource::TextureView(texture.view()),
            },
        ));
    }

    pub fn with_texture(mut self, texture: &'r Texture) -> Self {
        self.add_texture(texture);
        self
    }

    pub fn finish(self, gpu: &Handle) -> BindGroup {
        let (layout_entries, entries): (Vec<_>, Vec<_>) = self.pairs.into_iter().unzip();

        let layout = gpu
            .device()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &layout_entries,
            });
        let inner = gpu
            .device()
            .create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &entries,
            });

        BindGroup { inner, layout }
    }
}
