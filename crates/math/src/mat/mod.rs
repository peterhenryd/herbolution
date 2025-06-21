#![allow(non_camel_case_types)]

mod mat3;
mod mat4;

pub use mat3::Mat3;

pub type mat3u8 = Mat3<u8>;
pub type mat3u16 = Mat3<u16>;
pub type mat3u = Mat3<u32>;
pub type mat3u64 = Mat3<u64>;
pub type mat3u128 = Mat3<u128>;
pub type mat3usize = Mat3<usize>;
pub type mat3i8 = Mat3<i8>;
pub type mat3i16 = Mat3<i16>;
pub type mat3i = Mat3<i32>;
pub type mat3i64 = Mat3<i64>;
pub type mat3i128 = Mat3<i128>;
pub type mat3isize = Mat3<isize>;
pub type mat3f = Mat3<f32>;
pub type mat3d = Mat3<f64>;

pub use mat4::Mat4;

pub type mat4u8 = Mat4<u8>;
pub type mat4u16 = Mat4<u16>;
pub type mat4u = Mat4<u32>;
pub type mat4u64 = Mat4<u64>;
pub type mat4u128 = Mat4<u128>;
pub type mat4usize = Mat4<usize>;
pub type mat4i8 = Mat4<i8>;
pub type mat4i16 = Mat4<i16>;
pub type mat4i = Mat4<i32>;
pub type mat4i64 = Mat4<i64>;
pub type mat4i128 = Mat4<i128>;
pub type mat4isize = Mat4<isize>;
pub type mat4f = Mat4<f32>;
pub type mat4d = Mat4<f64>;
