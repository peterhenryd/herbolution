use crate::world::chunk::map::ChunkMap;
use crate::world::entity::Entity;
use generational_arena::{Arena, Index};
use std::time::Duration;

#[derive(Debug)]
pub struct EntitySet {
    arena: Arena<Entity>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct EntityId(Index);

impl EntitySet {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
        }
    }

    pub fn add(&mut self, entity: Entity) -> EntityId {
        EntityId(self.arena.insert(entity))
    }

    pub fn update(&mut self, dt: Duration, chunk_map: &mut ChunkMap) {
        for (_, entity) in self.arena.iter_mut() {
            entity.update(dt, chunk_map);
        }
    }

    pub fn get(&self, id: EntityId) -> Option<&Entity> {
        self.arena.get(id.0)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.arena.get_mut(id.0)
    }
}