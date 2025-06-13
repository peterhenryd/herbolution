use std::num::NonZeroU16;
use std::ops::Sub;
use crate::vector::{vec3u8, vec4u8, Vec3, Vec4};
use bytemuck::{Pod, Zeroable};
use num::NumCast;
use serde::{Deserialize, Serialize};
use static_assertions::assert_eq_size;

/// Stores three 5-bit unsigned integers (ranged 0-31) in a single 16-bit value.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Pod, Zeroable, Deserialize, Serialize)]
pub struct vec3u5(u16);

impl vec3u5 {
    pub const ZERO: Self = Self::new(0, 0, 0);

    #[inline]
    pub const fn new(x: u8, y: u8, z: u8) -> Self {
        debug_assert!(x < 32, "x out of range");
        debug_assert!(y < 32, "y out of range");
        debug_assert!(z < 32, "z out of range");

        Self((x as u16) << 10 | (y as u16) << 5 | z as u16)
    }

    pub const fn try_new(x: u8, y: u8, z: u8) -> Option<Self> {
        if x < 32 && y < 32 && z < 32 {
            Some(Self::new(x, y, z))
        } else {
            None
        }
    }

    pub fn try_from(vec: vec3u8) -> Option<Self> {
        Self::try_new(vec.x, vec.y, vec.z)
    }

    #[inline]
    pub const fn x(self) -> u8 {
        ((self.0 >> 10) & 31) as u8
    }

    #[inline]
    pub const fn y(self) -> u8 {
        ((self.0 >> 5) & 31) as u8
    }

    #[inline]
    pub const fn z(self) -> u8 {
        (self.0 & 31) as u8
    }

    #[inline]
    pub const fn into_u8(self) -> vec3u8 {
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
    pub const fn into_tuple(self) -> (u8, u8, u8) {
        (self.x(), self.y(), self.z())
    }
}

impl<T: NumCast> From<Vec3<T>> for vec3u5 {
    fn from(vec: Vec3<T>) -> Self {
        Self::new(
            NumCast::from(vec.x).unwrap(),
            NumCast::from(vec.y).unwrap(),
            NumCast::from(vec.z).unwrap(),
        )
    }
}

impl Sub for vec3u5 {
    type Output = vec3u5;

    fn sub(self, rhs: Self) -> Self::Output {
        vec3u5::new(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
        )
    }
}

impl Default for vec3u5 {
    fn default() -> Self {
        Self::ZERO
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

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct vec3u4(NonZeroU16);

assert_eq_size!(Option<vec3u4>, vec3u4);

impl vec3u4 {
    // Even though this is an unchecked constructor, it is safe because it will always uphold NonZeroU16's invariant.
    pub const fn new_unchecked(x: u8, y: u8, z: u8) -> Self {
        let n = 1
            | (x as u16) << 12
            | (y as u16) << 8
            | (z as u16) << 4;
        
        Self(unsafe { NonZeroU16::new_unchecked(n) })
    }
    
    pub const fn new(x: u8, y: u8, z: u8) -> Self {
        debug_assert!(x < 16, "x out of range");
        debug_assert!(y < 16, "y out of range");
        debug_assert!(z < 16, "z out of range");
        
        Self::new_unchecked(x, y, z)
    }

    pub const fn try_new(x: u8, y: u8, z: u8) -> Option<Self> {
        if x < 16 && y < 16 && z < 16 {
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
        self.x() as usize * 16usize.pow(2) 
            + self.z() as usize * 16 
            + self.y() as usize
    }
}

impl<T: NumCast> From<Vec3<T>> for vec3u4 {
    fn from(vec: Vec3<T>) -> Self {
        Self::new(
            NumCast::from(vec.x).unwrap(),
            NumCast::from(vec.y).unwrap(),
            NumCast::from(vec.z).unwrap(),
        )
    }
}