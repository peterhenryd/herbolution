use crate::gpu::mem::model::VertexShaderArgument;
use crate::gpu::mem::payload::{AutoShaderPayload, ShaderPayload};
use bytemuck::{Pod, Zeroable};
use math::color::Rgba;
use math::matrix::{mat4f, Mat4};
use math::num::traits::ConstZero;
use math::rotation::Quat;
use math::vector::{vec3f, Vec3};
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

pub fn buffer_layouts() -> [VertexBufferLayout<'static>; 2] {
    [Vertex3D::LAYOUT, Instance::LAYOUT]
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Vertex3D {
    pub pos: vec3f,
    pub norm: vec3f,
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
    pub const fn new(pos: vec3f, norm: vec3f) -> Self {
        Self { pos, norm }
    }
}

impl AutoShaderPayload for Vertex3D {}

pub struct Instance {
    pub pos: vec3f,
    pub quat: Quat,
    pub texture_index: u32,
    pub color: Rgba<f32>,
    pub is_lit: bool,
}

impl VertexShaderArgument for Instance {
    const ATTRIBUTES: &'static [VertexAttribute] = &vertex_attr_array![
        2 => Float32x4,
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x4,
        6 => Uint32,
        7 => Float32x4,
        8 => Uint32,
    ];
    const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<InstanceShaderPayload>() as BufferAddress,
        step_mode: VertexStepMode::Instance,
        attributes: &Self::ATTRIBUTES,
    };
}

impl Instance {
    pub fn model_matrix(&self) -> mat4f {
        Mat4::from_translation(self.pos) * self.quat.into_matrix()
    }
}

impl ShaderPayload for Instance {
    type Output<'a> = InstanceShaderPayload;

    fn payload(&self) -> Self::Output<'_> {
        InstanceShaderPayload {
            model: self.model_matrix(),
            texture_index: self.texture_index,
            color: self.color,
            is_lit: self.is_lit as u32,
        }
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            pos: Vec3::ZERO,
            quat: Quat::IDENTITY,
            texture_index: 0,
            color: Rgba::TRANSPARENT,
            is_lit: false,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct InstanceShaderPayload {
    pub model: mat4f,
    pub texture_index: u32,
    pub color: Rgba<f32>,
    pub is_lit: u32,
}