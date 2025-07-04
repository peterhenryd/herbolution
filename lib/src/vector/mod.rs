#![allow(non_camel_case_types)]

mod macros;

crate::reexport! {
    mod vec2;
    mod vec3;
    mod vec4;
}

pub type vec2u8 = Vec2<u8>;
pub type vec2u16 = Vec2<u16>;
pub type vec2u = Vec2<u32>;
pub type vec2u64 = Vec2<u64>;
pub type vec2u128 = Vec2<u128>;
pub type vec2usize = Vec2<usize>;
pub type vec2i8 = Vec2<i8>;
pub type vec2i16 = Vec2<i16>;
pub type vec2i = Vec2<i32>;
pub type vec2i64 = Vec2<i64>;
pub type vec2i128 = Vec2<i128>;
pub type vec2isize = Vec2<isize>;
pub type vec2f = Vec2<f32>;
pub type vec2d = Vec2<f64>;

pub type vec3u8 = Vec3<u8>;
pub type vec3u16 = Vec3<u16>;
pub type vec3u = Vec3<u32>;
pub type vec3u64 = Vec3<u64>;
pub type vec3u128 = Vec3<u128>;
pub type vec3usize = Vec3<usize>;
pub type vec3i8 = Vec3<i8>;
pub type vec3i16 = Vec3<i16>;
pub type vec3i = Vec3<i32>;
pub type vec3i64 = Vec3<i64>;
pub type vec3i128 = Vec3<i128>;
pub type vec3isize = Vec3<isize>;
pub type vec3f = Vec3<f32>;
pub type vec3d = Vec3<f64>;

pub type vec4u8 = Vec4<u8>;
pub type vec4u16 = Vec4<u16>;
pub type vec4u = Vec4<u32>;
pub type vec4u64 = Vec4<u64>;
pub type vec4u128 = Vec4<u128>;
pub type vec4usize = Vec4<usize>;
pub type vec4i8 = Vec4<i8>;
pub type vec4i16 = Vec4<i16>;
pub type vec4i = Vec4<i32>;
pub type vec4i64 = Vec4<i64>;
pub type vec4i128 = Vec4<i128>;
pub type vec4isize = Vec4<isize>;
pub type vec4f = Vec4<f32>;
pub type vec4d = Vec4<f64>;
