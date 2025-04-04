use crate::angle::Angle;
use crate::matrix::{mat4f, Mat4};
use crate::rotation::euler::Euler;
use crate::vector::{vec4f, Vec4};
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(
    Debug, Default, Copy, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Pod, Zeroable,
)]
pub struct Quat(vec4f);

impl Quat {
    pub const IDENTITY: Self = Self(Vec4::W);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Vec4::new(x, y, z, w))
    }

    pub fn from_euler<A: Angle<Comp = f32>>(euler: Euler<A>) -> Self {
        let (sx, cx) = (euler.yaw.into_rad() / 2.0).0.sin_cos();
        let (sy, cy) = (euler.pitch.into_rad() / 2.0).0.sin_cos();
        let (sz, cz) = (euler.roll.into_rad() / 2.0).0.sin_cos();

        Self(vec4f::new(
            sx * cy * cz - cx * sy * sz,
            cx * sy * cz + sx * cy * sz,
            cx * cy * sz - sx * sy * cz,
            cx * cy * cz + sx * sy * sz,
        ))
    }

    pub fn into_matrix(self) -> mat4f {
        let Vec4 { x, y, z, w } = self.0;
        Mat4::new(
            Vec4::new(
                1. - 2. * (y * y + z * z),
                2. * (x * y + z * w),
                2. * (x * z - y * w),
                0.,
            ),
            Vec4::new(
                2. * (x * y - z * w),
                1. - 2. * (x * x + z * z),
                2. * (y * z + x * w),
                0.,
            ),
            Vec4::new(
                2. * (x * z + y * w),
                2. * (y * z - x * w),
                1. - 2. * (x * x + y * y),
                0.,
            ),
            Vec4::new(0., 0., 0., 1.),
        )
    }
}

impl<A: Angle<Comp = f32>> From<Euler<A>> for Quat {
    fn from(value: Euler<A>) -> Self {
        Self::from_euler(value)
    }
}

impl From<Quat> for mat4f {
    fn from(value: Quat) -> Self {
        value.into_matrix()
    }
}
