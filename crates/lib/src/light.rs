use math::vec::vec3u5;
use std::fmt::{Debug, Formatter};

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
