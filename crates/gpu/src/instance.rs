use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;
use wgpu::BufferUsages;
use crate::handle::Handle;
use crate::buffer::Buffer;
use crate::payload::Payload;

static ID_COUNTER: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(0));

#[derive(Debug)]
pub struct Sets<I> {
    handle: Handle,
    buffers: Vec<Buffer<I>>,
    id: u32,
}

impl<I: Payload> Sets<I> {
    pub fn new(handle: &Handle) -> Self {
        Self {
            handle: Handle::clone(handle),
            buffers: Vec::new(),
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }
    
    pub fn insert_with_capacity(&mut self, capacity: usize) -> SetId {
        let buffer = Buffer::with_capacity(&self.handle, capacity as u64, BufferUsages::VERTEX | BufferUsages::COPY_DST);
        let index = self.buffers.len();
        self.buffers.push(buffer);
        
        SetId { parent_id: self.id, index }
    }
    
    pub fn insert<'a>(&mut self, data: impl IntoIterator<Item = &'a I::Source>) -> SetId {
        let instances = data.into_iter()
            .map(I::from_source)
            .collect::<Vec<_>>();
        
        let buffer = Buffer::create(&self.handle, &instances, BufferUsages::VERTEX | BufferUsages::COPY_DST);
        let index = self.buffers.len();
        self.buffers.push(buffer);
        
        SetId { parent_id: self.id, index }
    }
    
    pub fn insert_raw<'a>(&mut self, data: impl IntoIterator<Item = I>) -> SetId {
        let instances = data.into_iter().collect::<Vec<_>>();
        
        let buffer = Buffer::create(&self.handle, &instances, BufferUsages::VERTEX | BufferUsages::COPY_DST);
        let index = self.buffers.len();
        self.buffers.push(buffer);
        
        SetId { parent_id: self.id, index }
    }
    
    pub fn write<'a>(&mut self, id: SetId, data: impl IntoIterator<Item = &'a I::Source>) -> Result<(), ()> {
        debug_assert!(id.parent_id == self.id, "InstanceSetId does not belong to this InstanceSets instance");
        
        let instances = data.into_iter()
            .map(I::from_source)
            .collect::<Vec<_>>();
        
        self.buffers.get_mut(id.index).unwrap().write(&self.handle, 0, &instances)
    }

    pub fn write_raw<'a>(&mut self, id: SetId, data: impl IntoIterator<Item = I>) -> Result<(), ()> {
        debug_assert!(id.parent_id == self.id, "InstanceSetId does not belong to this InstanceSets instance");

        let instances = data.into_iter().collect::<Vec<_>>();
        self.buffers.get_mut(id.index).unwrap().write(&self.handle, 0, &instances)
    }
    
    pub fn get(&self, id: SetId) -> &Buffer<I> {
        debug_assert!(id.parent_id == self.id, "InstanceSetId does not belong to this InstanceSets instance");
        self.buffers.get(id.index).unwrap()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SetId {
    parent_id: u32,
    index: usize,
}