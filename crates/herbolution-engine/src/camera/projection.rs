use winit::dpi::PhysicalSize;

pub trait Projection {
    fn as_projection_matrix(&self) -> m;

    fn resize(&mut self, size: PhysicalSize<u32>);
}

pub struct Perspective {
    pub fov_y: f32,
    pub aspect: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl From<PhysicalSize<u32>> for Perspective {
    fn from(size: PhysicalSize<u32>) -> Self {
        Self {
            fov_y: 45_f32.to_radians(),
            aspect: size.width as f32 / size.height as f32,
            z_near: 0.1,
            z_far: 100.0,
        }
    }
}

impl Projection for Perspective {
    fn as_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_lh(self.fov_y, self.aspect, self.z_near, self.z_far)
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.aspect = size.width as f32 / size.height as f32;
    }
}

pub struct Orthographic {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Projection for Orthographic {
    fn as_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(self.left, self.right, self.bottom, self.top, 0.0, 1.0)
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.top = size.height as f32 / 2.0;
        self.bottom = -self.top;
        self.left = -(size.width as f32) / 2.0;
        self.right = -self.left;
    }
}
