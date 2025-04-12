use std::borrow::Borrow;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::chunk::channel::ServerChunkChannel;
use crate::chunk::map::ChunkMap;
use crate::entity::set::EntitySet;

#[derive(Debug)]
pub struct World {
    id: WorldId,
    chunk_map: ChunkMap,
    pub(crate) entity_set: EntitySet,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WorldId(String);

#[derive(Debug)]
pub struct WorldMap {
    channel: ServerChunkChannel,
    map: HashMap<WorldId, World>,
}

impl World {
    pub fn create(id: impl Into<WorldId>, channel: ServerChunkChannel) -> Self {
        let id = id.into();
        Self {
            chunk_map: ChunkMap::new(0, channel, PathBuf::from(".").join(id.0.clone())),
            entity_set: EntitySet::new(),
            id,
        }
    }

    pub async fn tick(&mut self) {
        self.chunk_map.tick();
        self.entity_set.tick(&mut self.chunk_map).await;
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

impl Borrow<str> for WorldId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl<'a> From<&'a str> for WorldId {
    fn from(value: &'a str) -> Self {
        Self(value.to_owned())
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