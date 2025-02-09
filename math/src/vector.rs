#![allow(non_camel_case_types)]

#[derive(derive::Vector, serde::Deserialize, serde::Serialize)]
pub struct vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(derive::Vector, serde::Deserialize, serde::Serialize)]
pub struct vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> vec3<T> {
    pub fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self {
            x: v,
            y: v,
            z: v,
        }
    }

    pub fn extend(self, w: T) -> vec4<T> {
        vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }
}

impl<T: std::ops::Div<Output=T>> std::ops::Div for vec2<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: num::Num> vec3<T> {
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> T {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }
}

impl<A: num::ToPrimitive> vec2<A> {
    pub fn cast<B: num::NumCast>(self) -> vec2<B> {
        vec2 {
            x: num::NumCast::from(self.x).unwrap(),
            y: num::NumCast::from(self.y).unwrap(),
        }
    }
}

impl<A: num::ToPrimitive> vec3<A> {
    pub fn cast<B: num::NumCast>(self) -> vec3<B> {
        vec3 {
            x: num::NumCast::from(self.x).unwrap(),
            y: num::NumCast::from(self.y).unwrap(),
            z: num::NumCast::from(self.z).unwrap(),
        }
    }
}

impl<T: Copy + num::Num> std::ops::Div<T> for vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Copy + num::Num> std::ops::Rem<T> for vec3<T> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
            z: self.z % rhs,
        }
    }
}

