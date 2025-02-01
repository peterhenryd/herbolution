use std::slice;
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WorldVertex {
    pub pos: [f32; 3],
    pub tex_pos: [f32; 2],
    pub tex_index: u32,
}

impl WorldVertex {
    const ATTRIBUTES: [VertexAttribute; 3] = vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
        2 => Uint32
    ];

    pub fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }

    pub fn new(pos: [f32; 3], tex_pos: [f32; 2], tex_index: u32) -> Self {
        Self { pos, tex_pos, tex_index }
    }
}

impl AsRef<[u8]> for WorldVertex {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const WorldVertex as *const u8, size_of::<Self>()) }
    }
}