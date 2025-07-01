use math::vector::vec3i;

use crate::chunk::map::ChunkMap;
use crate::entity::behavior::EntityBehaviors;
use crate::entity::body::EntityBody;
use crate::entity::set::EntityId;
use crate::handle::ClientHandle;

pub mod behavior;
pub mod body;
pub mod components;
pub mod set;

#[derive(Debug)]
pub struct Entity {
    pub(crate) data: EntityData,
    pub(crate) behaviors: EntityBehaviors,
}

impl Entity {
    pub fn update(&mut self, chunk_map: &mut ChunkMap, handle: &ClientHandle) {
        self.data.update(chunk_map);
        self.behaviors
            .update(&mut self.data, chunk_map, handle);
    }
}

#[derive(Debug)]
pub struct EntityData {
    pub(crate) body: EntityBody,
}

impl EntityData {
    pub fn body(&self) -> &EntityBody {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut EntityBody {
        &mut self.body
    }

    pub fn update(&mut self, chunk_map: &mut ChunkMap) {
        self.body.update(chunk_map);
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ActionState {
    pub is_left_hand_active: bool,
    pub is_right_hand_active: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ActionTarget {
    Cube(vec3i),
    Entity(EntityId),
}
