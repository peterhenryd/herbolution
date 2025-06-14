use bytemuck::{Pod, Zeroable};
use gpu::pipeline::{vertex_attr_array, VertexBufferLayout, VertexStepMode};
use gpu::{AtlasTextureCoord, Payload, Vertex};
use math::color::Rgba;
use math::rotation::Quat;
use math::vector::{vec2f, vec3d, vec3f, vec3i, vec3if, vec4f, Vec3};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
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
    pub texture_coord: AtlasTextureCoord,
    pub color: Rgba<f32>,
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
    pub uv_t: vec2f,
    pub uv_s: vec2f,
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
            9 => Float32x2,
            10 => Float32x2,
        ],
    };

    pub fn payload(&self) -> Instance3dPayload {
        let rotation_matrix = self.rotation.to_matrix();
        let vec3if { integral, fractional } = self.position.into();

        Instance3dPayload {
            model_0: rotation_matrix.x.extend(0.0),
            model_1: rotation_matrix.y.extend(0.0),
            model_2: rotation_matrix.z.extend(0.0),
            position_int: integral,
            position_fract: fractional,
            color: self.color,
            uv_t: self.texture_coord.translation,
            uv_s: self.texture_coord.scale,
        }
    }
}

impl Default for Instance3d {
    fn default() -> Self {
        Self { position: Vec3::ZERO, rotation: Quat::IDENTITY, texture_coord: AtlasTextureCoord::NONE, color: Rgba::TRANSPARENT }
    }
}

impl Payload for Instance3dPayload {
    type Source = Instance3d;

    fn from_source(source: &Self::Source) -> Self {
        source.payload()
    }
}
