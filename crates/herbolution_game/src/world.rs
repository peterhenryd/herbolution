use crate::chunk::map::ChunkMap;
use crate::entity::set::EntitySet;
use crate::handle::ClientHandle;
use lib::save::SaveWorld;

#[derive(Debug)]
pub struct World {
    //name: String,
    chunk_map: ChunkMap,
    pub(crate) entity_set: EntitySet,
}

impl World {
    pub fn from_save(save: SaveWorld) -> Self {
        Self {
            //name: save.name,
            chunk_map: ChunkMap::new(save.descriptor.seed, save.path),
            entity_set: EntitySet::new(),
        }
    }

    pub fn update(&mut self, handle: &ClientHandle) {
        self.chunk_map.update(handle);
        self.entity_set
            .update(handle, &mut self.chunk_map);
    }
}
