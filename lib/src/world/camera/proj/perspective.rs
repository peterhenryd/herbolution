use crate::world::camera::proj::Proj;
use math::angle::{Deg, Rad};
use math::matrix::{mat4, mat4f};
use math::vector::vec4;
use winit::dpi::PhysicalSize;

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

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.aspect = size.width as f32 / size.height as f32;
    }
}

impl Proj for Perspective {
    fn as_mat4f(&self) -> mat4f {
        let (sin_fov, cos_fov) = (self.fov_y.0 * 0.5).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / self.aspect;
        let r = self.z_far / (self.z_far - self.z_near);
        mat4::new(
            vec4::new(w, 0.0, 0.0, 0.0),
            vec4::new(0.0, h, 0.0, 0.0),
            vec4::new(0.0, 0.0, r, 1.0),
            vec4::new(0.0, 0.0, -r * self.z_near, 0.0),
        )
    }
}

impl From<PhysicalSize<u32>> for Perspective {
    fn from(PhysicalSize { width, height }: PhysicalSize<u32>) -> Self {
        Self::new(
            Deg(90.0),
            width as f32 / height as f32,
            0.001,
            500.0,
        )
    }
}