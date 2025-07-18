use bytemuck::{Pod, Zeroable};

use crate::matrix::{mat4f, Mat4};
use crate::size::size2u;
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
        Self {
            left,
            top,
            right,
            bottom,
            near,
            far,
        }
    }
}

impl From<size2u> for Orthographic {
    fn from(value: size2u) -> Self {
        Self::new(0.0, 0.0, value.width as f32, value.height as f32, -100.0, 100.0)
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
