use std::time::Duration;

use lib::save::SaveWorld;
use tracing::info;

use crate::chunk::map::ChunkMap;
use crate::entity::set::EntitySet;
use crate::generator::CHUNK_GENERATION_TIME;
use crate::handle::ClientHandle;

#[derive(Debug)]
pub struct World {
    chunk_map: ChunkMap,
    pub(crate) entity_set: EntitySet,
    acc: Duration,
}

impl World {
    pub fn from_save(save: SaveWorld) -> Self {
        Self {
            chunk_map: ChunkMap::new(save.descriptor.seed, save.path),
            entity_set: EntitySet::new(),
            acc: Duration::ZERO,
        }
    }

    pub fn update(&mut self, handle: &ClientHandle, dt: Duration) {
        self.acc += dt;
        if self.acc >= Duration::SECOND {
            self.acc -= Duration::SECOND;
            info!("Avg. chunk generation time: {:?}", CHUNK_GENERATION_TIME.average(),);
        }

        self.chunk_map.update(handle);
        self.entity_set
            .update(handle, &mut self.chunk_map, dt);
    }
}
