use lib::geometry::cuboid::face::Faces;
use lib::light::FacialLightLevels;
use math::num::ToPrimitive;
use math::vector::vec3i;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use crate::chunk;
use crate::chunk::ChunkLocalPos;

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Cube<M> {
    pub material: M,
    pub dependent_data: CubeDependentData,
}

impl<M> Cube<M> {
    pub const fn new(material: M) -> Self {
        Self {
            material,
            dependent_data: CubeDependentData::new(),
        }
    }
}

impl<M> Deref for Cube<M> {
    type Target = CubeDependentData;

    fn deref(&self) -> &Self::Target {
        &self.dependent_data
    }
}

impl<M> DerefMut for Cube<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dependent_data
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CubePos(pub vec3i);

impl From<vec3i> for CubePos {
    fn from(value: vec3i) -> Self {
        CubePos(value)
    }
}

impl From<ChunkLocalPos> for CubePos {
    fn from(pos: ChunkLocalPos) -> Self {
        CubePos(pos.chunk * chunk::LENGTH as i32 + pos.local.cast().unwrap())
    }
}

impl<M: Default> Default for Cube<M> {
    fn default() -> Self {
        Self::new(M::default())
    }
}

/// For cubes with any amount of translucency, this value is used to encode the amount of light
/// each face of the cube should let through as a 5-bit value (allowing for 32 light levels).
///
/// For opaque cubes, this value is used to encode the number of faces that should be rendered,
/// only requiring 6 bits in total as each facial rendering query only requires a boolean value.
/// In the future, more information may be encoded here for other purposes.
///
/// To avoid repetitive queries of the material palette, the first bit of this value designates
/// whether the cube is translucent (0) or opaque (1).
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CubeDependentData {
    value: u32
}

impl CubeDependentData {
    pub const fn new() -> Self {
        Self {
            value: 0,
        }
    }

    pub fn faces(&self) -> Faces {
        if self.value & 1 == 0 {
            Faces::empty()
        } else {
            Faces::from_bits_truncate((self.value >> 1).to_u8().unwrap())
        }
    }

    pub fn insert_faces(&mut self, faces: Faces) {
        self.set_opaque(self.faces().union(faces))
    }

    pub fn remove_faces(&mut self, faces: Faces) {
        self.set_opaque(self.faces().difference(faces))
    }

    pub fn light_levels(&self) -> FacialLightLevels {
        if self.value & 1 == 0 {
            let top = ((self.value >> 1) & 31) as u8;
            let bottom = ((self.value >> 6) & 31) as u8;
            let left = ((self.value >> 11) & 31) as u8;
            let right = ((self.value >> 16) & 31) as u8;
            let front = ((self.value >> 21) & 31) as u8;
            let back = ((self.value >> 26) & 31) as u8;

            FacialLightLevels::new(top, bottom, left, right, front, back)
        } else {
            FacialLightLevels::NONE
        }
    }

    #[inline]
    pub fn set_opaque(&mut self, faces: Faces) {
        self.value = (faces.bits() as u32) << 1 | 1;
    }

    pub fn set_translucent(&mut self, light_levels: FacialLightLevels) {
        let top = light_levels.top() as u32;
        let bottom = light_levels.bottom() as u32;
        let left = light_levels.left() as u32;
        let right = light_levels.right() as u32;
        let front = light_levels.front() as u32;
        let back = light_levels.back() as u32;

        self.value = (top & 31) << 1
            | (bottom & 31) << 6
            | (left & 31) << 11
            | (right & 31) << 16
            | (front & 31) << 21
            | (back & 31) << 26;
    }

    #[inline]
    pub fn is_opaque(&self) -> bool {
        self.value & 1 == 1
    }
}

impl Debug for CubeDependentData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CubeDependentData")
            .field("faces", &self.faces())
            .field("light_levels", &self.light_levels())
            .finish()
    }
}