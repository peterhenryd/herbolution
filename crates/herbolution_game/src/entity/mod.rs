use std::any::Any;

use hashbrown::HashSet;
use lib::chunk;
use lib::point::ChunkPt;
use math::vec::vec3i;

use crate::chunk::map::ChunkMap;
use crate::entity::behavior::{EntityBehavior, EntityBehaviorType};
use crate::entity::behavior::{EntityBehaviors, EntityContext};
use crate::entity::body::EntityBody;
use crate::entity::set::EntityId;
use crate::handle::ClientHandle;

pub mod behavior;
pub mod body;
pub mod set;

#[derive(Debug)]
pub struct Entity {
    pub data: EntityData,
    pub behaviors: EntityBehaviors,
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
    pub body: EntityBody,
    pub abilities: EntityAbilities,
}

impl EntityData {
    pub fn update(&mut self, chunk_map: &mut ChunkMap) {
        self.body.update(chunk_map, self.abilities);
    }
}

#[derive(Debug)]
pub struct ChunkLoader {
    pub(crate) prev_chunk_position: ChunkPt,
    radial_chunk_positions: HashSet<ChunkPt>,
}

impl ChunkLoader {
    pub fn new() -> Self {
        Self {
            prev_chunk_position: ChunkPt::ZERO,
            radial_chunk_positions: HashSet::new(),
        }
    }
}

impl EntityBehavior for ChunkLoader {
    fn update(&mut self, ctx: &mut EntityContext<'_>) {
        let chunk_position = ChunkPt(ctx.data.body.position.cast().unwrap() / chunk::LENGTH as i32);
        if chunk_position == self.prev_chunk_position {
            return;
        }

        self.prev_chunk_position = chunk_position;

        let new_positions = fill_rhombus(chunk_position, 16);

        self.radial_chunk_positions
            .iter()
            .filter(|&x| !new_positions.contains(x))
            .for_each(|&x| ctx.chunk_map.queue_unload(x));

        self.radial_chunk_positions = new_positions;

        for &chunk_position in &self.radial_chunk_positions {
            ctx.chunk_map.queue_load(chunk_position);
        }
    }

    fn from_mut(behavior: &mut EntityBehaviorType) -> Option<&mut Self>
    where
        Self: Sized,
    {
        match behavior {
            EntityBehaviorType::Player(_) => None,
            EntityBehaviorType::ChunkLoader(x) => Some(x),
            EntityBehaviorType::Dyn(x) => (x.as_mut() as &mut dyn Any).downcast_mut(),
            _ => None,
        }
    }
}

fn fill_rhombus(center: ChunkPt, radius: i32) -> HashSet<ChunkPt> {
    let mut set = HashSet::new();
    for x in -radius..=radius {
        for y in -radius / 6..=radius / 6 {
            for z in -radius..=radius {
                if x.abs() + y.abs() + z.abs() <= radius {
                    set.insert(ChunkPt(center.0 + vec3i::new(x, y, z)));
                }
            }
        }
    }
    set
}

#[derive(Debug, Copy, Clone)]
pub struct EntityAbilities {
    pub is_affected_by_gravity: bool,
    pub speed: f64,
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
