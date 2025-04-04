use crate::gpu::mem::payload::ShaderPayload;
use bytemuck::{Pod, Zeroable};
use math::angle::Rad;
use math::matrix::{mat4f, Mat4};
use math::projection::Projection;
use math::rotation::Euler;
use math::vector::vec3f;

pub mod frustum;

pub struct Camera<P> {
    pub position: vec3f,
    pub rotation: Euler<Rad<f32>>,
    pub projection: P,
}

impl<P> Camera<P> {
    pub fn new(position: vec3f, proj: P) -> Self {
        Self {
            position,
            rotation: Euler::IDENTITY,
            projection: proj,
        }
    }
}

impl<P: Projection> Camera<P> {
    pub fn view_projection_matrix(&self) -> mat4f {
        self.projection.to_matrix() * Mat4::view(self.position, self.rotation)
    }
}

impl<P: Projection> ShaderPayload for Camera<P> {
    type Output<'a> = CameraShaderPayload where Self: 'a;

    fn payload(&self) -> Self::Output<'_> {
        CameraShaderPayload {
            view_projection_matrix: self.view_projection_matrix(),
            world_position: self.position,
            _padding: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraShaderPayload {
    pub view_projection_matrix: mat4f,
    pub world_position: vec3f,
    pub _padding: u32,
}
