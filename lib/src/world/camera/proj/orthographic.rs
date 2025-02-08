use crate::world::camera::proj::Proj;
use herbolution_math::matrix::{mat4, mat4f};
use herbolution_math::vector::vec4;

pub struct Orthographic {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
}

impl Orthographic {
    pub const fn new(left: f32, top: f32, right: f32, bottom: f32, near: f32, far: f32) -> Self {
        Self { left, top, right, bottom, near, far }
    }
}

impl Proj for Orthographic {
    fn as_mat4f(&self) -> mat4f {
        let rcp_width = 1.0 / (self.right - self.left);
        let rcp_height = 1.0 / (self.top - self.bottom);
        let r = 1.0 / (self.near - self.far);
        mat4::new(
            vec4::new(rcp_width + rcp_width, 0.0, 0.0, 0.0),
            vec4::new(0.0, rcp_height + rcp_height, 0.0, 0.0),
            vec4::new(0.0, 0.0, r, 0.0),
            vec4::new(
                -(self.left + self.right) * rcp_width,
                -(self.top + self.bottom) * rcp_height,
                r * self.near,
                1.0,
            ),
        )
    }
}