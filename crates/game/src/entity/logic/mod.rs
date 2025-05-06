use crate::chunk::map::ChunkMap;
use crate::entity::EntityData;
use futures::future::BoxFuture;
use std::fmt::Debug;
use tokio::sync::Mutex;

pub mod player;

pub trait EntityLogic: Send + Debug {
    fn tick<'a>(&'a self, data: &'a Mutex<EntityData>, chunk_map: &'a Mutex<ChunkMap>) -> BoxFuture<'a, ()>;
}