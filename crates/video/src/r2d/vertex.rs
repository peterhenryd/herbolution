use bytemuck::{Pod, Zeroable};
use wgpu::{vertex_attr_array, VertexBufferLayout, VertexStepMode};
use math::color::Rgba;
use math::vector::{vec2f, vec4f};
use crate::mem::Payload;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Vertex2d {
    pub position: vec2f,
    pub uv: vec2f,
    pub color: Rgba<f32>,
}

impl Vertex2d {
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Vertex2d>() as u64,
        step_mode: VertexStepMode::Vertex,
        attributes: &vertex_attr_array![
            0 => Float32x2, // position
            1 => Float32x2, // uv
            2 => Float32x4, // color
        ],
    };
}

impl Payload for Vertex2d {
    type Source = Vertex2d;

    fn from_source(source: &Self::Source) -> Self {
        *source
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instance2d {
    pub position: vec2f,
}

impl Instance2d {
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Instance2dPayload>() as u64,
        step_mode: VertexStepMode::Instance,
        attributes: &vertex_attr_array![
            3 => Float32x4, // model matrix column 0
            4 => Float32x4, // model matrix column 1
            5 => Float32x4, // model matrix column 2
            6 => Float32x2, // texture translation
            7 => Float32x2, // texture scale
        ],
    };
    
    pub fn payload(&self) -> Instance2dPayload {
        Instance2dPayload {
            model_0: Default::default(),
            model_1: Default::default(),
            model_2: Default::default(),
            uv_t: Default::default(),
            uv_s: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Instance2dPayload {
    model_0: vec4f,
    model_1: vec4f,
    model_2: vec4f,
    uv_t: vec2f,
    uv_s: vec2f,
}

impl Payload for Instance2dPayload {
    type Source = Instance2d;

    fn from_source(source: &Self::Source) -> Self {
        source.payload()
    }
}