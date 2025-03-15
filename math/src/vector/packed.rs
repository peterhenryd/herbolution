use crate::vector::{vec3u8, Vec3};
use bytemuck::{Pod, Zeroable};
use num::NumCast;
use serde::{Deserialize, Serialize};

/// Stores three 5-bit unsigned integers (ranged 0-31) in a single 16-bit value.
#[repr(C)]
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
