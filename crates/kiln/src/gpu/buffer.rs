use std::hash::Hash;
use std::marker::PhantomData;

use bytemuck::{Pod, cast_slice};
use either::Either;
pub use wgpu::BufferUsages;

use crate::Gpu;

#[derive(Debug)]
pub struct Buffer<T> {
    inner: Option<wgpu::Buffer>,
    usage: BufferUsages,
    _marker: PhantomData<T>,
}

impl<T> Buffer<T> {
    pub fn new(gpu: &Gpu, capacity: usize, usage: BufferUsages) -> Self {
        Self {
            inner: Some(gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: (capacity * size_of::<T>()) as u64,
                usage,
                mapped_at_creation: false,
            })),
            usage,
            _marker: PhantomData,
        }
    }

    pub fn new_with_data(gpu: &Gpu, data: &[T], usage: BufferUsages) -> Self
    where
        T: Pod,
    {
        use wgpu::util::{BufferInitDescriptor, DeviceExt};
        Self {
            inner: Some(
                gpu.device
                    .create_buffer_init(&BufferInitDescriptor {
                        label: None,
                        contents: cast_slice(data),
                        usage,
                    }),
            ),
            usage,
            _marker: PhantomData,
        }
    }

    pub fn empty(usage: BufferUsages) -> Self {
        Self {
            inner: None,
            usage,
            _marker: PhantomData,
        }
    }

    pub fn buffer(&self) -> Option<&wgpu::Buffer> {
        self.inner.as_ref()
    }

    pub fn usage(&self) -> BufferUsages {
        self.usage
    }

    pub fn size_bytes(&self) -> u64 {
        self.inner.as_ref().map_or(0, |b| b.size())
    }

    pub fn capacity(&self) -> usize {
        (self.size_bytes() / size_of::<T>() as u64) as usize
    }

    fn ensure_allocated(&mut self, gpu: &Gpu, min_elements: usize) {
        if self.inner.is_none() && min_elements > 0 {
            let size_bytes = (min_elements * size_of::<T>()) as u64;
            let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: size_bytes,
                usage: self.usage,
                mapped_at_creation: false,
            });
            self.inner = Some(buffer);
        }
    }

    pub fn write(&mut self, gpu: &Gpu, data: &[T])
    where
        T: bytemuck::Pod,
    {
        self.ensure_allocated(gpu, data.len());
        debug_assert!(data.len() <= self.capacity());

        if let Some(buffer) = &self.inner {
            gpu.queue
                .write_buffer(buffer, 0, cast_slice(data));
        }
    }

    pub fn write_at(&mut self, gpu: &Gpu, offset_elements: usize, data: &[T])
    where
        T: bytemuck::Pod,
    {
        self.ensure_allocated(gpu, offset_elements + data.len());
        debug_assert!(offset_elements + data.len() <= self.capacity());

        if let Some(buffer) = &self.inner {
            let offset_bytes = (offset_elements * size_of::<T>()) as u64;
            gpu.queue
                .write_buffer(buffer, offset_bytes, cast_slice(data));
        }
    }

    pub fn is_allocated(&self) -> bool {
        self.inner.is_some()
    }

    pub fn take(&mut self) -> Option<wgpu::Buffer> {
        self.inner.take()
    }

    pub fn replace(&mut self, buffer: wgpu::Buffer) -> Option<wgpu::Buffer> {
        self.inner.replace(buffer)
    }

    pub fn slice(&self, start_elements: usize, end_elements: usize) -> Option<wgpu::BufferSlice> {
        debug_assert!(start_elements <= end_elements);
        debug_assert!(end_elements <= self.capacity());

        self.inner.as_ref().map(|b| {
            let start_bytes = (start_elements * size_of::<T>()) as u64;
            let end_bytes = (end_elements * size_of::<T>()) as u64;
            b.slice(start_bytes..end_bytes)
        })
    }

    pub fn slice_full(&self) -> Option<wgpu::BufferSlice> {
        self.inner.as_ref().map(|b| b.slice(..))
    }

    pub(crate) fn inner(&self) -> &wgpu::Buffer {
        self.inner
            .as_ref()
            .expect("Unallocated buffers cannot be dereferenced")
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        if let Some(buffer) = &self.inner {
            buffer.destroy();
        }
    }
}

impl Gpu {
    #[inline]
    pub fn build_buffer<T>(&self) -> BufferBuilder<'_, '_, T> {
        BufferBuilder {
            gpu: self,
            data_or_size: either::Right(0),
            usage: BufferUsages::empty(),
        }
    }
}

pub struct BufferBuilder<'gpu, 'a, T> {
    gpu: &'gpu Gpu,
    data_or_size: Either<&'a [T], usize>,
    usage: BufferUsages,
}

impl<'a, T> BufferBuilder<'_, 'a, T> {
    #[inline]
    pub fn data(mut self, data: &'a [T]) -> Self {
        self.data_or_size = Either::Left(data);
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.data_or_size = Either::Right(size);
        self
    }
}

impl<'a, T: Pod> BufferBuilder<'_, 'a, T> {
    #[inline]
    pub fn create(self) -> Buffer<T> {
        match self.data_or_size {
            Either::Left(data) => Buffer::new_with_data(self.gpu, data, self.usage),
            Either::Right(size) => Buffer::new(self.gpu, size, self.usage),
        }
    }
}

pub struct GrowBuffer<T> {
    buffer: Buffer<T>,
    len: usize,
}

