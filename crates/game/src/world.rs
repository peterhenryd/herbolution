use crate::chunk::channel::ServerChunkChannel;
use crate::chunk::map::ChunkMap;
use crate::entity::set::EntitySet;
use hashbrown::HashMap;
use std::path::PathBuf;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct World {
    id: String,
    chunk_map: Mutex<ChunkMap>,
    pub(crate) entity_set: EntitySet,
}

#[derive(Debug)]
pub struct WorldMap {
    channel: ServerChunkChannel,
    map: HashMap<String, World>,
}

impl World {
    pub fn create(id: impl Into<String>, channel: ServerChunkChannel) -> Self {
        let id = id.into();
        Self {
            chunk_map: Mutex::new(ChunkMap::new(0, channel, PathBuf::from(".").join(&id))),
            entity_set: EntitySet::new(),
            id,
        }
    }

    pub async fn tick(&mut self) {
        self.chunk_map.lock().await.tick();
        self.entity_set.tick(&self.chunk_map).await;
    }
}

impl WorldMap {
    pub fn new(channel: ServerChunkChannel) -> Self {
        Self {
            channel,
            map: HashMap::new(),
        }
    }

    pub fn primary(&mut self) -> &mut World {
        if !self.map.contains_key("world") {
            self.insert(World::create("world", self.channel.clone()));
        }

        self.map.get_mut("world").unwrap()
    }

    pub fn insert(&mut self, world: World) {
        self.map.insert(world.id.clone(), world);
    }

    pub async fn tick(&mut self) {
        for (_, world) in &mut self.map {
            world.tick().await;
        }
    }
}