use std::slice;
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UiVertex {
    pub pos: [f32; 2],
    pub tex_pos: [f32; 2],
    pub tex_index: u32,
}

impl UiVertex {
    const ATTRIBUTES: [VertexAttribute; 3] = vertex_attr_array![
        0 => Float32x2,
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

    #[inline]
    pub const fn new(pos: [f32; 2], tex_pos: [f32; 2], tex_index: u32) -> Self {
        Self { pos, tex_pos, tex_index }
    }
}

impl AsRef<[u8]> for UiVertex {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const UiVertex as *const u8, size_of::<Self>()) }
    }
}