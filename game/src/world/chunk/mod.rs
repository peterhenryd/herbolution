use std::ops::BitAnd;
use crossbeam::channel::{Sender, TrySendError};
use hashbrown::HashMap;
use math::vector::{vec3i, vec3u5};

mod cube;
mod mesh;
pub mod generator;
mod map;
pub mod material;
mod channel;

pub use cube::{Cube, CubePosition, CubeDependentData};
pub use mesh::Mesh;
pub use generator::Generator;
pub use map::Map;
pub use material::Material;

pub const LENGTH: usize = 32;
pub const SIZE: usize = LENGTH.pow(3);

#[derive(Debug)]
pub struct Chunk {
    mesh: Mesh,
    sender: Sender<ChunkUpdate>,
}

impl Chunk {
    pub fn new(mesh: Mesh, sender: Sender<ChunkUpdate>) -> Self {
        Self { mesh, sender }
    }

    fn send_overwrites(&mut self) -> bool {
        let overwrites = self.mesh.overwrites();

        if overwrites.is_empty() {
            return true;
        }

        match self.sender.try_send(ChunkUpdate { overwrites }) {
            Ok(()) | Err(TrySendError::Full(_)) => true,
            Err(TrySendError::Disconnected(_)) => false,
        }
    }

    pub fn tick(&mut self) {
    }
}

fn in_bounds(position: vec3i) -> bool {
    position.x >= 0 && position.x < LENGTH as i32
        && position.y >= 0 && position.y < LENGTH as i32
        && position.z >= 0 && position.z < LENGTH as i32
}

pub fn linearize(position: vec3u5) -> usize {
    position.cast().unwrap().linearize(LENGTH)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ChunkLocalPosition {
    pub chunk: vec3i,
    pub local: vec3u5,
}

impl From<CubePosition> for ChunkLocalPosition {
    fn from(pos: CubePosition) -> Self {
        ChunkLocalPosition {
            chunk: pos.0 >> 5,
            local: pos.0.bitand(LENGTH as i32 - 1).into()
        }
    }
}

pub struct ChunkUpdate {
    pub overwrites: HashMap<vec3u5, Cube<Option<Material>>>,
}