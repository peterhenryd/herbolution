use bytemuck::{Pod, Zeroable};
use gpu::pipeline::{vertex_attr_array, VertexBufferLayout, VertexStepMode};
use gpu::{AtlasTextureCoord, Payload, Vertex};
use math::color::Rgba;
use math::matrix::Mat3;
use math::rotation::Quat;
use math::vector::{vec2f, vec3f, Vec2};

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

impl Payload for Vertex2d {
    type Source = Vertex2d;

    fn from_source(source: &Self::Source) -> Self {
        *source
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instance2d {
    pub position: vec2f,
    pub rotation: Quat,
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
            5 => Float32x2, // position
            6 => Float32x4, // color
            7 => Float32x2, // texture translation
            8 => Float32x2, // texture scale
        ],
    };

    pub fn payload(&self) -> Instance2dPayload {
        let Mat3 { x: r0, y: r1, .. } = self.rotation.to_matrix();

        Instance2dPayload {
            model_0: Vec2::new(r0.x, r1.x),
            model_1: Vec2::new(r0.y, r1.y),
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

impl Payload for Instance2dPayload {
    type Source = Instance2d;

    fn from_source(source: &Self::Source) -> Self {
        source.payload()
    }
}
