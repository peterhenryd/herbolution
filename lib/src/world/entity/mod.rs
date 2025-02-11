use crate::world::chunk::map::ChunkMap;
use crate::world::entity::data::EntityData;
use crate::world::entity::physics::EntityPhysics;
use math::vector::vec3f;
use std::time::Duration;

pub mod set;
pub mod data;
pub mod physics;

#[derive(Debug)]
pub struct Entity {
    pub physics: EntityPhysics,
    pub data: Box<EntityData>,
    pub abilities: EntityAbilities,
}

pub struct Motion {
    pub direction: vec3f,
    pub speed: f32,
}

impl Entity {
    pub fn update(&mut self, dt: Duration, chunk_map: &mut ChunkMap) {
        self.physics.update(dt, chunk_map, self.abilities);

        match self.data.as_mut() {
            EntityData::Player(data) => data.update(dt, chunk_map, &mut self.physics, self.abilities),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EntityAbilities {
    pub is_affected_by_gravity: bool,
}