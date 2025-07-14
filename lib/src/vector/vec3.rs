use std::num::NonZeroU16;
use std::ops::{Mul, Sub};

use num::NumCast;
use serde::{Deserialize, Serialize};
use static_assertions::assert_eq_size;

use crate::vector::macros::vector;
use crate::vector::{vec3d, vec3f, vec3i, vec3u8, Vec2, Vec4};

vector! {
    struct Vec3<T> {
        x(X = 1, 0, 0): T,
        y(Y = 0, 1, 0): T,
        z(Z = 0, 0, 1): T,
    }
    linearize(z, y, x)
}

impl<T> Vec3<T> {
    #[inline]
    pub fn extend(self, w: T) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> Self
    where
        T: Copy + Sub<Output = T>,
        T: Mul<Output = T>,
    {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    pub fn xy(self) -> Vec2<T> {
        Vec2 { x: self.x, y: self.y }
    }

    #[inline]
    pub fn xz(self) -> Vec2<T> {
        Vec2 { x: self.x, y: self.z }
    }

    #[inline]
    pub fn xxx(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    pub fn yyy(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    pub fn zzz(self) -> Self
    where
        T: Copy,
    {
        Self {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }
}

impl vec3d {
    #[inline]
    pub fn split_int_fract(self) -> (vec3i, vec3f) {
        (self.cast::<i32>(), self.cast::<f32>().fract())
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct vec3u4(NonZeroU16);

impl vec3u4 {
    #[inline]
    pub const fn new_unchecked(x: u8, y: u8, z: u8) -> Self {
        let n = 1 | (x as u16) << 12 | (y as u16) << 8 | (z as u16) << 4;

        Self(unsafe { NonZeroU16::new_unchecked(n) })
    }

    #[inline]
    pub const fn new(x: u8, y: u8, z: u8) -> Self {
        debug_assert!(x < 16, "x out of range");
        debug_assert!(y < 16, "y out of range");
        debug_assert!(z < 16, "z out of range");

        Self::new_unchecked(x, y, z)
    }

    #[inline]
    pub const fn try_new(x: u8, y: u8, z: u8) -> Option<Self> {
        if x < 16 && y < 16 && z < 16 {
            Some(Self::new_unchecked(x, y, z))
        } else {
            None
        }
    }

    #[inline]
    pub fn try_from(vec: vec3u8) -> Option<Self> {
        Self::try_new(vec.x, vec.y, vec.z)
    }

    #[inline]
    pub const fn x(self) -> u8 {
        (self.0.get() >> 12 & 15) as u8
    }

    #[inline]
    pub const fn y(self) -> u8 {
        (self.0.get() >> 8 & 15) as u8
    }

    #[inline]
    pub const fn z(self) -> u8 {
        (self.0.get() >> 4 & 15) as u8
    }

    #[inline]
    pub const fn to_vec3u8(self) -> vec3u8 {
        Vec3::new(self.x(), self.y(), self.z())
    }

    #[inline]
    pub fn cast<U: NumCast>(self) -> Option<Vec3<U>> {
        Some(Vec3 {
            x: NumCast::from(self.x())?,
            y: NumCast::from(self.y())?,
            z: NumCast::from(self.z())?,
        })
    }

    #[inline]
    pub const fn to_tuple(self) -> (u8, u8, u8) {
        (self.x(), self.y(), self.z())
    }

    #[inline]
    pub fn linearize(&self) -> usize {
        self.x() as usize * 16usize.pow(2) + self.z() as usize * 16 + self.y() as usize
    }
}

assert_eq_size!(Option<vec3u4>, vec3u4);

impl<T: NumCast> From<Vec3<T>> for vec3u4 {
    fn from(vec: Vec3<T>) -> Self {
        Self::new(NumCast::from(vec.x).unwrap(), NumCast::from(vec.y).unwrap(), NumCast::from(vec.z).unwrap())
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct vec3u5(NonZeroU16);

impl vec3u5 {
    pub const ZERO: Self = Self::new(0, 0, 0);

    #[inline]
    pub const fn new_unchecked(x: u8, y: u8, z: u8) -> Self {
        let n = 1 | (x as u16) << 11 | (y as u16) << 6 | (z as u16) << 1;

        Self(unsafe { NonZeroU16::new_unchecked(n) })
    }

    #[inline]
    pub const fn new(x: u8, y: u8, z: u8) -> Self {
        debug_assert!(x < 32, "x out of range");
        debug_assert!(y < 32, "y out of range");
        debug_assert!(z < 32, "z out of range");

        Self::new_unchecked(x, y, z)
    }

    pub const fn try_new(x: u8, y: u8, z: u8) -> Option<Self> {
        if x < 32 && y < 32 && z < 32 {
            Some(Self::new_unchecked(x, y, z))
        } else {
            None
        }
    }

    pub fn try_from(vec: vec3u8) -> Option<Self> {
        Self::try_new(vec.x, vec.y, vec.z)
    }

    #[inline]
    pub const fn x(self) -> u8 {
        ((self.0.get() >> 11) & 31) as u8
    }

    #[inline]
    pub const fn set_x(&mut self, x: u8) {
        debug_assert!(x < 32, "x out of range");
        self.0 = NonZeroU16::new((self.0.get() & !(31 << 11)) | ((x as u16) << 11)).unwrap();
    }

    #[inline]
    pub const fn y(self) -> u8 {
        ((self.0.get() >> 6) & 31) as u8
    }

    #[inline]
    pub const fn set_y(&mut self, y: u8) {
        debug_assert!(y < 32, "y out of range");
        self.0 = NonZeroU16::new((self.0.get() & !(31 << 6)) | ((y as u16) << 6)).unwrap();
    }

    #[inline]
    pub const fn z(self) -> u8 {
        ((self.0.get() >> 1) & 31) as u8
    }

    #[inline]
    pub const fn set_z(&mut self, z: u8) {
        debug_assert!(z < 32, "z out of range");
        self.0 = NonZeroU16::new((self.0.get() & !(31 << 1)) | ((z as u16) << 1)).unwrap();
    }

    #[inline]
    pub const fn into_u8(self) -> vec3u8 {
        Vec3::new(self.x(), self.y(), self.z())
    }

    #[inline]
    pub fn try_cast<U: NumCast>(self) -> Option<Vec3<U>> {
        Some(Vec3 {
            x: NumCast::from(self.x())?,
            y: NumCast::from(self.y())?,
            z: NumCast::from(self.z())?,
        })
    }

    #[inline]
    pub fn cast<U: NumCast>(self) -> Vec3<U> {
        self.try_cast().unwrap()
    }

    #[inline]
    pub const fn into_tuple(self) -> (u8, u8, u8) {
        (self.x(), self.y(), self.z())
    }

    #[inline]
    pub fn linearize(&self) -> usize {
        self.x() as usize * 32usize.pow(2) + self.z() as usize * 32 + self.y() as usize
    }
}

assert_eq_size!(Option<vec3u5>, vec3u5);

impl<T: NumCast> From<Vec3<T>> for vec3u5 {
    fn from(vec: Vec3<T>) -> Self {
        Self::new(NumCast::from(vec.x).unwrap(), NumCast::from(vec.y).unwrap(), NumCast::from(vec.z).unwrap())
    }
}

impl Sub for vec3u5 {
    type Output = vec3u5;

    fn sub(self, rhs: Self) -> Self::Output {
        vec3u5::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Default for vec3u5 {
    fn default() -> Self {
        Self::ZERO
    }
}
