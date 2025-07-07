use crate::video::resource::{AtlasTextureCoord, Vertex};
use bytemuck::{Pod, Zeroable};
use lib::color::Rgba;
use lib::matrix::Mat3;
use lib::rotation::Quat;
use lib::size::size2f;
use lib::vector::{vec2f, vec3f};
use wgpu::{vertex_attr_array, VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Vertex2d {
    pub position: vec2f,
    pub normal: vec2f,
    pub uv: vec2f,
}

impl Vertex2d {
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Vertex2d>() as u64,
        step_mode: VertexStepMode::Vertex,
        attributes: &vertex_attr_array![
            0 => Float32x2, // position
            1 => Float32x2, // normal
            2 => Float32x2, // uv
        ],
    };

    pub fn new(position: vec2f, normal: vec2f, uv: vec2f) -> Self {
        Self { position, normal, uv }
    }
}

impl Vertex for Vertex2d {
    fn new_3d(position: vec3f, normal: vec3f, uv: vec2f) -> Self {
        Self::new(position.xy(), normal.xy(), uv)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Instance2d {
    model_0: vec2f,
    model_1: vec2f,
    model_2: vec2f,
    color: Rgba<f32>,
    uv_t: vec2f,
    uv_s: vec2f,
    scale: size2f,
}

impl Instance2d {
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as u64,
        step_mode: VertexStepMode::Instance,
        attributes: &vertex_attr_array![
            3 => Float32x2, // model matrix column 0
            4 => Float32x2, // model matrix column 1
            5 => Float32x2, // model matrix column 2
            6 => Float32x4, // color
            7 => Float32x2, // texture translation
            8 => Float32x2, // texture scale
        ],
    };

    pub fn new(position: vec2f, rotation: Quat, scale: size2f, color: Rgba<f32>, texture_coord: AtlasTextureCoord) -> Self {
        let Mat3 { x: rx, y: ry, .. } = rotation.to_axes();

        Self {
            model_0: vec2f::new(rx.x * scale.width, ry.x * scale.height),
            model_1: vec2f::new(rx.y * scale.width, ry.y * scale.height),
            model_2: position,
            color,
            uv_t: texture_coord.translation,
            uv_s: texture_coord.scale,
            scale,
        }
    }
}
