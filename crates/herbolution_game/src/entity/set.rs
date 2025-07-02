use generational_arena::{Arena, Index, Iter, IterMut};
use std::time::Duration;

use crate::chunk::map::ChunkMap;
use crate::entity::Entity;
use crate::handle::ClientHandle;

#[derive(Debug)]
pub struct EntitySet {
    arena: Arena<Entity>,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct EntityId(Index);

impl EntitySet {
    pub fn new() -> Self {
        Self { arena: Arena::new() }
    }

    pub fn add(&mut self, entity: Entity) -> EntityId {
        EntityId(self.arena.insert(entity))
    }

    pub fn update(&mut self, handle: &ClientHandle, chunk_map: &mut ChunkMap, dt: Duration) {
        for (_, entity) in self.arena.iter_mut() {
            entity.update(chunk_map, handle, dt);
        }
    }

    pub fn get(&self, id: EntityId) -> Option<&Entity> {
        self.arena.get(id.0)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.arena.get_mut(id.0)
    }

    pub fn iter(&self) -> EntityIter<'_> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a EntitySet {
    type Item = (EntityId, &'a Entity);
    type IntoIter = EntityIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EntityIter(self.arena.iter())
    }
}

impl<'a> IntoIterator for &'a mut EntitySet {
    type Item = (EntityId, &'a mut Entity);
    type IntoIter = EntityIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EntityIterMut(self.arena.iter_mut())
    }
}

pub struct EntityIter<'a>(Iter<'a, Entity>);

impl<'a> Iterator for EntityIter<'a> {
    type Item = (EntityId, &'a Entity);

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(index, entity)| (EntityId(index), entity))
    }
}

pub struct EntityIterMut<'a>(IterMut<'a, Entity>);

impl<'a> Iterator for EntityIterMut<'a> {
    type Item = (EntityId, &'a mut Entity);

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(index, entity)| (EntityId(index), entity))
    }
}
