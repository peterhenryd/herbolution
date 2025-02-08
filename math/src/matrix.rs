#![allow(non_camel_case_types)]

use crate::vector::{vec2, vec3, vec4};

#[derive(derive::Matrix)]
pub struct mat2<T> {
    pub x: vec2<T>,
    pub y: vec2<T>,
}

#[derive(derive::Matrix)]
pub struct mat3<T> {
    pub x: vec3<T>,
    pub y: vec3<T>,
    pub z: vec3<T>,
}

#[derive(derive::Matrix)]
pub struct mat4<T> {
    pub x: vec4<T>,
    pub y: vec4<T>,
    pub z: vec4<T>,
    pub w: vec4<T>,
}

impl<A: num::ToPrimitive> mat4<A> {
    pub fn cast<B: num::NumCast>(self) -> mat4<B> {
        mat4 {
            x: self.x.cast(),
            y: self.y.cast(),
            z: self.z.cast(),
            w: self.w.cast(),
        }
    }
}

impl<T: num::Num> mat4<T> {
    pub fn from_translation(vec3 { x, y, z }: vec3<T>) -> Self {
        Self {
            x: vec4::x(),
            y: vec4::y(),
            z: vec4::z(),
            w: vec4::new(x, y, z, T::one()),
        }
    }
}

impl<T: num::Num> From<mat3<T>> for mat4<T> {
    fn from(value: mat3<T>) -> Self {
        Self {
            x: value.x.extend(T::zero()),
            y: value.y.extend(T::zero()),
            z: value.z.extend(T::zero()),
            w: vec4::w(),
        }
    }
}

impl<T: Copy + num::Num> std::ops::Mul<vec4<T>> for mat4<T> {
    type Output = vec4<T>;

    fn mul(self, rhs: vec4<T>) -> Self::Output {
        self.x * rhs.xxxx() + self.y * rhs.yyyy() + self.z * rhs.zzzz() + self.w * rhs.wwww()
    }
}

impl<T: Copy + num::Num> std::ops::Mul for mat4<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self * rhs.x,
            self * rhs.y,
            self * rhs.z,
            self * rhs.w,
        )
    }
}

pub type mat2u8 = mat2<u8>;
pub type mat2u16 = mat2<u16>;
pub type mat2u = mat2<u32>;
pub type mat2u64 = mat2<u64>;
pub type mat2u128 = mat2<u128>;
pub type mat2usize = mat2<usize>;
pub type mat2i8 = mat2<i8>;
pub type mat2i16 = mat2<i16>;
pub type mat2i = mat2<i32>;
pub type mat2i64 = mat2<i64>;
pub type mat2i128 = mat2<i128>;
pub type mat2isize = mat2<isize>;
pub type mat2f = mat2<f32>;
pub type mat2d = mat2<f64>;

pub type mat3u8 = mat3<u8>;
pub type mat3u16 = mat3<u16>;
pub type mat3u = mat3<u32>;
pub type mat3u64 = mat3<u64>;
pub type mat3u128 = mat3<u128>;
pub type mat3usize = mat3<usize>;
pub type mat3i8 = mat3<i8>;
pub type mat3i16 = mat3<i16>;
pub type mat3i = mat3<i32>;
pub type mat3i64 = mat3<i64>;
pub type mat3i128 = mat3<i128>;
pub type mat3isize = mat3<isize>;
pub type mat3f = mat3<f32>;
pub type mat3d = mat3<f64>;

pub type mat4u8 = mat4<u8>;
pub type mat4u16 = mat4<u16>;
pub type mat4u = mat4<u32>;
pub type mat4u64 = mat4<u64>;
pub type mat4u128 = mat4<u128>;
pub type mat4usize = mat4<usize>;
pub type mat4i8 = mat4<i8>;
pub type mat4i16 = mat4<i16>;
pub type mat4i = mat4<i32>;
pub type mat4i64 = mat4<i64>;
pub type mat4i128 = mat4<i128>;
pub type mat4isize = mat4<isize>;
pub type mat4f = mat4<f32>;
pub type mat4d = mat4<f64>;

macro impl_mat_2($arr_name:ident => $name:ident) {
    impl $arr_name {
        pub fn into_struct(self) -> $name {
            $name::new(
                vec2::new(self.0[0], self.0[1]),
                vec2::new(self.0[2], self.0[3]),
            )
        }
    }

    impl $name {
        pub fn into_arr(self) -> $arr_name {
            $arr_name([
                self.x.x, self.x.y,
                self.y.x, self.y.y,
            ])
        }
    }

    impl From<$arr_name> for $name {
        fn from(value: $arr_name) -> Self {
            value.into_struct()
        }
    }

    impl Into<$arr_name> for $name {
        fn into(self) -> $arr_name {
            self.into_arr()
        }
    }
}

macro impl_mat_3($arr_name:ident => $name:ident) {
    impl $arr_name {
        pub fn into_struct(self) -> $name {
            $name::new(
                vec3::new(self.0[0], self.0[1], self.0[2]),
                vec3::new(self.0[3], self.0[4], self.0[5]),
                vec3::new(self.0[6], self.0[7], self.0[8]),
            )
        }
    }

    impl $name {
        pub fn into_arr(self) -> $arr_name {
            $arr_name([
                self.x.x, self.x.y, self.x.z,
                self.y.x, self.y.y, self.y.z,
                self.z.x, self.z.y, self.z.z,
            ])
        }
    }

    impl From<$arr_name> for $name {
        fn from(value: $arr_name) -> Self {
            value.into_struct()
        }
    }

    impl Into<$arr_name> for $name {
        fn into(self) -> $arr_name {
            self.into_arr()
        }
    }
}

