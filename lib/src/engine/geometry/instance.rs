use bytemuck::{Pod, Zeroable};
use math::matrix::{mat4, mat4f, ArrMat4F32};
use math::quat::Quat;
use math::vector::vec3f;
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};
use math::as_no_uninit::AsNoUninit;

pub struct Instance {
    pub position: vec3f,
    pub rotation: Quat,
    pub texture_index: u32,
    pub light_level_alpha: f32,
}

impl Instance {
    pub(super) const LAYOUT: VertexBufferLayout<'_> = VertexBufferLayout {
        array_stride: size_of::<ArrInstance>() as BufferAddress,
        step_mode: VertexStepMode::Instance,
        attributes: &Self::ATTRIBUTES,
    };
    const ATTRIBUTES: [VertexAttribute; 6] = vertex_attr_array![
        2 => Float32x4,
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x4,
        6 => Uint32,
        7 => Float32,
    ];

    pub fn get_model_mat4f(&self) -> mat4f {
        mat4::from_translation(self.position) * self.rotation.into_mat4f()
    }
}

impl AsNoUninit for Instance {
    type Output = ArrInstance;

    fn as_no_uninit(&self) -> Self::Output {
        ArrInstance(self.get_model_mat4f().into(), self.texture_index, self.light_level_alpha)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrInstance(pub ArrMat4F32, pub u32, pub f32);