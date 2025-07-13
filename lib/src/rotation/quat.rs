use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::matrix::{mat3f, Mat3};
use crate::rotation::Euler;
use crate::vector::{vec3f, vec4f, Vec3, Vec4};

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

        Self(Vec4::new(
            sx * cy * cz - cx * sy * sz,
            cx * sy * cz + sx * cy * sz,
            cx * cy * sz - sx * sy * cz,
            cx * cy * cz + sx * sy * sz,
        ))
    }

    pub fn from_axes(axes: mat3f) -> Self {
        let Mat3 { x, y, z } = axes;

        let trace = x.x + y.y + z.z;
        if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0;
            Self(Vec4::new((y.z - z.y) / s, (z.x - x.z) / s, (x.y - y.x) / s, 0.25 * s))
        } else if x.x > y.y && x.x > z.z {
            let s = (1.0 + x.x - y.y - z.z).sqrt() * 2.0;
            Self(Vec4::new(0.25 * s, (y.x + x.y) / s, (z.x + x.z) / s, (y.z - z.y) / s))
        } else if y.y > z.z {
            let s = (1.0 + y.y - x.x - z.z).sqrt() * 2.0;
            Self(Vec4::new((x.y + y.x) / s, 0.25 * s, (z.y + y.z) / s, (z.x - x.z) / s))
        } else {
            let s = (1.0 + z.z - x.x - y.y).sqrt() * 2.0;
            Self(Vec4::new((x.z + z.x) / s, (y.z + z.y) / s, 0.25 * s, (x.y - y.x) / s))
        }
    }

    pub fn to_axes(self) -> mat3f {
        let Vec4 { x, y, z, w } = self.0;
        Mat3::new(
            Vec3::new(1. - 2. * (y * y + z * z), 2. * (x * y + z * w), 2. * (x * z - y * w)),
            Vec3::new(2. * (x * y - z * w), 1. - 2. * (x * x + z * z), 2. * (y * z + x * w)),
            Vec3::new(2. * (x * z + y * w), 2. * (y * z - x * w), 1. - 2. * (x * x + y * y)),
        )
    }

    pub fn look_to(dir: vec3f, up: vec3f) -> Self {
        Self::from_axes(Mat3::look_to(dir, up))
    }
}

impl From<Euler<f32>> for Quat {
    fn from(value: Euler<f32>) -> Self {
        Self::from_euler(value)
    }
}

impl From<Quat> for mat3f {
    fn from(value: Quat) -> Self {
        value.to_axes()
    }
}