macro impl_mat_4($arr_name:ident => $name:ident) {
    impl $arr_name {
        pub fn into_struct(self) -> $name {
            $name::new(
                vec4::new(self.0[0], self.0[1], self.0[2], self.0[3]),
                vec4::new(self.0[4], self.0[5], self.0[6], self.0[7]),
                vec4::new(self.0[8], self.0[9], self.0[10], self.0[11]),
                vec4::new(self.0[12], self.0[13], self.0[14], self.0[15]),
            )
        }
    }

    impl $name {
        pub fn into_arr(self) -> $arr_name {
            $arr_name([
                self.x.x, self.x.y, self.x.z, self.x.w,
                self.y.x, self.y.y, self.y.z, self.y.w,
                self.z.x, self.z.y, self.z.z, self.z.w,
                self.w.x, self.w.y, self.w.z, self.w.w,
            ])
        }
    }

    impl From<$arr_name> for $name {
        fn from(value: $arr_name) -> Self {
            value.into_struct()
        }
    }

    impl From<$name> for $arr_name {
        fn from(value: $name) -> Self {
            value.into_arr()
        }
    }
}

macro impl_mat_2s($($arr_name:ident => $name:ident),*) {
    $(impl_mat_2!($arr_name => $name);)*
}
macro impl_mat_3s($($arr_name:ident => $name:ident),*) {
    $(impl_mat_3!($arr_name => $name);)*
}
macro impl_mat_4s($($arr_name:ident => $name:ident),*) {
    $(impl_mat_4!($arr_name => $name);)*
}

pub struct ArrMat2U8(pub [u8; 4]);
pub struct ArrMat2U16(pub [u16; 4]);
pub struct ArrMat2U32(pub [u32; 4]);
pub struct ArrMat2U64(pub [u64; 4]);
pub struct ArrMat2U128(pub [u128; 4]);
pub struct ArrMat2Usize(pub [usize; 4]);

pub struct ArrMat2I8(pub [i8; 4]);
pub struct ArrMat2I16(pub [i16; 4]);
pub struct ArrMat2I32(pub [i32; 4]);
pub struct ArrMat2I64(pub [i64; 4]);
pub struct ArrMat2I128(pub [i128; 4]);
pub struct ArrMat2Isize(pub [isize; 4]);

pub struct ArrMat2F32(pub [f32; 4]);
pub struct ArrMat2F64(pub [f64; 4]);

impl_mat_2s!(
    ArrMat2U8 => mat2u8,
    ArrMat2U16 => mat2u16,
    ArrMat2U32 => mat2u,
    ArrMat2U64 => mat2u64,
    ArrMat2U128 => mat2u128,
    ArrMat2Usize => mat2usize,
    ArrMat2I8 => mat2i8,
    ArrMat2I16 => mat2i16,
    ArrMat2I32 => mat2i,
    ArrMat2I64 => mat2i64,
    ArrMat2I128 => mat2i128,
    ArrMat2Isize => mat2isize,
    ArrMat2F32 => mat2f,
    ArrMat2F64 => mat2d
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3U8(pub [u8; 9]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3U16(pub [u16; 9]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3U32(pub [u32; 9]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3U64(pub [u64; 9]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3U128(pub [u128; 9]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3Usize(pub [usize; 9]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3I8(pub [i8; 9]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3I16(pub [i16; 9]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3I32(pub [i32; 9]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3I64(pub [i64; 9]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3I128(pub [i128; 9]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3Isize(pub [isize; 9]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3F32(pub [f32; 9]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat3F64(pub [f64; 9]);

impl_mat_3s!(
    ArrMat3U8 => mat3u8,
    ArrMat3U16 => mat3u16,
    ArrMat3U32 => mat3u,
    ArrMat3U64 => mat3u64,
    ArrMat3U128 => mat3u128,
    ArrMat3Usize => mat3usize,
    ArrMat3I8 => mat3i8,
    ArrMat3I16 => mat3i16,
    ArrMat3I32 => mat3i,
    ArrMat3I64 => mat3i64,
    ArrMat3I128 => mat3i128,
    ArrMat3Isize => mat3isize,
    ArrMat3F32 => mat3f,
    ArrMat3F64 => mat3d
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4U8(pub [u8; 16]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4U16(pub [u16; 16]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4U32(pub [u32; 16]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4U64(pub [u64; 16]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4U128(pub [u128; 16]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4Usize(pub [usize; 16]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4I8(pub [i8; 16]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4I16(pub [i16; 16]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4I32(pub [i32; 16]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4I64(pub [i64; 16]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4I128(pub [i128; 16]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4Isize(pub [isize; 16]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4F32(pub [f32; 16]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrMat4F64(pub [f64; 16]);

impl_mat_4s!(
    ArrMat4U8 => mat4u8,
    ArrMat4U16 => mat4u16,
    ArrMat4U32 => mat4u,
    ArrMat4U64 => mat4u64,
    ArrMat4U128 => mat4u128,
    ArrMat4Usize => mat4usize,
    ArrMat4I8 => mat4i8,
    ArrMat4I16 => mat4i16,
    ArrMat4I32 => mat4i,
    ArrMat4I64 => mat4i64,
    ArrMat4I128 => mat4i128,
    ArrMat4Isize => mat4isize,
    ArrMat4F32 => mat4f,
    ArrMat4F64 => mat4d
);
