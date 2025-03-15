use bytemuck::{Pod, Zeroable};
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};
use math::vector::vec2f;
use crate::gpu::mem::model::VertexShaderArgument;
use crate::gpu::mem::payload::AutoShaderPayload;

pub fn buffer_layouts() -> [VertexBufferLayout<'static>; 1] {
    [Vertex2D::LAYOUT]
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Vertex2D {
    pub position: vec2f,
}

impl Vertex2D {
    pub const fn new(position: vec2f) -> Self {
        Self { position }
    }
}

impl VertexShaderArgument for Vertex2D {
    const ATTRIBUTES: &'static [VertexAttribute] = &vertex_attr_array![
        0 => Float32x2
    ];
    const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Vertex2D>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: Self::ATTRIBUTES,
    };
}

impl AutoShaderPayload for Vertex2D {}