impl<T> GrowBuffer<T> {
    pub fn new(gpu: &Gpu, capacity_elements: usize, usage: BufferUsages) -> Self {
        let buffer = if capacity_elements > 0 {
            Buffer::new(gpu, capacity_elements, usage)
        } else {
            Buffer::empty(usage)
        };

        Self { buffer, len: 0 }
    }

    pub fn new_with_data(gpu: &Gpu, data: &[T], usage: BufferUsages) -> Self
    where
        T: bytemuck::Pod,
    {
        let buffer = Buffer::new_with_data(gpu, data, usage);
        Self { buffer, len: data.len() }
    }

    pub fn empty(usage: BufferUsages) -> Self {
        Self {
            buffer: Buffer::empty(usage),
            len: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    #[inline]
    pub fn usage(&self) -> BufferUsages {
        self.buffer.usage()
    }

    fn next_capacity(&self, min_capacity: usize) -> usize {
        let current = self.capacity();
        if min_capacity <= current {
            return current;
        }

        let grown = (current * 3) / 2;
        grown.max(min_capacity).max(4)
    }

    pub fn reserve(&mut self, gpu: &Gpu, additional_elements: usize) {
        let required = self.len + additional_elements;
        if required <= self.capacity() {
            return;
        }

        let new_capacity = self.next_capacity(required);
        self.grow_to_capacity(gpu, new_capacity);
    }

    pub fn reserve_exact(&mut self, gpu: &Gpu, additional_elements: usize) {
        let required = self.len + additional_elements;
        if required <= self.capacity() {
            return;
        }

        self.grow_to_capacity(gpu, required);
    }

    fn grow_to_capacity(&mut self, gpu: &Gpu, new_capacity_elements: usize) {
        let new_buffer = Buffer::new(gpu, new_capacity_elements, self.buffer.usage());
        self.buffer = new_buffer;
    }

    pub fn overwrite(&mut self, gpu: &Gpu, data: &[T])
    where
        T: bytemuck::Pod,
    {
        if data.len() > self.capacity() {
            let new_capacity = self.next_capacity(data.len());
            self.grow_to_capacity(gpu, new_capacity);
        }

        self.buffer.write(gpu, data);
        self.len = data.len();
    }

    pub unsafe fn set_len(&mut self, len_elements: usize) {
        debug_assert!(len_elements <= self.capacity());
        self.len = len_elements;
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn shrink_to_fit(&mut self, gpu: &Gpu) {
        if self.len == 0 {
            self.buffer = Buffer::empty(self.buffer.usage());
        } else if self.len < self.capacity() {
            self.grow_to_capacity(gpu, self.len);
        }
    }

    pub fn buffer(&self) -> &Buffer<T> {
        &self.buffer
    }

    pub fn slice(&self) -> Option<wgpu::BufferSlice> {
        if self.len == 0 {
            return None;
        }
        self.buffer.slice(0, self.len)
    }

    pub fn slice_range(&self, start_elements: usize, end_elements: usize) -> Option<wgpu::BufferSlice> {
        debug_assert!(start_elements <= end_elements);
        debug_assert!(end_elements <= self.len);

        if start_elements >= end_elements {
            return None;
        }

        self.buffer.slice(start_elements, end_elements)
    }

    pub fn write(&mut self, gpu: &Gpu, data: &[T])
    where
        T: bytemuck::Pod,
    {
        if data.len() > self.capacity() {
            let new_capacity = self.next_capacity(data.len());
            self.grow_to_capacity(gpu, new_capacity);
        }

        debug_assert!(data.len() <= self.capacity());
        self.buffer.write(gpu, data);
    }

    pub fn write_at(&mut self, gpu: &Gpu, offset_elements: usize, data: &[T])
    where
        T: bytemuck::Pod,
    {
        let required_capacity = offset_elements + data.len();
        if required_capacity > self.capacity() {
            let new_capacity = self.next_capacity(required_capacity);
            self.grow_to_capacity(gpu, new_capacity);
        }

        debug_assert!(offset_elements + data.len() <= self.capacity());
        self.buffer.write_at(gpu, offset_elements, data);
    }
}

pub trait BufferUsagesBuilderExt: Sized {
    fn usage(self, usage: BufferUsages) -> Self;

    fn use_map_read(self) -> Self {
        self.usage(BufferUsages::MAP_READ)
    }

    fn use_map_write(self) -> Self {
        self.usage(BufferUsages::MAP_WRITE)
    }

    fn use_copy_dst(self) -> Self {
        self.usage(BufferUsages::COPY_DST)
    }

    fn use_copy_src(self) -> Self {
        self.usage(BufferUsages::COPY_SRC)
    }

    fn use_index(self) -> Self {
        self.usage(BufferUsages::INDEX)
    }

    fn use_vertex(self) -> Self {
        self.usage(BufferUsages::VERTEX)
    }

    fn use_uniform(self) -> Self {
        self.usage(BufferUsages::UNIFORM)
    }

    fn use_storage(self) -> Self {
        self.usage(BufferUsages::STORAGE)
    }

    fn use_indirect(self) -> Self {
        self.usage(BufferUsages::INDIRECT)
    }

    fn use_query_resolve(self) -> Self {
        self.usage(BufferUsages::QUERY_RESOLVE)
    }

    fn use_blas_input(self) -> Self {
        self.usage(BufferUsages::BLAS_INPUT)
    }

    fn use_tlas_input(self) -> Self {
        self.usage(BufferUsages::TLAS_INPUT)
    }
}

impl<T> BufferUsagesBuilderExt for BufferBuilder<'_, '_, T> {
    fn usage(mut self, usage: BufferUsages) -> Self {
        self.usage.insert(usage);
        self
    }
}
