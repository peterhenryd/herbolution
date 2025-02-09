use bytemuck::{Pod, Zeroable};
use crate::engine::as_no_uninit::AsNoUninit;
use crate::world::camera::proj::Proj;
use crate::world::transform::Transform;
use math::matrix::{mat4, mat4f, ArrMat4F32};
use math::vector::{vec3, vec4, ArrVec3F32};

pub mod proj;
pub mod frustum;

pub struct Camera<P> {
    pub transform: Transform,
    pub proj: P,
}

impl<P> Camera<P> {
    pub fn new(view: Transform, proj: P) -> Self {
        Self { transform: view, proj }
    }
}

impl<P: Proj> Camera<P> {
    pub fn as_mat4f(&self) -> mat4f {
        self.proj.as_mat4f() * self.transform.as_view_mat4f()
    }
}

impl<P: Proj> AsNoUninit for Camera<P> {
    type Output = ArrCamera;

    fn as_no_uninit(&self) -> Self::Output {
        ArrCamera(self.as_mat4f().into(), self.transform.position.into(), 0)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ArrCamera(pub ArrMat4F32, pub ArrVec3F32, u32);

impl Transform {
    pub fn as_view_mat4f(&self) -> mat4f {
        let f = -self.rotation.into_center().cast();
        let s = f.cross(vec3::y()).normalize();
        let u = s.cross(f);

        mat4::new(
            vec4::new(s.x, u.x, -f.x, 0.0),
            vec4::new(s.y, u.y, -f.y, 0.0),
            vec4::new(s.z, u.z, -f.z, 0.0),
            vec4::new(-self.position.dot(s), -self.position.dot(u), self.position.dot(f), 1.0),
        )
    }
}