use math::vector::vec3u5;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct LightLevel(u8);

impl LightLevel {
    pub const fn new(level: u8) -> Self {
        debug_assert!(level < 32, "light level out of bounds");

        Self(level)
    }

    #[inline]
    pub const fn into_u8(self) -> u8 {
        self.0
    }
}

impl Into<u8> for LightLevel {
    fn into(self) -> u8 {
        self.into_u8()
    }
}

#[repr(C)]
pub struct FacialLightLevels(vec3u5, vec3u5);

impl FacialLightLevels {
    pub const NONE: Self = Self(vec3u5::ZERO, vec3u5::ZERO);

    pub const fn new(top: u8, bottom: u8, left: u8, right: u8, front: u8, back: u8) -> Self {
        Self(
            vec3u5::new(top, bottom, left),
            vec3u5::new(right, front, back),
        )
    }

    pub const fn top(&self) -> u8 {
        self.0.x()
    }

    pub const fn bottom(&self) -> u8 {
        self.0.y()
    }

    pub const fn left(&self) -> u8 {
        self.0.z()
    }

    pub const fn right(&self) -> u8 {
        self.1.x()
    }

    pub const fn front(&self) -> u8 {
        self.1.y()
    }

    pub const fn back(&self) -> u8 {
        self.1.z()
    }
}

impl Debug for FacialLightLevels {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FacialLightLevels")
            .field("top", &self.top())
            .field("bottom", &self.bottom())
            .field("left", &self.left())
            .field("right", &self.right())
            .field("front", &self.front())
            .field("back", &self.back())
            .finish()
    }
}
