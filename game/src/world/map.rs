use crossbeam::channel::Sender;
use hashbrown::HashMap;
use crate::Response;
use crate::world::{World, WorldId};

pub struct WorldMap {
    sender: Sender<Response>,
    map: HashMap<WorldId, World>,
}

impl WorldMap {
    pub fn new(sender: Sender<Response>) -> Self {
        Self {
            map: HashMap::new(),
            sender
        }
    }

    pub fn primary(&mut self) -> &mut World {
        if !self.map.contains_key("world") {
            self.insert(World::create("world", self.sender.clone()));
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