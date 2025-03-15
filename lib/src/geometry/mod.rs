use bytemuck::{Pod, Zeroable};
use math::color::Rgba;
use math::matrix::mat4f;

pub mod cuboid;
pub mod plane;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct InstanceShaderPayload {
    pub model_matrix: mat4f,
    pub texture_index: u32,
    pub color: Rgba<f32>,
}