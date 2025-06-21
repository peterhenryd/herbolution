use bytemuck::{Pod, Zeroable};
use num::NumCast;
use serde::{Deserialize, Serialize};
use crate::vec::vec_type;

vec_type! {
    struct Vec4<T> {
        x(X = 1, 0, 0, 0): T,
        y(Y = 0, 1, 0, 0): T,
        z(Z = 0, 0, 1, 0): T,
        w(W = 0, 0, 0, 1): T,
    }
    linearize(x, y, z, w)
}

impl<T> Vec4<T> {

    pub fn xxxx(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.x,
            y: self.x,
            z: self.x,
            w: self.x,
        }
    }

    pub fn yyyy(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.y,
            y: self.y,
            z: self.y,
            w: self.y,
        }
    }

    pub fn zzzz(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.z,
            y: self.z,
            z: self.z,
            w: self.z,
        }
    }

    pub fn wwww(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.w,
            y: self.w,
            z: self.w,
            w: self.w,
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Pod, Zeroable, Deserialize, Serialize)]
pub struct vec4u4(u16);

impl vec4u4 {
    pub const fn new_unchecked(x: u8, y: u8, z: u8, w: u8) -> Self {
        let n = (x as u16) << 12
            | (y as u16) << 8
            | (z as u16) << 4
            | w as u16;

        Self(n)
    }

    pub const fn new(x: u8, y: u8, z: u8, w: u8) -> Self {
        debug_assert!(x < 16, "x out of range");
        debug_assert!(y < 16, "y out of range");
        debug_assert!(z < 16, "z out of range");
        debug_assert!(w < 16, "w out of range");

        Self::new_unchecked(x, y, z, w)
    }

    pub const fn try_new(x: u8, y: u8, z: u8, w: u8) -> Option<Self> {
        if x < 16 && y < 16 && z < 16 && w < 16 {
            Some(Self::new_unchecked(x, y, z, w))
        } else {
            None
        }
    }

    pub fn try_from(vec: vec4u8) -> Option<Self> {
        Self::try_new(vec.x, vec.y, vec.z, vec.w)
    }

    #[inline]
    pub const fn x(self) -> u8 {
        ((self.0 >> 12) & 15) as u8
    }

    #[inline]
    pub const fn y(self) -> u8 {
        ((self.0 >> 8) & 15) as u8
    }

    #[inline]
    pub const fn z(self) -> u8 {
        ((self.0 >> 4) & 15) as u8
    }

    #[inline]
    pub const fn w(self) -> u8 {
        (self.0 & 15) as u8
    }

    #[inline]
    pub const fn to_vec4u8(self) -> vec4u8 {
        Vec4::new(self.x(), self.y(), self.z(), self.w())
    }

    #[inline]
    pub fn cast<U: NumCast>(self) -> Option<Vec4<U>> {
        Some(Vec4 {
            x: NumCast::from(self.x())?,
            y: NumCast::from(self.y())?,
            z: NumCast::from(self.z())?,
            w: NumCast::from(self.w())?,
        })
    }

    #[inline]
    pub const fn to_tuple(self) -> (u8, u8, u8, u8) {
        (self.x(), self.y(), self.z(), self.w())
    }
}

impl<T: NumCast> From<Vec4<T>> for vec4u4 {
    fn from(vec: Vec4<T>) -> Self {
        Self::new(
            NumCast::from(vec.x).unwrap(),
            NumCast::from(vec.y).unwrap(),
            NumCast::from(vec.z).unwrap(),
            NumCast::from(vec.w).unwrap(),
        )
    }
}

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