use std::fmt::Debug;
use crate::world::chunk::map::ChunkMap;
use crate::world::entity::EntityData;

pub mod player;

pub trait EntityLogic: Debug {
    fn tick(&mut self, data: &mut EntityData, chunk_map: &mut ChunkMap);
}