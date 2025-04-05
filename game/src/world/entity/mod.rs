use hashbrown::HashSet;
use math::num::traits::ConstZero;
use math::vector::{vec3f, vec3i, Vec3};
use crate::world::chunk;
use crate::world::entity::body::EntityBody;
use crate::world::entity::logic::EntityLogic;
use crate::world::entity::logic::player::PlayerLogic;
use crate::world::entity::set::EntityId;

pub mod logic;
pub mod body;
pub mod set;

#[derive(Debug)]
pub struct Entity {
    pub data: EntityData,
    pub logic: EntityLogicVariant,
}

impl Entity {
    pub fn tick(&mut self, chunk_map: &mut chunk::Map) {
        self.data.tick(chunk_map);
        self.logic.tick(&mut self.data, chunk_map);
    }
}

#[derive(Debug)]
pub struct EntityData {
    pub body: EntityBody,
    pub abilities: EntityAbilities,
    pub(crate) chunk_loader: Option<ChunkLoader>,
}

impl EntityData {
    pub fn tick(&mut self, chunk_map: &mut chunk::Map) {
        if let Some(chunk_loader) = &mut self.chunk_loader {
            chunk_loader.reload_radial_chunks(self.body.pos, chunk_map);
        }

        self.body.update(chunk_map, self.abilities);
    }
}

#[derive(Debug)]
pub enum EntityLogicVariant {
    Player(PlayerLogic),
    Custom(Box<dyn EntityLogic>),
}

impl EntityLogicVariant {
    pub fn tick(&mut self, data: &mut EntityData, chunk_map: &mut chunk::Map) {
        match self {
            EntityLogicVariant::Player(logic) => logic.tick(data, chunk_map),
            EntityLogicVariant::Custom(logic) => logic.tick(data, chunk_map),
        }
    }
}

#[derive(Debug)]
pub struct ChunkLoader {
    pub(crate) prev_chunk_position: vec3i,
    owned_chunk_positions: HashSet<vec3i>,
    pub(crate) radius: i32,
    prev_radius: i32,
}

impl ChunkLoader {
    pub fn new(radius: i32) -> Self {
        Self {
            prev_chunk_position: Vec3::ZERO,
            owned_chunk_positions: HashSet::new(),
            radius,
            prev_radius: radius,
        }
    }

    pub fn reload_radial_chunks(&mut self, position: vec3f, chunk_map: &mut chunk::Map) {
        let chunk_position = position.cast().unwrap() / chunk::LENGTH as i32;

        if chunk_position == self.prev_chunk_position && self.radius == self.prev_radius {
            return;
        } else {
            self.prev_chunk_position = chunk_position;
        }
        self.prev_radius = self.radius;

        let positions = rhombus(chunk_position, self.radius);

        let remove_positions = self.owned_chunk_positions.iter()
            .filter(|&position| !positions.contains(position));
        for position in remove_positions {
            chunk_map.unload_chunk(*position);
        }

        for &position in &positions {
            chunk_map.load_chunk(position);
        }

        self.owned_chunk_positions = positions;
    }
}

fn rhombus(center: vec3i, radius: i32) -> HashSet<vec3i> {
    let mut positions = HashSet::new();

    for x in -radius..=radius {
        for y in -1i32..=1 {
            for z in -radius..=radius {
                if x.abs() + z.abs() <= radius {
                    positions.insert(center + vec3i::new(x, y, z));
                }
            }
        }
    }

    positions
}

impl From<PlayerLogic> for EntityLogicVariant {
    fn from(logic: PlayerLogic) -> Self {
        EntityLogicVariant::Player(logic)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EntityAbilities {
    pub is_affected_by_gravity: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EntityTarget {
    Cube(vec3i),
    Entity(EntityId),
}