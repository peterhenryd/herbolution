use bytemuck::{Pod, Zeroable};
use gpu::{vertex_attr_array, AtlasTextureCoord, Vertex, VertexBufferLayout, VertexStepMode};
use math::color::Rgba;
use math::matrix::Mat3;
use math::rotation::Quat;
use math::vector::{vec2f, vec3f};

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

#[derive(Debug, Copy, Clone)]
pub struct Instance2d {
    pub position: vec2f,
    pub rotation: Quat,
    pub scale: vec2f,
    pub color: Rgba<f32>,
    pub texture_coord: AtlasTextureCoord,
}

impl Instance2d {
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Instance2dPayload>() as u64,
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

    pub fn payload(&self) -> Instance2dPayload {
        let Mat3 { x: rx, y: ry, .. } = self.rotation.to_axes();

        Instance2dPayload {
            model_0: vec2f::new(rx.x * self.scale.x, ry.x * self.scale.y),
            model_1: vec2f::new(rx.y * self.scale.x, ry.y * self.scale.y),
            model_2: self.position,
            color: self.color,
            uv_t: self.texture_coord.translation,
            uv_s: self.texture_coord.scale,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Instance2dPayload {
    model_0: vec2f,
    model_1: vec2f,
    model_2: vec2f,
    color: Rgba<f32>,
    uv_t: vec2f,
    uv_s: vec2f,
}
