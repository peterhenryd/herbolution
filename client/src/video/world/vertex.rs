use bytemuck::{Pod, Zeroable};
use lib::color::Rgba;
use lib::matrix::Mat3;
use lib::rotation::Quat;
use lib::vector::{Vec3, Vec4, vec2f, vec3f, vec4f};
use serde::{Deserialize, Serialize};
use wgpu::{VertexBufferLayout, VertexStepMode, vertex_attr_array};

use crate::video::resource::Vertex;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable, Deserialize, Serialize)]
pub struct Vertex3d {
    pub position: vec3f,
    pub normal: vec3f,
    pub uv: vec2f,
}

impl Vertex3d {
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Vertex3d>() as u64,
        step_mode: VertexStepMode::Vertex,
        attributes: &vertex_attr_array![
            0 => Float32x3,
            1 => Float32x3,
            2 => Float32x2,
        ],
    };

    pub const fn new(position: vec3f, normal: vec3f, uv: vec2f) -> Self {
        Self { position, normal, uv }
    }
}

impl Vertex for Vertex3d {
    fn new_3d(position: vec3f, normal: vec3f, uv: vec2f) -> Self {
        Self::new(position, normal, uv)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Instance3d {
    model_0: vec4f,
    model_1: vec4f,
    model_2: vec4f,
    model_3: vec4f,
    color: Rgba<f32>,
    light: u32,
    ao: vec4f,
}

impl Instance3d {
    pub fn new(position: vec3f, rotation: Quat, scale: vec3f, color: Rgba<f32>, light: u32, ao: vec4f) -> Self {
        let rotation_matrix = rotation.to_axes();
        let model_matrix = rotation_matrix * Mat3::from(scale);

        Instance3d {
            model_0: model_matrix.x.extend(0.0),
            model_1: model_matrix.y.extend(0.0),
            model_2: model_matrix.z.extend(0.0),
            model_3: position.extend(1.0),
            color,
            light,
            ao,
        }
    }
}

impl Instance3d {
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Instance3d>() as u64,
        step_mode: VertexStepMode::Instance,
        attributes: &vertex_attr_array![
            3 => Float32x4,
            4 => Float32x4,
            5 => Float32x4,
            6 => Float32x4,
            7 => Float32x4,
            8 => Uint32,
            9 => Float32x4,
        ],
    };
}

impl Default for Instance3d {
    fn default() -> Self {
        Instance3d::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE, Rgba::TRANSPARENT, 1, Vec4::ZERO)
    }
}
