#![allow(non_camel_case_types)]

use crate::vector::n2::Vector2;
use crate::vector::n3::Vector3;
use crate::vector::n4::Vector4;

mod n2;
mod n3;
mod n4;

pub type vec2i8 = Vector2<i8>;
pub type vec2i16 = Vector2<i16>;
pub type vec2i = Vector2<i32>;
pub type vec2i64 = Vector2<i64>;
pub type vec2i128 = Vector2<i128>;
pub type vec2isize = Vector2<isize>;
pub type vec2u8 = Vector2<u8>;
pub type vec2u16 = Vector2<u16>;
pub type vec2u = Vector2<u32>;
pub type vec2u64 = Vector2<u64>;
pub type vec2u128 = Vector2<u128>;
pub type vec2usize = Vector2<usize>;
pub type vec2f = Vector2<f32>;
pub type vec2d = Vector2<f64>;
pub type vec2<T> = Vector2<T>;

pub type vec3i8 = Vector3<i8>;
pub type vec3i16 = Vector3<i16>;
pub type vec3i = Vector3<i32>;
pub type vec3i64 = Vector3<i64>;
pub type vec3i128 = Vector3<i128>;
pub type vec3isize = Vector3<isize>;
pub type vec3u8 = Vector3<u8>;
pub type vec3u16 = Vector3<u16>;
pub type vec3u = Vector3<u32>;
pub type vec3u64 = Vector3<u64>;
pub type vec3u128 = Vector3<u128>;
pub type vec3usize = Vector3<usize>;
pub type vec3f = Vector3<f32>;
pub type vec3d = Vector3<f64>;
pub type vec3<T> = Vector3<T>;

pub type vec4i8 = Vector4<i8>;
pub type vec4i16 = Vector4<i16>;
pub type vec42i = Vector4<i32>;
pub type vec4i64 = Vector4<i64>;
pub type vec4i128 = Vector4<i128>;
pub type vec4isize = Vector4<isize>;
pub type vec4u8 = Vector4<u8>;
pub type vec4u16 = Vector4<u16>;
pub type vec4u = Vector4<u32>;
pub type vec4u64 = Vector4<u64>;
pub type vec4u128 = Vector4<u128>;
pub type vec4usize = Vector4<usize>;
pub type vec4f = Vector4<f32>;
pub type vec4d = Vector4<f64>;
pub type vec4<T> = Vector4<T>;