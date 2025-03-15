use math::num::{Integer, ToPrimitive};
use std::fmt::{Debug, Formatter};
use bytemuck::{Pod, Zeroable};
use lib::geometry::cuboid::face::Faces;
use lib::light::FacialLightLevels;
use math::vector::vec3i;
use crate::world::chunk;
use crate::world::chunk::ChunkLocalPosition;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cube<M> {
    pub material: M,
    /// For cubes with any amount of translucency, this value is used to encode the amount of light
    /// each face of the cube should let through, as a 5-bit value (allowing for 32 light levels).
    ///
    /// For opaque cubes, this value is used to encode the number of faces that should be rendered,
    /// only requiring 6 bits as each facial rendering query only requires a boolean value.
    /// In the future, more logic may be encoded here for other purposes.
    ///
    /// To avoid repetitive queries of the material palette, the first bit of this value designates
    /// whether the logic is translucent (0) or opaque (1).
    pub dependent_data: u32,
}

impl<M> Cube<M> {
    pub const fn new(material_index: M) -> Self {
        Self {
            material: material_index,
            dependent_data: 0,
        }
    }

    pub fn faces(&self) -> Faces {
        if self.dependent_data & 1 == 0 {
            Faces::empty()
        } else {
            Faces::from_bits_truncate((self.dependent_data >> 1).to_u8().unwrap())
        }
    }

    pub fn insert_faces(&mut self, faces: Faces) {
        self.set_opaque(self.faces().union(faces))
    }

    pub fn remove_faces(&mut self, faces: Faces) {
        self.set_opaque(self.faces().difference(faces))
    }

    pub fn light_levels(&self) -> FacialLightLevels {
        if self.dependent_data & 1 == 0 {
            let top = ((self.dependent_data >> 1) & 31) as u8;
            let bottom = ((self.dependent_data >> 6) & 31) as u8;
            let left = ((self.dependent_data >> 11) & 31) as u8;
            let right = ((self.dependent_data >> 16) & 31) as u8;
            let front = ((self.dependent_data >> 21) & 31) as u8;
            let back = ((self.dependent_data >> 26) & 31) as u8;

            FacialLightLevels::new(top, bottom, left, right, front, back)
        } else {
            FacialLightLevels::NONE
        }
    }

    #[inline]
    pub fn set_opaque(&mut self, faces: Faces) {
        self.dependent_data = (faces.bits() as u32) << 1 | 1;
    }

    pub fn set_translucent(&mut self, light_levels: FacialLightLevels)
    where
        M: Integer,
    {
        let top = light_levels.top() as u32;
        let bottom = light_levels.bottom() as u32;
        let left = light_levels.left() as u32;
        let right = light_levels.right() as u32;
        let front = light_levels.front() as u32;
        let back = light_levels.back() as u32;

        self.dependent_data = (top & 31) << 1
            | (bottom & 31) << 6
            | (left & 31) << 11
            | (right & 31) << 16
            | (front & 31) << 21
            | (back & 31) << 26;
    }

    #[inline]
    pub fn is_opaque(&self) -> bool {
        self.dependent_data & 1 == 1
    }
}

impl<M: Debug> Debug for Cube<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cube")
            .field("material_index", &self.material)
            .field("faces", &self.faces())
            .field("light_levels", &self.light_levels())
            .finish()
    }
}

unsafe impl<M: Pod> Pod for Cube<M> {}

unsafe impl<M: Zeroable> Zeroable for Cube<M> {}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CubePosition(pub vec3i);

impl From<vec3i> for CubePosition {
    fn from(value: vec3i) -> Self {
        CubePosition(value)
    }
}

impl From<ChunkLocalPosition> for CubePosition {
    fn from(pos: ChunkLocalPosition) -> Self {
        CubePosition(vec3i {
            x: pos.chunk.x * chunk::LENGTH as i32 + pos.local.x() as i32,
            y: pos.chunk.y * chunk::LENGTH as i32 + pos.local.y() as i32,
            z: pos.chunk.z * chunk::LENGTH as i32 + pos.local.z() as i32,
        })
    }
}