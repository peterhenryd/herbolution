use hashbrown::HashMap;
use crate::world::{World, WorldId};
use crate::world::chunk::channel::ServerChunkChannel;

pub struct WorldMap {
    channel: ServerChunkChannel,
    map: HashMap<WorldId, World>,
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

    pub fn tick(&mut self) {
        for (_, world) in &mut self.map {
            world.tick();
        }
    }
}