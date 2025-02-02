use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};
use std::slice;
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct WorldVertex {
    pub pos: Vec3,
    pub tex_pos: Vec2,
}

impl WorldVertex {
    const ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
    ];

    pub const fn new(pos: Vec3, tex_pos: Vec2) -> Self {
        Self { pos, tex_pos }
    }

    pub fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

impl AsRef<[u8]> for WorldVertex {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const WorldVertex as *const u8, size_of::<Self>()) }
    }
}
