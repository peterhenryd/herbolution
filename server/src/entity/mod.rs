use lib::vector::vec3i;
use time::Duration;

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
    pub fn update(&mut self, chunk_map: &mut ChunkMap, handle: &ClientHandle, dt: Duration) {
        self.data.update(chunk_map, dt);
        self.behaviors
            .update(&mut self.data, chunk_map, handle, dt);
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

    pub fn update(&mut self, chunk_map: &mut ChunkMap, dt: Duration) {
        self.body.update(chunk_map, dt);
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ActionState {
    pub is_left_hand_active: bool,
    pub is_right_hand_active: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ActionTarget {
    Cube(CubeTarget),
    Entity(EntityId),
}

impl ActionTarget {
    pub fn stateless_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ActionTarget::Cube(a), ActionTarget::Cube(b)) => a.position == b.position,
            (ActionTarget::Entity(a), ActionTarget::Entity(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CubeTarget {
    pub position: vec3i,
}