impl<T: num::Float> vec3<T> {
    pub fn square_length(&self) -> T
    where
        T: Copy,
    {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> T
    where
        T: Copy,
    {
        self.square_length().sqrt()
    }

    pub fn normalize(&self) -> Self
    where
        T: Copy,
    {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

#[derive(derive::Vector, serde::Deserialize, serde::Serialize)]
pub struct vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<A: num::ToPrimitive> vec4<A> {
    pub fn cast<B: num::NumCast>(self) -> vec4<B> {
        vec4 {
            x: num::NumCast::from(self.x).unwrap(),
            y: num::NumCast::from(self.y).unwrap(),
            z: num::NumCast::from(self.z).unwrap(),
            w: num::NumCast::from(self.w).unwrap(),
        }
    }
}

impl<T> vec3<T> {
    pub fn xy(self) -> vec2<T> {
        vec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn xz(self) -> vec2<T> {
        vec2 {
            x: self.x,
            y: self.z,
        }
    }
}

impl<T> vec4<T> {
    pub fn xxxx(&self) -> vec4<T>
    where
        T: Copy,
    {
        vec4 {
            x: self.x,
            y: self.x,
            z: self.x,
            w: self.x,
        }
    }

    pub fn yyyy(&self) -> vec4<T>
    where
        T: Copy,
    {
        vec4 {
            x: self.y,
            y: self.y,
            z: self.y,
            w: self.y,
        }
    }

    pub fn zzzz(&self) -> vec4<T>
    where
        T: Copy,
    {
        vec4 {
            x: self.z,
            y: self.z,
            z: self.z,
            w: self.z,
        }
    }

    pub fn wwww(&self) -> vec4<T>
    where
        T: Copy,
    {
        vec4 {
            x: self.w,
            y: self.w,
            z: self.w,
            w: self.w,
        }
    }
}

pub type vec2u8 = vec2<u8>;
pub type vec2u16 = vec2<u16>;
pub type vec2u = vec2<u32>;
pub type vec2u64 = vec2<u64>;
pub type vec2u128 = vec2<u128>;
pub type vec2usize = vec2<usize>;
pub type vec2i8 = vec2<i8>;
pub type vec2i16 = vec2<i16>;
pub type vec2i = vec2<i32>;
pub type vec2i64 = vec2<i64>;
pub type vec2i128 = vec2<i128>;
pub type vec2isize = vec2<isize>;
pub type vec2f = vec2<f32>;
pub type vec2d = vec2<f64>;

pub type vec3u8 = vec3<u8>;
pub type vec3u16 = vec3<u16>;
pub type vec3u = vec3<u32>;
pub type vec3u64 = vec3<u64>;
pub type vec3u128 = vec3<u128>;
pub type vec3usize = vec3<usize>;
pub type vec3i8 = vec3<i8>;
pub type vec3i16 = vec3<i16>;
pub type vec3i = vec3<i32>;
pub type vec3i64 = vec3<i64>;
pub type vec3i128 = vec3<i128>;
pub type vec3isize = vec3<isize>;
pub type vec3f = vec3<f32>;
pub type vec3d = vec3<f64>;

pub type vec4u8 = vec4<u8>;
pub type vec4u16 = vec4<u16>;
pub type vec4u = vec4<u32>;
pub type vec4u64 = vec4<u64>;
pub type vec4u128 = vec4<u128>;
pub type vec4usize = vec4<usize>;
pub type vec4i8 = vec4<i8>;
pub type vec4i16 = vec4<i16>;
pub type vec4i = vec4<i32>;
pub type vec4i64 = vec4<i64>;
pub type vec4i128 = vec4<i128>;
pub type vec4isize = vec4<isize>;
pub type vec4f = vec4<f32>;
pub type vec4d = vec4<f64>;

macro impl_vec_2($arr_name:ident => $name:ident) {
    impl $arr_name {
        pub fn into_struct(self) -> $name {
            $name::new(self.0[0], self.0[1])
        }
    }

    impl $name {
        pub fn into_arr(self) -> $arr_name {
            $arr_name([self.x, self.y])
        }
    }

    impl From<$name> for $arr_name {
        fn from(value: $name) -> Self {
            value.into_arr()
        }
    }

    impl From<$arr_name> for $name {
        fn from(value: $arr_name) -> Self {
            value.into_struct()
        }
    }
}

macro impl_vec_2s($($arr_name:ident => $name:ident),*) {
    $(impl_vec_2!($arr_name => $name);)*
}

macro impl_vec_3($arr_name:ident => $name:ident) {
    impl $arr_name {
        pub fn into_struct(self) -> $name {
            $name::new(self.0[0], self.0[1], self.0[2])
        }
    }

    impl $name {
        pub fn into_arr(self) -> $arr_name {
            $arr_name([self.x, self.y, self.z])
        }
    }

    impl From<$name> for $arr_name {
        fn from(value: $name) -> Self {
            value.into_arr()
        }
    }

    impl From<$arr_name> for $name {
        fn from(value: $arr_name) -> Self {
            value.into_struct()
        }
    }
}

macro impl_vec_3s($($arr_name:ident => $name:ident),*) {
    $(impl_vec_3!($arr_name => $name);)*
}

macro impl_vec_4($arr_name:ident => $name:ident) {
    impl $arr_name {
        pub fn into_struct(self) -> $name {
            $name::new(self.0[0], self.0[1], self.0[2], self.0[3])
        }
    }

    impl $name {
        pub fn into_arr(self) -> $arr_name {
            $arr_name([self.x, self.y, self.z, self.w])
        }
    }

    impl From<$name> for $arr_name {
        fn from(value: $name) -> Self {
            value.into_arr()
        }
    }

    impl From<$arr_name> for $name {
        fn from(value: $arr_name) -> Self {
            value.into_struct()
        }
    }
}

macro impl_vec_4s($($arr_name:ident => $name:ident),*) {
    $(impl_vec_4!($arr_name => $name);)*
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2U8(pub [u8; 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2U16(pub [u16; 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2U32(pub [u32; 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2U64(pub [u64; 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2U128(pub [u128; 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2Usize(pub [usize; 2]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2I8(pub [i8; 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2I16(pub [i16; 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2I32(pub [i32; 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2I64(pub [i64; 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2I128(pub [i128; 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2Isize(pub [isize; 2]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2F32(pub [f32; 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec2F64(pub [f64; 2]);

impl_vec_2s!(
    ArrVec2U8 => vec2u8,
    ArrVec2U16 => vec2u16,
    ArrVec2U32 => vec2u,
    ArrVec2U64 => vec2u64,
    ArrVec2U128 => vec2u128,
    ArrVec2Usize => vec2usize,
    ArrVec2I8 => vec2i8,
    ArrVec2I16 => vec2i16,
    ArrVec2I32 => vec2i,
    ArrVec2I64 => vec2i64,
    ArrVec2I128 => vec2i128,
    ArrVec2Isize => vec2isize,
    ArrVec2F32 => vec2f,
    ArrVec2F64 => vec2d
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3U8(pub [u8; 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3U16(pub [u16; 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3U32(pub [u32; 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3U64(pub [u64; 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3U128(pub [u128; 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3Usize(pub [usize; 3]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3I8(pub [i8; 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3I16(pub [i16; 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3I32(pub [i32; 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3I64(pub [i64; 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3I128(pub [i128; 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3Isize(pub [isize; 3]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3F32(pub [f32; 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec3F64(pub [f64; 3]);

impl_vec_3s!(
    ArrVec3U8 => vec3u8,
    ArrVec3U16 => vec3u16,
    ArrVec3U32 => vec3u,
    ArrVec3U64 => vec3u64,
    ArrVec3U128 => vec3u128,
    ArrVec3Usize => vec3usize,
    ArrVec3I8 => vec3i8,
    ArrVec3I16 => vec3i16,
    ArrVec3I32 => vec3i,
    ArrVec3I64 => vec3i64,
    ArrVec3I128 => vec3i128,
    ArrVec3Isize => vec3isize,
    ArrVec3F32 => vec3f,
    ArrVec3F64 => vec3d
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec4U8(pub [u8; 4]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec4U16(pub [u16; 4]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec4U32(pub [u32; 4]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec4U64(pub [u64; 4]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec4U128(pub [u128; 4]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec4I8(pub [i8; 4]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec4I16(pub [i16; 4]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec4I32(pub [i32; 4]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec4I64(pub [i64; 4]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec4I128(pub [i128; 4]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec4F32(pub [f32; 4]);
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArrVec4F64(pub [f64; 4]);

impl_vec_4s!(
    ArrVec4U8 => vec4u8,
    ArrVec4U16 => vec4u16,
    ArrVec4U32 => vec4u,
    ArrVec4U64 => vec4u64,
    ArrVec4U128 => vec4u128,
    ArrVec4I8 => vec4i8,
    ArrVec4I16 => vec4i16,
    ArrVec4I32 => vec4i,
    ArrVec4I64 => vec4i64,
    ArrVec4I128 => vec4i128,
    ArrVec4F32 => vec4f,
    ArrVec4F64 => vec4d
);
