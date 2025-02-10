use bytemuck::{Pod, Zeroable};
use math::vector::{vec3f, ArrVec3F32};
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};
use math::as_no_uninit::AsNoUninit;

pub struct Vertex {
    pub position: vec3f,
    pub normal: vec3f,
}

impl Vertex {
    pub(super) const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<ArrVertex>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &Self::ATTRIBUTES,
    };
    const ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
    ];
}

impl AsNoUninit for Vertex {
    type Output = ArrVertex;

    fn as_no_uninit(&self) -> Self::Output {
        ArrVertex(self.position.into(), self.normal.into())
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrVertex(pub ArrVec3F32, pub ArrVec3F32);