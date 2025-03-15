use std::mem::take;
use std::ops::{Deref, DerefMut};
use bytemuck::{cast_slice, NoUninit};
use wgpu::{BindGroupEntry, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, ShaderStages};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::gpu::handle::Handle;
use crate::gpu::mem::bind_group::{AddBindEntries, BindEntry};
use crate::gpu::mem::payload::ShaderPayload;

pub struct UnaryBuffer<T> {
    buffer: Buffer,
    value: T,
    visibility: ShaderStages,
    is_dirty: bool,
}

impl<T: ShaderPayload> UnaryBuffer<T> {
    pub fn create(handle: &Handle, value: T, visibility: ShaderStages) -> Self {
        Self {
            buffer: handle.create_buffer(&[value.payload()], BufferUsages::UNIFORM | BufferUsages::COPY_DST),
            value,
            visibility,
            is_dirty: false,
        }
    }

    pub fn submit(&mut self, handle: &Handle) {
        if !take(&mut self.is_dirty) {
            return;
        }

        handle.write_buffer(&self.buffer, 0, &[self.value.payload()]);
    }
}

impl<T: ShaderPayload> AddBindEntries for UnaryBuffer<T> {
    fn add_entries<'a>(&'a self, entries: &mut Vec<BindEntry<'a>>) {
        let binding = entries.len() as u32;
        entries.push(BindEntry {
            layout_entry: BindGroupLayoutEntry {
                binding,
                visibility: self.visibility,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            group_entry: BindGroupEntry {
                binding,
                resource: self.buffer.as_entire_binding()
            },
        })
    }
}

impl<T> Deref for UnaryBuffer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for UnaryBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.is_dirty = true;
        &mut self.value
    }
}

pub struct ArrayBuffer<T> {
    buffer: Buffer,
    vec: Vec<T>,
    visibility: ShaderStages,
    is_dirty: bool,
}

impl<T: ShaderPayload> ArrayBuffer<T> {
    pub fn create(handle: &Handle, vec: Vec<T>, visibility: ShaderStages) -> Self {
        let payloads = vec.iter().map(|x| x.payload()).collect::<Vec<_>>();
        Self {
            buffer: handle.create_buffer(&payloads, BufferUsages::STORAGE | BufferUsages::COPY_DST),
            vec,
            visibility,
            is_dirty: false,
        }
    }

    pub fn submit(&mut self, handle: &Handle) {
        if !take(&mut self.is_dirty) {
            return;
        }

        let payloads = self.vec.iter().map(|x| x.payload()).collect::<Vec<_>>();

        if payloads.len() * size_of::<T>() > self.buffer.size() as usize {
            self.buffer = handle.create_buffer(&payloads, BufferUsages::STORAGE | BufferUsages::COPY_DST);
        } else {
            handle.write_buffer(&self.buffer, 0, &payloads);
        }
    }
}

impl<T> Deref for ArrayBuffer<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T> DerefMut for ArrayBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.is_dirty = true;
        &mut self.vec
    }
}

impl<T: ShaderPayload> AddBindEntries for ArrayBuffer<T> {
    fn add_entries<'a>(&'a self, entries: &mut Vec<BindEntry<'a>>) {
        let binding = entries.len() as u32;
        entries.push(BindEntry {
            layout_entry: BindGroupLayoutEntry {
                binding,
                visibility: self.visibility,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            group_entry: BindGroupEntry {
                binding,
                resource: self.buffer.as_entire_binding()
            },
        })
    }
}

impl Handle {
    #[inline]
    pub fn create_buffer(&self, entries: &[impl NoUninit], usage: BufferUsages) -> Buffer {
        self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(entries),
            usage,
        })
    }

    #[inline]
    pub fn create_vertex_buffer(&self, vertices: &[impl NoUninit]) -> Buffer {
        self.create_buffer(vertices, BufferUsages::VERTEX | BufferUsages::COPY_DST)
    }

    #[inline]
    pub fn create_index_buffer(&self, indices: &[u16]) -> Buffer {
        self.create_buffer(indices, BufferUsages::INDEX | BufferUsages::COPY_DST)
    }

    pub fn create_unary_buffer<T: ShaderPayload>(&self, value: T, visibility: ShaderStages) -> UnaryBuffer<T> {
        UnaryBuffer::create(self, value, visibility)
    }

    pub fn create_array_buffer<T: ShaderPayload>(&self, vec: Vec<T>, visibility: ShaderStages) -> ArrayBuffer<T> {
        ArrayBuffer::create(self, vec, visibility)
    }

    #[inline]
    pub fn write_buffer<T: NoUninit>(&self, buffer: &Buffer, offset: u64, data: &[T]) {
        self.queue.write_buffer(buffer, offset * size_of::<T>() as u64, cast_slice(data));
    }
}