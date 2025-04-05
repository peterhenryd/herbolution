pub mod chunk;
pub mod entity;
pub mod map;

use crate::world::entity::set::EntitySet;
use std::borrow::Borrow;
use std::random::random;

pub struct World {
    id: WorldId,
    chunk_map: chunk::Map,
    pub(crate) entity_set: EntitySet,
}

impl World {
    pub fn create(id: impl Into<WorldId>, clientbound: ClientboundChunks) -> Self {
        Self {
            id: id.into(),
            chunk_map: chunk::Map::new(random(), clientbound),
            entity_set: EntitySet::new(),
        }
    }

    pub fn tick(&mut self) {
        self.chunk_map.tick();
        self.entity_set.tick(&mut self.chunk_map);
    }

    #[inline]
    pub fn chunks(&self) -> &chunk::Map {
        &self.chunk_map
    }

    #[inline]
    pub fn chunks_mut(&mut self) -> &mut chunk::Map {
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WorldId(String);

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