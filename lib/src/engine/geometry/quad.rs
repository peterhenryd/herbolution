use crate::engine::as_no_uninit::AsNoUninit;
use bytemuck::{Pod, Zeroable};
use math::matrix::{mat4, mat4f, ArrMat4F32};
use math::quat::Quat;
use math::vector::{vec3f, ArrVec2F32, ArrVec3F32};
use crate::engine::geometry::vertex::ArrVertex;
use crate::engine::gpu::Gpu;
use crate::engine::mesh::Mesh;

const N: f32 = 8.0;

pub struct Quad {
    pub position: vec3f,
    pub rotation: Quat,
    pub texture_index: u32,
}

impl Quad {
    const VERTICES: [ArrVertex; 4] = [
        ArrVertex(ArrVec3F32([-0.5, 0.5, 0.5]), ArrVec2F32([0.0, 0.0])),
        ArrVertex(ArrVec3F32([0.5, 0.5, 0.5]), ArrVec2F32([1.0 / N, 0.0])),
        ArrVertex(ArrVec3F32([-0.5, -0.5, 0.5]), ArrVec2F32([0.0, 1.0 / N])),
        ArrVertex(ArrVec3F32([0.5, -0.5, 0.5]), ArrVec2F32([1.0 / N, 1.0 / N])),
    ];
    const INDICES: [u16; 6] = [0, 2, 1, 3, 1, 2];

    pub fn create_mesh(gpu: &Gpu) -> Mesh {
        Mesh::create(gpu, &Self::VERTICES, &Self::INDICES)
    }

    pub fn as_mat4f(&self) -> mat4f {
        mat4::from_translation(self.position) * self.rotation.into_mat4f()
    }
}

impl AsNoUninit for Quad {
    type Output = RawQuad;

    fn as_no_uninit(&self) -> Self::Output {
        RawQuad(self.as_mat4f().into(), self.texture_index)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct RawQuad(pub ArrMat4F32, pub u32);