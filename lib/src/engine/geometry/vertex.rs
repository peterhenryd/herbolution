use bytemuck::{Pod, Zeroable};
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};
use math::vector::{vec2f, vec3f, ArrVec2F32, ArrVec3F32};
use crate::engine::as_no_uninit::AsNoUninit;

pub struct Vertex {
    pub position: vec3f,
    pub texture_position: vec2f,
}

impl Vertex {
    pub(super) const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<ArrVertex>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &Self::ATTRIBUTES,
    };
    const ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
    ];
}

impl AsNoUninit for Vertex {
    type Output = ArrVertex;

    fn as_no_uninit(&self) -> Self::Output {
        ArrVertex(self.position.into(), self.texture_position.into())
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrVertex(pub ArrVec3F32, pub ArrVec2F32);