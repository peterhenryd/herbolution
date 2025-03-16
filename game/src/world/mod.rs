pub mod chunk;
pub mod entity;
pub mod map;

use std::borrow::Borrow;
use kanal::Sender;
use math::vector::Vec3;
use crate::Response;
use crate::world::chunk::map::ChunkMap;
use crate::world::entity::set::EntitySet;

pub struct World {
    id: WorldId,
    chunk_map: ChunkMap,
    pub(crate) entity_set: EntitySet,
}

impl World {
    pub fn create(id: WorldId, sender: Sender<Response>) -> Self {
        let mut chunk_map = ChunkMap::new(48323, sender);
        for x in -2..2 {
            for y in 0..3 {
                for z in -2..2 {
                    chunk_map.load_chunk(Vec3::new(x, y, z));
                }
            }
        }

        Self {
            id,
            chunk_map,
            entity_set: EntitySet::new(),
        }
    }

    pub fn tick(&mut self) {
        self.chunk_map.tick();
        self.entity_set.tick(&mut self.chunk_map);
    }

    #[inline]
    pub fn chunks(&self) -> &ChunkMap {
        &self.chunk_map
    }

    #[inline]
    pub fn chunks_mut(&mut self) -> &mut ChunkMap {
        &mut self.chunk_map
    }

    #[inline]
    pub fn entities(&self) -> &EntitySet {
        &self.entity_set
    }

    #[inline]
    pub fn entities_mut(&mut self) -> &mut EntitySet {
        &mut self.entity_set
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WorldId(String);

impl Borrow<str> for WorldId {
    fn borrow(&self) -> &str {
        &self.0
    }
}