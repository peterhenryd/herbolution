use bytemuck::{Pod, Zeroable};
use math::mat::{Mat4, mat4f};
use math::proj::Proj;
use math::rotation::Euler;
use math::vec::{Vec3, vec3d, vec3f, vec3if, vec4f, vec4i};

#[derive(Debug, Clone, PartialEq)]
pub struct Camera<P> {
    pub position: vec3d,
    pub view: View,
    pub proj: P,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum View {
    Rotate { rotation: Euler<f32> },
    Forward,
}

impl View {
    pub fn rotatable() -> Self {
        View::Rotate { rotation: Euler::IDENTITY }
    }

    pub fn rotation(&mut self) -> Option<&mut Euler<f32>> {
        match self {
            View::Rotate { rotation } => Some(rotation),
            View::Forward => None,
        }
    }

    pub fn get_eye(self) -> vec3f {
        match self {
            View::Rotate { .. } => Vec3::ZERO,
            View::Forward => -Vec3::Z,
        }
    }

    pub fn get_dir(self) -> vec3f {
        match self {
            View::Rotate { rotation } => rotation.into_view_center(),
            View::Forward => Vec3::Z,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraPayload {
    view_proj: mat4f,
    position: vec4f,
    position_int: vec4i,
    position_fract: vec4f,
}

impl<P> Camera<P> {
    pub fn new(position: vec3d, view: View, proj: P) -> Self {
        Self { position, view, proj }
    }

    pub fn view_proj(&self) -> mat4f
    where
        P: Proj,
    {
        self.proj.to_matrix() * Mat4::look_to(self.view.get_eye(), self.view.get_dir(), Vec3::Y)
    }

    pub fn payload(&self) -> CameraPayload
    where
        P: Proj,
    {
        let vec3if { integral, fractional } = self.position.into();
        CameraPayload {
            view_proj: self.view_proj(),
            position: self
                .position
                .cast()
                .unwrap()
                .extend(0.0),
            position_int: integral.extend(0),
            position_fract: fractional.extend(0.0),
        }
    }
}
