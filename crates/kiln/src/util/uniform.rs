use std::slice;

use bytemuck::Pod;

use crate::*;

#[derive(Debug)]
pub struct Uniform<T> {
    buffer: Buffer<T>,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl<T> Uniform<T> {
    pub fn write(&mut self, gpu: &Gpu, data: &T)
    where
        T: Pod,
    {
        self.buffer.write(gpu, slice::from_ref(data));
    }

    pub fn buffer(&self) -> &Buffer<T> {
        &self.buffer
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}

pub struct UniformBuilder<'gpu, 'a, T> {
    gpu: &'gpu Gpu,
    usage: BufferUsages,
    visibility: wgpu::ShaderStages,
    data: &'a T,
}

impl<T> BufferUsagesBuilderExt for UniformBuilder<'_, '_, T> {
    fn usage(mut self, usage: BufferUsages) -> Self {
        self.usage.insert(usage);
        self
    }
}

impl<T> ShaderStagesBuilderExt for UniformBuilder<'_, '_, T> {
    fn visibility(mut self, visibility: wgpu::ShaderStages) -> Self {
        self.visibility.insert(visibility);
        self
    }
}

impl<'gpu, 'a, T> UniformBuilder<'gpu, 'a, T> {
    pub fn create(self) -> Uniform<T>
    where
        T: Pod,
    {
        let buffer = self
            .gpu
            .build_buffer()
            .data(slice::from_ref(self.data))
            .usage(self.usage)
            .create();
        let (bind_group, bind_group_layout) = self
            .gpu
            .build_bind_group()
            .add_entry(
                buffer
                    .to_binding()
                    .uniform()
                    .visibility(self.visibility),
            )
            .create_with_layout();

        Uniform {
            buffer,
            bind_group,
            bind_group_layout: bind_group_layout.into_owned(),
        }
    }
}

impl Gpu {
    pub fn build_uniform<'a, T: Pod>(&self, init: &'a T) -> UniformBuilder<'_, 'a, T> {
        UniformBuilder {
            gpu: self,
            usage: BufferUsages::UNIFORM,
            visibility: wgpu::ShaderStages::empty(),
            data: init,
        }
    }
}
