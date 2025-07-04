use bytemuck::{Pod, Zeroable};
use lib::matrix::{Mat4, mat4f};
use lib::proj::Proj;
use lib::rotation::Euler;
use lib::vector::{Vec3, vec3d, vec3f, vec4f, vec4i};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct VideoCamera {
    view_proj: mat4f,
    position: vec4f,
    position_int: vec4i,
    position_fract: vec4f,
}

impl VideoCamera {
    pub fn new<P>(position: vec3d, view: View, proj: P) -> Self
    where
        P: Proj,
    {
        let (position_int, position_fract) = position.split_int_fract();

        Self {
            view_proj: proj.to_matrix() * Mat4::look_to(view.get_eye(), view.get_dir(), Vec3::Y),
            position: position.cast().extend(0.0),
            position_int: position_int.extend(0),
            position_fract: position_fract.extend(0.0),
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

    pub fn get_eye(self) -> vec3f {
        match self {
            View::Rotate { .. } => Vec3::ZERO,
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
