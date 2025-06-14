use crate::chunk::channel::ServerChunkChannel;
use crate::chunk::map::ChunkMap;
use crate::entity::set::EntitySet;
use lib::fs::save::SaveWorld;

#[derive(Debug)]
pub struct World {
    //name: String,
    chunk_map: ChunkMap,
    pub(crate) entity_set: EntitySet,
}

impl World {
    pub fn from_save(save: SaveWorld, channel: ServerChunkChannel) -> Self {
        Self {
            //name: save.name,
            chunk_map: ChunkMap::new(save.descriptor.seed, channel, save.path),
            entity_set: EntitySet::new(),
        }
    }

    pub fn tick(&mut self) {
        self.chunk_map.tick();
        self.entity_set.tick(&mut self.chunk_map);
    }
}
