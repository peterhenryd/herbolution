use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::matrix::{mat3f, Mat3};
use crate::rotation::euler::Euler;
use crate::vector::{vec4f, Vec3, Vec4};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Pod, Zeroable)]
pub struct Quat(pub vec4f);

impl Quat {
    pub const IDENTITY: Self = Self(Vec4::W);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Vec4::new(x, y, z, w))
    }

    pub fn from_euler(euler: Euler<f32>) -> Self {
        let (sx, cx) = (euler.yaw / 2.0).sin_cos();
        let (sy, cy) = (euler.pitch / 2.0).sin_cos();
        let (sz, cz) = (euler.roll / 2.0).sin_cos();

        Self(vec4f::new(
            sx * cy * cz - cx * sy * sz,
            cx * sy * cz + sx * cy * sz,
            cx * cy * sz - sx * sy * cz,
            cx * cy * cz + sx * sy * sz,
        ))
    }

    pub fn to_matrix(self) -> mat3f {
        let Vec4 { x, y, z, w } = self.0;
        Mat3::new(
            Vec3::new(1. - 2. * (y * y + z * z), 2. * (x * y + z * w), 2. * (x * z - y * w)),
            Vec3::new(2. * (x * y - z * w), 1. - 2. * (x * x + z * z), 2. * (y * z + x * w)),
            Vec3::new(2. * (x * z + y * w), 2. * (y * z - x * w), 1. - 2. * (x * x + y * y)),
        )
    }
}

impl From<Euler<f32>> for Quat {
    fn from(value: Euler<f32>) -> Self {
        Self::from_euler(value)
    }
}

impl From<Quat> for mat3f {
    fn from(value: Quat) -> Self {
        value.to_matrix()
    }
}
