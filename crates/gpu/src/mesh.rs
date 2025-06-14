use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

use bytemuck::NoUninit;
use math::vector::{vec2f, vec3f, Vec2, Vec3};
use wgpu::{BufferUsages, IndexFormat};

use crate::buffer::Buffer;
use crate::handle::Handle;
use crate::payload::Payload;

#[derive(Debug)]
pub struct Mesh<V, I> {
    pub vertex_buffer: Buffer<V>,
    pub index_buffer: Buffer<I>,
}

impl<V, I> Mesh<V, I> {
    pub fn create(handle: &Handle, vertices: &[V], indices: &[I]) -> Self
    where
        V: NoUninit,
        I: NoUninit,
    {
        Self { vertex_buffer: Buffer::create(handle, vertices, BufferUsages::VERTEX), index_buffer: Buffer::create(handle, indices, BufferUsages::INDEX) }
    }

    pub fn new(vertex_buffer: Buffer<V>, index_buffer: Buffer<I>) -> Self {
        Self { vertex_buffer, index_buffer }
    }

    pub fn load_into_render_pass(&self, render_pass: &mut wgpu::RenderPass<'_>) -> u32
    where
        I: Index,
    {
        render_pass.set_index_buffer(self.index_buffer.inner.slice(..), I::FORMAT);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.inner.slice(..));
        self.index_buffer.len() as u32
    }
}

static MESHES_ID_COUNTER: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(0));

#[derive(Debug)]
pub struct Meshes<V, I> {
    handle: Handle,
    vec: Vec<Mesh<V, I>>,
    id: u32,
}

impl<V, I> Meshes<V, I> {
    pub fn new(handle: &Handle) -> Self {
        Self { handle: Handle::clone(handle), vec: Vec::new(), id: MESHES_ID_COUNTER.fetch_add(1, Ordering::Relaxed) }
    }

    pub fn create_and_insert(&mut self, vertices: &[V::Source], indices: &[I::Source]) -> MeshId
    where
        V: Payload,
        I: Payload,
    {
        let vertices = vertices.iter().map(V::from_source).collect::<Vec<_>>();
        let indices = indices.iter().map(I::from_source).collect::<Vec<_>>();
        let mesh = Mesh::create(&self.handle, &vertices, &indices);

        self.insert(mesh)
    }

    pub fn insert(&mut self, mesh: Mesh<V, I>) -> MeshId {
        let index = self.vec.len();
        self.vec.push(mesh);
        MeshId { parent_id: self.id, index }
    }

    pub fn create_and_insert_from(&mut self, f: impl FnOnce(&Handle) -> Mesh<V, I>) -> MeshId {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MeshId {
    parent_id: u32,
    index: usize,
}

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

pub fn quad<V: Vertex, I: Index>(handle: &Handle) -> Mesh<V, I> {
    Mesh::create(
        handle,
        &[
            V::new_3d(Vec3::new(-0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 0.0)),
            V::new_3d(Vec3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 0.0)),
            V::new_3d(Vec3::new(-0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 1.0)),
            V::new_3d(Vec3::new(0.5, -0.5, 0.5), Vec3::new(0.0, 0.0, 1.0), Vec2::new(1.0, 1.0)),
        ],
        &[0, 2, 1, 3, 1, 2].map(I::new_u16),
    )
}

pub trait Vertex: NoUninit {
    fn new_3d(position: vec3f, normal: vec3f, uv: vec2f) -> Self;
}
