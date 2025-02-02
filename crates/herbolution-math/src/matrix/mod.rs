#![allow(non_camel_case_types)]

use crate::matrix::n4::Matrix4;
use crate::quat::Quat;
use crate::vector::{vec3f, vec4f};

pub mod n3;
pub mod n4;

pub type mat3u8 = Matrix4<u8>;
pub type mat3u16 = Matrix4<u16>;
pub type mat3u = Matrix4<u32>;
pub type mat3u64 = Matrix4<u64>;
pub type mat3u128 = Matrix4<u128>;
pub type mat3usize = Matrix4<usize>;
pub type mat3i8 = Matrix4<i8>;
pub type mat3i16 = Matrix4<i16>;
pub type mat3i = Matrix4<i32>;
pub type mat3i64 = Matrix4<i64>;
pub type mat3i128 = Matrix4<i128>;
pub type mat3isize = Matrix4<isize>;
pub type mat3f = Matrix4<f32>;
pub type mat3d = Matrix4<f64>;
pub type mat3<T> = Matrix4<T>;

pub type mat4u8 = Matrix4<u8>;
pub type mat4u16 = Matrix4<u16>;
pub type mat4u = Matrix4<u32>;
pub type mat4u64 = Matrix4<u64>;
pub type mat4u128 = Matrix4<u128>;
pub type mat4usize = Matrix4<usize>;
pub type mat4i8 = Matrix4<i8>;
pub type mat4i16 = Matrix4<i16>;
pub type mat4i = Matrix4<i32>;
pub type mat4i64 = Matrix4<i64>;
pub type mat4i128 = Matrix4<i128>;
pub type mat4isize = Matrix4<isize>;
pub type mat4f = Matrix4<f32>;
pub type mat4d = Matrix4<f64>;
pub type mat4<T> = Matrix4<T>;

impl mat4f {
    pub fn from_translation(v: vec3f) -> Self {
        Self::new(
            vec4f::new(1.0, 0.0, 0.0, 0.0),
            vec4f::new(0.0, 1.0, 0.0, 0.0),
            vec4f::new(0.0, 0.0, 1.0, 0.0),
            vec4f::new(v.x, v.y, v.z, 1.0)
        )
    }

    pub fn from_quat(quat: Quat) -> Self {
        let Quat(vec4f { x, y, z, w }) = quat;
        let (x2, y2, z2) = (x + x, y + y, z + z);
        let (xx, xy, xz, xw) = (x * x2, x * y2, x * z2, x * w);
        let (yy, yz, yw) = (y * y2, y * z2, y * w);
        let (zz, zw) = (z * z2, z * w);

        Self::new(
            vec4f::new(1.0 - (yy + zz), xy + zw, xz - yw, 0.0),
            vec4f::new(xy - zw, 1.0 - (xx + zz), yz + xw, 0.0,),
            vec4f::new(xz + yw, yz - xw, 1.0 - (xx + yy), 0.0),
            vec4f::new(0.0, 0.0, 0.0, 1.0)
        )
    }
}