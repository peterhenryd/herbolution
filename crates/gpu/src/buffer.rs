use std::cmp::max;
use std::marker::PhantomData;

use bytemuck::{NoUninit, bytes_of, cast_slice};
use wgpu::BufferDescriptor;
pub use wgpu::BufferUsages as Usage;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::handle::Handle;

#[derive(Debug, Clone)]
pub struct Buffer<T> {
    inner: wgpu::Buffer,
    len: u64,
    _marker: PhantomData<T>,
}

impl<T> Buffer<T> {
    pub fn create(handle: &Handle, data: &[T], usage: Usage) -> Self
    where
        T: NoUninit,
    {
        Self {
            inner: handle
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

    pub fn with_capacity(handle: &Handle, capacity: u64, usage: Usage) -> Self {
        Self {
            inner: handle
                .device()
                .create_buffer(&BufferDescriptor {
                    label: None,
                    size: capacity * size_of::<T>() as u64,
                    usage,
                    mapped_at_creation: false,
                }),
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn write(&mut self, handle: &Handle, offset: u64, data: &[T]) -> Result<(), ()>
    where
        T: NoUninit,
    {
        let new_len = offset + data.len() as u64;
        if new_len > self.capacity() || offset > self.len {
            return Err(());
        }

        handle
            .queue()
            .write_buffer(&self.inner, offset * size_of::<T>() as u64, cast_slice(data));
        self.len = max(self.len, new_len);

        Ok(())
    }

    pub fn push(&mut self, handle: &Handle, value: T) -> Result<(), T>
    where
        T: NoUninit,
    {
        if self.len >= self.capacity() {
            return Err(value);
        }

        handle
            .queue()
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
    pub fn usage(&self) -> Usage {
        self.inner.usage()
    }

    #[inline]
    pub fn inner(&self) -> &wgpu::Buffer {
        &self.inner
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
    pub fn with_capacity(handle: &Handle, capacity: u64, usage: Usage) -> Self {
        Self {
            buffer: Buffer::with_capacity(handle, capacity, usage),
        }
    }

    #[inline]
    pub fn empty(handle: &Handle, usage: Usage) -> Self {
        Self {
            buffer: Buffer::with_capacity(handle, 0, usage),
        }
    }

    pub fn write(&mut self, handle: &Handle, data: &[T])
    where
        T: NoUninit,
    {
        let len = data.len() as u64;
        if len > self.capacity() {
            let usage = self.buffer.inner.usage();
            self.buffer.inner = handle
                .device()
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: cast_slice(data),
                    usage,
                });
        } else {
            handle
                .queue()
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
