use bytemuck::{Pod, Zeroable};
use math::matrix::{mat4f, Mat4};
use math::proj::Proj;
use math::rotation::Euler;
use math::vector::{vec3d, vec3f, vec3i, vec3if};

#[derive(Debug, Clone, PartialEq)]
pub struct Camera<P> {
    pub position: vec3d,
    pub rotation: Euler<f32>,
    pub proj: P
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraPayload {
    pub view_proj: mat4f,
    pub position: vec3f,
    pub _padding_0: u32,
    pub position_int: vec3i,
    pub _padding_1: u32,
    pub position_fract: vec3f,
    pub _padding_2: u32,
}

impl<P> Camera<P> {
    pub fn new(position: vec3d, rotation: Euler<f32>, proj: P) -> Self {
        Self {
            position,
            rotation,
            proj,
        }
    }

    pub fn view_proj(&self) -> mat4f
    where
        P: Proj,
    {
        self.proj.to_matrix() * Mat4::view_origin(self.rotation)
    }

    pub fn payload(&self) -> CameraPayload
    where
        P: Proj,
    {
        let vec3if { integral, fractional } = self.position.into();
        CameraPayload {
            view_proj: self.view_proj(),
            position: self.position.cast().unwrap(),
            _padding_0: 0,
            position_int: integral,
            _padding_1: 0,
            position_fract: fractional,
            _padding_2: 0,
        }
    }
}