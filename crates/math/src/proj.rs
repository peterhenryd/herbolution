use bytemuck::{Pod, Zeroable};
use crate::matrix::{mat4f, Mat4};
use crate::size::Size2;
use crate::vector::Vec4;

pub trait Proj {
    fn to_matrix(&self) -> mat4f;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Orthographic {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Perspective {
    pub fov_y: f32,
    pub aspect: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Orthographic {
    pub const fn new(left: f32, top: f32, right: f32, bottom: f32, near: f32, far: f32) -> Self {
        Self { left, top, right, bottom, near, far }
    }
}

impl From<Size2<u32>> for Orthographic {
    fn from(value: Size2<u32>) -> Self {
        let aspect = value.width as f32 / value.height as f32;
        Self::new(-aspect / 2.0, -0.5, aspect / 2.0, 0.5, -1.0, 1.0)
    }
}

impl Proj for Orthographic {
    fn to_matrix(&self) -> mat4f {
        let rcp_width = 1.0 / (self.right - self.left);
        let rcp_height = 1.0 / (self.top - self.bottom);
        let r = 1.0 / (self.near - self.far);
        Mat4::new(
            Vec4::new(rcp_width + rcp_width, 0.0, 0.0, 0.0),
            Vec4::new(0.0, rcp_height + rcp_height, 0.0, 0.0),
            Vec4::new(0.0, 0.0, r, 0.0),
            Vec4::new(
                -(self.left + self.right) * rcp_width,
                -(self.top + self.bottom) * rcp_height,
                r * self.near,
                1.0,
            ),
        )
    }
}

impl Perspective {
    pub fn new(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> Self {
        Self { fov_y, aspect, z_near, z_far }
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.aspect = size.width as f32 / size.height as f32;
    }
}

impl Proj for Perspective {
    fn to_matrix(&self) -> mat4f {
        let (sin_fov, cos_fov) = (self.fov_y * 0.5).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / self.aspect;
        let r = self.z_far / (self.z_far - self.z_near);
        Mat4::new(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, r, 1.0),
            Vec4::new(0.0, 0.0, -r * self.z_near, 0.0),
        )
    }
}