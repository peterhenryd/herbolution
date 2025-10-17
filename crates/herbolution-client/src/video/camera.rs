use kiln_types::rotation::Euler;
use kiln_types::mat::Mat4;
use kiln_types::vec::Vec3;
use kiln_types::proj::Proj;
use kiln_types::vec::vec3f;
use kiln_types::vec::vec4f;
use kiln_types::mat::mat4f;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct VideoCamera {
    pub(crate) view_proj: mat4f,
    position: vec4f,
}

impl VideoCamera {
    pub fn new<P>(position: vec3f, view: View, proj: P) -> Self
    where
        P: Proj,
    {
        Self {
            view_proj: proj.to_matrix() * Mat4::look_to(view.get_eye(position), view.get_dir(), Vec3::Y),
            position: position.extend(0.0),
        }
    }
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

    pub fn get_eye(self, position: vec3f) -> vec3f {
        match self {
            View::Rotate { .. } => position,
            View::Forward => -Vec3::Z,
        }
    }

    pub fn get_dir(self) -> vec3f {
        match self {
            View::Rotate { rotation } => -rotation.into_view_center().normalize(),
            View::Forward => -Vec3::Z,
        }
    }
}
