use std::fmt::{Debug, Formatter};

use lib::spatial::{CubeFaces, PerFaceU5};

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Cube<M> {
    pub material: M,
    pub flags: CubeFlags,
}

impl<M> Cube<M> {
    pub const fn new(material: M) -> Self {
        Self {
            material,
            flags: CubeFlags::new(),
        }
    }
}

impl<M: Default> Default for Cube<M> {
    fn default() -> Self {
        Self::new(M::default())
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CubeFlags {
    value: u32,
}

impl CubeFlags {
    pub const fn new() -> Self {
        Self { value: 0 }
    }

    pub fn faces(&self) -> CubeFaces {
        if self.value & 1 == 0 {
            CubeFaces::none()
        } else {
            CubeFaces::from((self.value >> 1) as u8)
        }
    }

    pub fn insert_faces(&mut self, faces: impl Into<CubeFaces>) {
        self.set_opaque(self.faces() + faces.into())
    }

    pub fn remove_faces(&mut self, faces: impl Into<CubeFaces>) {
        self.set_opaque(self.faces() - faces.into())
    }

    pub fn light_levels(&self) -> PerFaceU5 {
        if self.value & 1 == 0 {
            let east = ((self.value >> 1) & 31) as u8;
            let west = ((self.value >> 6) & 31) as u8;
            let top = ((self.value >> 11) & 31) as u8;
            let bottom = ((self.value >> 16) & 31) as u8;
            let north = ((self.value >> 21) & 31) as u8;
            let south = ((self.value >> 26) & 31) as u8;

            PerFaceU5::new(east, west, top, bottom, north, south)
        } else {
            PerFaceU5::ZERO
        }
    }

    #[inline]
    pub fn set_opaque(&mut self, faces: CubeFaces) {
        self.value = (faces.bits() as u32) << 1 | 1;
    }

    pub fn set_translucent(&mut self, light_levels: PerFaceU5) {
        let east = light_levels.east() as u32;
        let west = light_levels.west() as u32;
        let top = light_levels.up() as u32;
        let bottom = light_levels.down() as u32;
        let north = light_levels.north() as u32;
        let south = light_levels.south() as u32;

        self.value = (east & 31) << 1 | (west & 31) << 6 | (top & 31) << 11 | (bottom & 31) << 16 | (north & 31) << 21 | (south & 31) << 26;
    }

    #[inline]
    pub fn is_opaque(&self) -> bool {
        self.value & 1 == 1
    }
}

impl Debug for CubeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CubeDependentData")
            .field("faces", &self.faces())
            .field("light_levels", &self.light_levels())
            .finish()
    }
}
