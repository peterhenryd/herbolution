#![allow(non_camel_case_types)]

pub mod n4;

pub use n4::Mat4;

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
