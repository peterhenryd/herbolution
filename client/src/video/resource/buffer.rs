use crate::video::gpu::Handle;
use bytemuck::{bytes_of, cast_slice, NoUninit};
use std::cmp::max;
use std::marker::PhantomData;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BufferDescriptor, BufferUsages};

#[derive(Debug, Clone)]
pub struct Buffer<T> {
    inner: wgpu::Buffer,
    len: u64,
    _marker: PhantomData<T>,
}

impl<T> Buffer<T> {
    pub fn create(gpu: &Handle, capacity: u64, usage: BufferUsages) -> Self {
        Self {
            inner: gpu.device().create_buffer(&BufferDescriptor {
                label: None,
                size: capacity * size_of::<T>() as u64,
                usage,
                mapped_at_creation: false,
            }),
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn from_data(gpu: &Handle, data: &[T], usage: BufferUsages) -> Self
    where
        T: NoUninit,
    {
        Self {
            inner: gpu
                .device()
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: cast_slice(data),
                    usage,
                }),
            len: data.len() as u64,
            _marker: PhantomData,
        }
    }

    pub fn write(&mut self, gpu: &Handle, offset: u64, data: &[T]) -> Result<(), ()>
    where
        T: NoUninit,
    {
        let new_len = offset + data.len() as u64;
        if new_len > self.capacity() || offset > self.len {
            return Err(());
        }

        gpu.queue()
            .write_buffer(&self.inner, offset * size_of::<T>() as u64, cast_slice(data));
        self.len = max(self.len, new_len);

        Ok(())
    }

    pub fn push(&mut self, gpu: &Handle, value: T) -> Result<(), T>
    where
        T: NoUninit,
    {
        if self.len >= self.capacity() {
            return Err(value);
        }

        gpu.queue()
            .write_buffer(&self.inner, self.len * size_of::<T>() as u64, bytes_of(&value));
        self.len += 1;

        Ok(())
    }

    #[inline]
    pub fn len(&self) -> u64 {
        self.len
    }

    #[inline]
    pub fn capacity(&self) -> u64 {
        self.inner.size() / size_of::<T>() as u64
    }

    #[inline]
    pub fn usage(&self) -> BufferUsages {
        self.inner.usage()
    }

    #[inline]
    pub fn inner(&self) -> &wgpu::Buffer {
        &self.inner
    }

    pub fn shorten_to(&mut self, new_len: u64) {
        if new_len < self.len {
            self.len = new_len;
        }
    }
}

impl<'a, T> AsRef<Buffer<T>> for &'a Buffer<T> {
    fn as_ref(&self) -> &Buffer<T> {
        self
    }
}

#[derive(Debug, Clone)]
pub struct GrowBuffer<T> {
    buffer: Buffer<T>,
}

impl<T> GrowBuffer<T> {
    #[inline]
    pub fn with_capacity(gpu: &Handle, capacity: u64, usage: BufferUsages) -> Self {
        Self {
            buffer: Buffer::create(gpu, capacity, usage),
        }
    }

    #[inline]
    pub fn empty(gpu: &Handle, usage: BufferUsages) -> Self {
        Self {
            buffer: Buffer::create(gpu, 0, usage),
        }
    }

    pub fn write(&mut self, gpu: &Handle, data: &[T])
    where
        T: NoUninit,
    {
        let len = data.len() as u64;
        if len > self.capacity() {
            let usage = self.buffer.inner.usage();
            self.buffer.inner = gpu
                .device()
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: cast_slice(data),
                    usage,
                });
        } else {
            gpu.queue()
                .write_buffer(&self.buffer.inner, 0, cast_slice(data));
        }
        self.buffer.len = len;
    }

    pub fn len(&self) -> u64 {
        self.buffer.len
    }

    pub fn capacity(&self) -> u64 {
        self.buffer.capacity()
    }

    pub fn inner(&self) -> &wgpu::Buffer {
        self.buffer.inner()
    }
}

impl<'a, T> AsRef<Buffer<T>> for &'a GrowBuffer<T> {
    fn as_ref(&self) -> &Buffer<T> {
        &self.buffer
    }
}
