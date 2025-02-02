pub mod projection;

use bytemuck::NoUninit;
use winit::dpi::PhysicalSize;
use math::vector::vec3f;
use serde::{Deserialize, Serialize};
use crate::camera::projection::{Perspective, Projection};
use crate::gpu::uniform::UniformObject;

pub struct Camera<P> {
    pub view: View,
    pub projection: P,
}

impl Camera<Perspective> {
    pub fn perspective(position: vec3f, rotation: ViewRotation, size: PhysicalSize<u32>) -> Self {
        Self {
            projection: Perspective::from(size),
            view: View { position, rotation },
        }
    }
}

impl<P: Projection> UniformObject for Camera<P> {
    fn get_raw(&self) -> impl NoUninit {
        self.projection.as_projection_matrix() * self.view.as_matrix()
    }
}

pub struct View {
    pub position: vec3f,
    pub rotation: ViewRotation,
}

impl View {
    pub fn as_matrix(&self) -> Mat4 {
        Mat4::look_to_lh(self.position, self.rotation.into_center(), vec3f::Y)
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct ViewRotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for ViewRotation {
    fn default() -> Self {
        Self {
            yaw: 0.,
            pitch: 90_f32.to_radians(),
        }
    }
}

impl ViewRotation {
    pub fn into_center(self) -> vec3f {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        vec3f::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize()
    }

    pub fn into_directions(self) -> (vec3f, vec3f) {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        let straight = vec3f::new(cos_yaw, 0.0, sin_yaw);
        let side = vec3f::new(-sin_yaw, 0.0, cos_yaw);

        (straight.normalize(), side.normalize())
    }
}
