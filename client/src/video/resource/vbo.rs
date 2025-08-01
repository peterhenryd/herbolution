use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::video::gpu;
use crate::video::resource::Buffer;
use bytemuck::NoUninit;
use lib::vector::{vec2f, vec3f};
use serde::{Deserialize, Serialize};
use wgpu::{BufferUsages, IndexFormat};

static MESHES_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
static SETS_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub trait Index: NoUninit {
    const FORMAT: IndexFormat;

    fn new_u16(value: u16) -> Self;
}

impl Index for u16 {
    const FORMAT: IndexFormat = IndexFormat::Uint16;

    fn new_u16(value: u16) -> Self {
        value
    }
}

impl Index for u32 {
    const FORMAT: IndexFormat = IndexFormat::Uint32;

    fn new_u16(value: u16) -> Self {
        value as u32
    }
}

pub trait Vertex: NoUninit {
    fn new_3d(position: vec3f, normal: vec3f, uv: vec2f) -> Self;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MeshId {
    parent_id: u32,
    index: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SetId {
    parent_id: u32,
    index: usize,
}

// Mesh

#[derive(Debug)]
pub struct Mesh<V, I> {
    vertex_buffer: Buffer<V>,
    index_buffer: Buffer<I>,
}

impl<V, I> Mesh<V, I> {
    pub fn create(gpu: &gpu::Handle, vertices: &[V], indices: &[I]) -> Self
    where
        V: NoUninit,
        I: NoUninit,
    {
        Self {
            vertex_buffer: Buffer::from_data(gpu, vertices, BufferUsages::VERTEX),
            index_buffer: Buffer::from_data(gpu, indices, BufferUsages::INDEX),
        }
    }

    pub fn new(vertex_buffer: Buffer<V>, index_buffer: Buffer<I>) -> Self {
        Self { vertex_buffer, index_buffer }
    }

    pub fn read(gpu: &gpu::Handle, path: impl AsRef<Path>) -> Mesh<V, I>
    where
        V: Vertex,
        I: Index,
    {
        let contents = read_to_string(path).unwrap();
        let mut data = toml::from_str::<TomlMeshFile>(&contents)
            .unwrap()
            .mesh;

        let size = data.positions.len();
        let mut vertices = Vec::with_capacity(size);

        let max = data.normals.len();
        let mut i = 0;
        while data.normals.len() < size {
            let value = data.normals[i % max];
            data.normals.push(value);
            i += 1;
        }

        let max = data.uvs.len();
        let mut i = 0;
        while data.uvs.len() < size {
            let value = data.uvs[i % max];
            data.uvs.push(value);
            i += 1;
        }

        let positions = data.positions.into_iter();
        let normals = data.normals.into_iter();
        let uvs = data.uvs.into_iter();
        for ((position, normal), uv) in positions.zip(normals).zip(uvs) {
            vertices.push(V::new_3d(position.into(), normal.into(), uv.into()));
        }

        let indices = data
            .indices
            .into_iter()
            .map(I::new_u16)
            .collect::<Vec<_>>();

        Mesh::create(gpu, &vertices, &indices)
    }

    pub fn load_into_render_pass(&self, render_pass: &mut wgpu::RenderPass<'_>) -> u32
    where
        I: Index,
    {
        render_pass.set_index_buffer(self.index_buffer.inner().slice(..), I::FORMAT);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.inner().slice(..));
        self.index_buffer.len() as u32
    }
}

// Meshes

#[derive(Debug)]
pub struct Meshes<V, I = u16> {
    handle: gpu::Handle,
    vec: Vec<Mesh<V, I>>,
    id: u32,
}

impl<V, I> Meshes<V, I> {
    pub fn new(gpu: &gpu::Handle) -> Self {
        Self {
            handle: gpu.clone(),
            vec: Vec::new(),
            id: MESHES_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn create_and_insert(&mut self, vertices: &[V], indices: &[I]) -> MeshId
    where
        V: NoUninit,
        I: NoUninit,
    {
        self.insert(Mesh::create(&self.handle, &vertices, &indices))
    }

    pub fn insert(&mut self, mesh: Mesh<V, I>) -> MeshId {
        let index = self.vec.len();
        self.vec.push(mesh);
        MeshId { parent_id: self.id, index }
    }

    pub fn create_and_insert_from(&mut self, f: impl FnOnce(&gpu::Handle) -> Mesh<V, I>) -> MeshId {
        let mesh = f(&self.handle);
        self.insert(mesh)
    }

    #[inline]
    pub fn get(&self, id: MeshId) -> &Mesh<V, I> {
        debug_assert!(id.parent_id == self.id, "MeshId does not belong to this Meshes instance");

        self.vec.get(id.index).unwrap()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TomlMeshFile {
    mesh: TomlMesh,
}

#[derive(Debug, Serialize, Deserialize)]
struct TomlMesh {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u16>,
}

// Sets

#[derive(Debug)]
pub struct Sets<I> {
    handle: gpu::Handle,
    buffers: Vec<Buffer<I>>,
    id: u32,
}

impl<I> Sets<I> {
    pub fn new(gpu: &gpu::Handle) -> Self {
        Self {
            handle: gpu::Handle::clone(gpu),
            buffers: Vec::new(),
            id: SETS_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn insert_with_capacity(&mut self, capacity: usize) -> SetId {
        let buffer = Buffer::create(&self.handle, capacity as u64, BufferUsages::VERTEX | BufferUsages::COPY_DST);
        let index = self.buffers.len();
        self.buffers.push(buffer);

        SetId { parent_id: self.id, index }
    }

    pub fn insert<'a>(&mut self, instances: &[I]) -> SetId
    where
        I: NoUninit,
    {
        let buffer = Buffer::from_data(&self.handle, instances, BufferUsages::VERTEX | BufferUsages::COPY_DST);
        let index = self.buffers.len();
        self.buffers.push(buffer);

        SetId { parent_id: self.id, index }
    }

    pub fn insert_from(&mut self, iter: impl IntoIterator<Item = I>) -> SetId
    where
        I: NoUninit,
    {
        self.insert(&iter.into_iter().collect::<Vec<_>>())
    }

    pub fn write<'a>(&mut self, id: SetId, instances: &[I]) -> Result<(), ()>
    where
        I: NoUninit,
    {
        debug_assert!(id.parent_id == self.id, "InstanceSetId does not belong to this InstanceSets instance");

        self.buffers[id.index].write(&self.handle, 0, &instances)
    }

    pub fn write_from<'a>(&mut self, id: SetId, iter: impl IntoIterator<Item = I>) -> Result<(), ()>
    where
        I: NoUninit,
    {
        self.write(id, &iter.into_iter().collect::<Vec<_>>())
    }

    pub fn get(&self, id: SetId) -> &Buffer<I> {
        debug_assert!(id.parent_id == self.id, "InstanceSetId does not belong to this InstanceSets instance");
        &self.buffers[id.index]
    }

    pub fn get_mut(&mut self, id: SetId) -> &mut Buffer<I> {
        debug_assert!(id.parent_id == self.id, "InstanceSetId does not belong to this InstanceSets instance");
        &mut self.buffers[id.index]
    }
}
