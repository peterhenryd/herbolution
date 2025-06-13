use crate::chunk::map::ChunkMap;
use crate::entity::EntityData;
use std::fmt::Debug;

pub mod player;

pub trait EntityLogic: Send + Debug {
    fn tick(&mut self, data: &mut EntityData, chunk_map: &mut ChunkMap);
}