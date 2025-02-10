use math::angle::Rad;
use math::vector::{vec3, vec3d, vec3f};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: vec3f,
    pub rotation: Rotation,
}

impl Transform {
    pub const fn new(position: vec3f, rotation: Rotation) -> Self {
        Self { position, rotation }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct Rotation {
    pub yaw: Rad<f64>,
    pub pitch: Rad<f64>,
}

impl Rotation {
    pub const fn new(yaw: Rad<f64>, pitch: Rad<f64>) -> Self {
        Self { yaw, pitch }
    }
}

impl Rotation {
    pub fn into_center(self) -> vec3d {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();
        vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize()
    }

    pub fn into_directions(self) -> (vec3d, vec3d) {
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        let straight = vec3::new(cos_yaw, 0.0, sin_yaw);
        let side = vec3::new(-sin_yaw, 0.0, cos_yaw);

        (straight.normalize(), side.normalize())
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Self::new(Rad(0.0), Rad(-90.0))
    }
}