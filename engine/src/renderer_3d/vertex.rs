use bytemuck::{Pod, Zeroable};
use math::vector::vec3f;
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};
use math::color::Rgba;
use math::matrix::{mat4f, Mat4};
use math::rotation::Quat;
use crate::gpu::mem::model::VertexShaderArgument;
use crate::gpu::mem::payload::{AutoShaderPayload, ShaderPayload};

pub fn buffer_layouts() -> [VertexBufferLayout<'static>; 2] {
    [Vertex3D::LAYOUT, Instance::LAYOUT]
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Vertex3D {
    pub position: vec3f,
    pub normal: vec3f,
}

impl VertexShaderArgument for Vertex3D {
    const ATTRIBUTES: &'static [VertexAttribute] = &vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
    ];
    const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Vertex3D>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: Self::ATTRIBUTES,
    };
}

impl Vertex3D {
    pub const fn new(position: vec3f, normal: vec3f) -> Self {
        Self { position, normal }
    }
}

impl AutoShaderPayload for Vertex3D {}

pub struct Instance {
    pub position: vec3f,
    pub rotation: Quat,
    pub texture_index: u32,
    pub color: Rgba<f32>,
}

impl VertexShaderArgument for Instance {
    const ATTRIBUTES: &'static [VertexAttribute] = &vertex_attr_array![
        2 => Float32x4,
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x4,
        6 => Uint32,
        7 => Float32x4,
    ];
    const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<InstanceShaderPayload>() as BufferAddress,
        step_mode: VertexStepMode::Instance,
        attributes: &Self::ATTRIBUTES,
    };
}

impl Instance {
    pub fn model_matrix(&self) -> mat4f {
        Mat4::from_translation(self.position) * self.rotation.into_matrix()
    }
}

impl ShaderPayload for Instance {
    type Output<'a> = InstanceShaderPayload;

    fn payload(&self) -> Self::Output<'_> {
        InstanceShaderPayload {
            model_matrix: self.model_matrix(),
            texture_index: self.texture_index,
            color: self.color,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct InstanceShaderPayload {
    pub model_matrix: mat4f,
    pub texture_index: u32,
    pub color: Rgba<f32>,
}