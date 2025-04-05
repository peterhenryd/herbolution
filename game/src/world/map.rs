use hashbrown::HashMap;
use crate::channel::ClientboundChunks;
use crate::world::{World, WorldId};

pub struct WorldMap {
    channel: ClientboundChunks,
    map: HashMap<WorldId, World>,
}

impl WorldMap {
    pub fn new(channel: ClientboundChunks) -> Self {
        Self {
            map: HashMap::new(),
            channel,
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

    pub fn tick(&mut self) {
        for (_, world) in &mut self.map {
            world.tick();
        }
    }
}