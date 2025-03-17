use bytemuck::{Pod, Zeroable};
use crate::angle::{Deg, Rad};
use crate::matrix::{mat4f, Mat4};
use crate::projection::Projection;
use crate::size::Size2;
use crate::vector::Vec4;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Perspective {
    pub fov_y: Rad<f32>,
    pub aspect: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Perspective {
    pub fn new(fov_y: impl Into<Rad<f32>>, aspect: f32, z_near: f32, z_far: f32) -> Self {
        Self { fov_y: fov_y.into(), aspect, z_near, z_far }
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.aspect = size.width as f32 / size.height as f32;
    }
}

impl Projection for Perspective {
    fn to_matrix(&self) -> mat4f {
        let (sin_fov, cos_fov) = (self.fov_y.0 * 0.5).sin_cos();
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

impl From<Size2<u32>> for Perspective {
    fn from(Size2 { width, height }: Size2<u32>) -> Self {
        Self::new(
            Deg(70.0),
            width as f32 / height as f32,
            0.001,
            500.0,
        )
    }
}