use crate::world::geometry::vertex::WorldVertex;
use crate::world::model::Model;
use bytemuck::{Pod, Zeroable};
use wgpu::{vertex_attr_array, VertexAttribute, VertexBufferLayout};
use math::matrix::{mat4, mat4f};
use math::quat::Quat;
use math::vector::{vec2f, vec3f};
use crate::gpu::Gpu;
use crate::math::vector::Vector3;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Quad {
    pub model: [f32; 16],
    pub texture_index: u32,
}

impl Quad {
    pub fn new(Vector3 { x, y, z }: Vector3<f32>, quat: Quat, texture_index: u32) -> Self {
        let model = mat4::from_translation(vec3f::new(x, y, z)) * mat4f::from_quat(quat);
        Self {
            model: model.to_cols_array(),
            texture_index,
        }
    }
}

impl Quad {
    const ATTRIBUTES: [VertexAttribute; 5] = vertex_attr_array![
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x4,
        6 => Float32x4,
        7 => Uint32
    ];

    pub fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: 4 * 16 + 4,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }

    const VERTICES: [WorldVertex; 4] = [
        WorldVertex::new(vec3f::new(-0.5, 0.5, 0.5), vec2f::new(0., 0.)),
        WorldVertex::new(vec3f::new(0.5, 0.5, 0.5), vec2f::new(1., 0.)),
        WorldVertex::new(vec3f::new(-0.5, -0.5, 0.5), vec2f::new(0., 1.)),
        WorldVertex::new(vec3f::new(0.5, -0.5, 0.5), vec2f::new(1., 1.)),
    ];
    const INDICES: [u16; 6] = [0, 2, 1, 3, 1, 2];

    pub fn model(gpu: &Gpu) -> Model {
        Model::new(gpu, &Self::VERTICES, &Self::INDICES)
    }
}
