use crate::vector::{vec3i, vec3u5, Vec3};
use crate::world::{CHUNK_EXP, CHUNK_LENGTH};
use std::ops::{Add, AddAssign, BitAnd, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CubePt(pub vec3i);

impl From<vec3i> for CubePt {
    fn from(value: vec3i) -> Self {
        Self(value)
    }
}

impl From<ChunkCubePt> for CubePt {
    fn from(value: ChunkCubePt) -> Self {
        Self(value.chunk.0 * CHUNK_LENGTH as i32 + value.local.try_cast().unwrap())
    }
}

pub struct ChunkCubePt {
    pub chunk: ChunkPt,
    pub local: vec3u5,
}

impl From<CubePt> for ChunkCubePt {
    fn from(value: CubePt) -> Self {
        Self {
            chunk: ChunkPt(value.0 >> CHUNK_EXP as i32),
            local: value.0.bitand(CHUNK_LENGTH as i32 - 1).into(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ChunkPt(pub vec3i);

impl ChunkPt {
    pub const ZERO: Self = Self(Vec3::ZERO);
}

impl Add for ChunkPt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for ChunkPt {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Add<vec3i> for ChunkPt {
    type Output = Self;

    fn add(self, rhs: vec3i) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<vec3i> for ChunkPt {
    fn add_assign(&mut self, rhs: vec3i) {
        self.0 += rhs;
    }
}

impl Sub for ChunkPt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for ChunkPt {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Sub<vec3i> for ChunkPt {
    type Output = Self;

    fn sub(self, rhs: vec3i) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<vec3i> for ChunkPt {
    fn sub_assign(&mut self, rhs: vec3i) {
        self.0 -= rhs;
    }
}
