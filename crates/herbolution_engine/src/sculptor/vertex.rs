use bytemuck::{Pod, Zeroable};
use gpu::{vertex_attr_array, Payload, Vertex, VertexBufferLayout, VertexStepMode};
use math::color::Rgba;
use math::rotation::Quat;
use math::vector::{vec2f, vec3d, vec3f, vec3i, vec4f, Vec3};
use serde::{Deserialize, Serialize};

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

impl Payload for Vertex3d {
    type Source = Vertex3d;

    fn from_source(source: &Self::Source) -> Self {
        *source
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instance3d {
    pub position: vec3d,
    pub rotation: Quat,
    pub color: Rgba<f32>,
    pub light: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Instance3dPayload {
    pub model_0: vec4f,
    pub model_1: vec4f,
    pub model_2: vec4f,
    pub position_int: vec3i,
    pub position_fract: vec3f,
    pub color: Rgba<f32>,
    pub light: u32,
}

impl Instance3d {
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Instance3dPayload>() as u64,
        step_mode: VertexStepMode::Instance,
        attributes: &vertex_attr_array![
            3 => Float32x4,
            4 => Float32x4,
            5 => Float32x4,
            6 => Sint32x3,
            7 => Float32x3,
            8 => Float32x4,
            9 => Uint32,
        ],
    };

    pub fn payload(&self) -> Instance3dPayload {
        let rotation_matrix = self.rotation.to_matrix();
        let (integral, fractional) = self.position.split_int_fract();

        Instance3dPayload {
            model_0: rotation_matrix.x.extend(0.0),
            model_1: rotation_matrix.y.extend(0.0),
            model_2: rotation_matrix.z.extend(0.0),
            position_int: integral,
            position_fract: fractional,
            color: self.color,
            light: self.light,
        }
    }
}

impl Default for Instance3d {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            color: Rgba::TRANSPARENT,
            light: 0,
        }
    }
}

impl Payload for Instance3dPayload {
    type Source = Instance3d;

    fn from_source(source: &Self::Source) -> Self {
        source.payload()
    }
}
