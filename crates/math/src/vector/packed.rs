use std::ops::Sub;
use crate::vector::{vec3u8, Vec3};
use bytemuck::{Pod, Zeroable};
use num::NumCast;
use serde::{Deserialize, Serialize};

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

    pub fn linearize(&self) -> usize {
        self.cast().unwrap().linearize(32)
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