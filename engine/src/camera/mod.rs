use bytemuck::{Pod, Zeroable};
use math::matrix::mat4f;
use math::projection::Projection;
use math::transform::Transform;
use math::vector::vec3f;
use crate::gpu::mem::payload::ShaderPayload;

pub mod frustum;

pub struct Camera<P> {
    pub transform: Transform,
    pub projection: P,
}

impl<P> Camera<P> {
    pub fn new(view: Transform, proj: P) -> Self {
        Self {
            transform: view,
            projection: proj,
        }
    }
}

impl<P: Projection> Camera<P> {
    pub fn view_projection_matrix(&self) -> mat4f {
        self.projection.to_matrix() * self.transform.to_view_matrix()
    }
}

impl<P: Projection> ShaderPayload for Camera<P> {
    type Output<'a> = CameraShaderPayload where Self: 'a;

    fn payload(&self) -> Self::Output<'_> {
        CameraShaderPayload {
            view_projection_matrix: self.view_projection_matrix(),
            world_position: self.transform.position,
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
