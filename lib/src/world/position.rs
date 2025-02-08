use crate::world::chunk;
use math::vector::{vec3, vec3i, vec3u8};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CubePosition(pub vec3i);

impl From<vec3i> for CubePosition {
    fn from(value: vec3i) -> Self {
        CubePosition(value)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ChunkLocalPosition {
    pub chunk: vec3i,
    pub local: vec3u8,
}

impl From<ChunkLocalPosition> for CubePosition {
    fn from(clp: ChunkLocalPosition) -> Self {
        CubePosition(vec3i {
            x: clp.chunk.x * chunk::LENGTH as i32 + clp.local.x as i32,
            y: clp.chunk.y * chunk::LENGTH as i32 + clp.local.y as i32,
            z: clp.chunk.z * chunk::LENGTH as i32 + clp.local.z as i32,
        })
    }
}

impl From<CubePosition> for ChunkLocalPosition {
    fn from(wp: CubePosition) -> Self {
        let chunk_x = wp.0.x.div_euclid(chunk::LENGTH as i32);
        let chunk_y = wp.0.y.div_euclid(chunk::LENGTH as i32);
        let chunk_z = wp.0.z.div_euclid(chunk::LENGTH as i32);

        let local_x = wp.0.x.rem_euclid(chunk::LENGTH as i32) as u8;
        let local_y = wp.0.y.rem_euclid(chunk::LENGTH as i32) as u8;
        let local_z = wp.0.z.rem_euclid(chunk::LENGTH as i32) as u8;

        ChunkLocalPosition {
            chunk: vec3::new(chunk_x, chunk_y, chunk_z),
            local: vec3::new(local_x, local_y, local_z),
        }
    }
}