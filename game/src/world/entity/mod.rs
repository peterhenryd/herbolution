use crate::world::chunk::map::ChunkMap;
use crate::world::entity::body::EntityBody;
use crate::world::entity::logic::EntityLogic;

pub mod logic;
pub mod body;
pub mod set;

#[derive(Debug)]
pub struct Entity {
    pub data: EntityData,
    pub logic: Box<dyn EntityLogic>,
}

#[derive(Debug)]
pub struct EntityData {
    pub body: EntityBody,
    pub abilities: EntityAbilities,
}

impl Entity {
    pub fn tick(&mut self, chunk_map: &mut ChunkMap) {
        self.logic.tick(&mut self.data, chunk_map);
        self.data.body.update(chunk_map, self.data.abilities);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EntityAbilities {
    pub is_affected_by_gravity: bool,
}
