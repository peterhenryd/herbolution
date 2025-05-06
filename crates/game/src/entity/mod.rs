use crate::chunk;
use crate::chunk::map::ChunkMap;
use crate::entity::body::EntityBody;
use crate::entity::logic::player::PlayerLogic;
use crate::entity::logic::EntityLogic;
use crate::entity::set::EntityId;
use hashbrown::HashSet;
use math::num::traits::ConstZero;
use math::vector::{vec3f, vec3i, Vec3};
use tokio::sync::Mutex;

pub mod logic;
pub mod body;
pub mod set;

#[derive(Debug)]
pub struct Entity {
    pub data: Mutex<EntityData>,
    pub logic: EntityLogicVariant,
}

impl Entity {
    pub async fn tick(&mut self, chunk_map: &Mutex<ChunkMap>) {
        self.data.lock().await.tick(&mut *chunk_map.lock().await).await;
        self.logic.tick(&self.data, chunk_map).await;
    }
}

#[derive(Debug)]
pub struct EntityData {
    pub body: EntityBody,
    pub abilities: EntityAbilities,
    pub(crate) chunk_loader: Option<ChunkLoader>,
}

impl EntityData {
    pub async fn tick(&mut self, chunk_map: &mut ChunkMap) {
        if let Some(chunk_loader) = &mut self.chunk_loader {
            chunk_loader.reload_radial_chunks(self.body.pos, chunk_map);
        }

        self.body.update(chunk_map, self.abilities).await;
    }
}

#[derive(Debug)]
pub enum EntityLogicVariant {
    Player(PlayerLogic),
    Custom(Box<dyn EntityLogic>),
}

impl EntityLogicVariant {
    pub async fn tick(&mut self, data: &Mutex<EntityData>, chunk_map: &Mutex<ChunkMap>) {
        match self {
            EntityLogicVariant::Player(logic) => logic.tick(data, chunk_map).await,
            EntityLogicVariant::Custom(logic) => logic.tick(data, chunk_map).await,
        }
    }
}

#[derive(Debug)]
pub struct ChunkLoader {
    pub(crate) prev_chunk_pos: vec3i,
    owned_chunk_pos: HashSet<vec3i>,
    wait: u32,
}

impl ChunkLoader {
    pub fn new() -> Self {
        Self {
            prev_chunk_pos: Vec3::ZERO,
            owned_chunk_pos: HashSet::new(),
            wait: 120,
        }
    }

    pub fn reload_radial_chunks(&mut self, pos: vec3f, chunk_map: &mut ChunkMap) {
        let chunk_pos = pos.cast().unwrap() / chunk::LENGTH as i32;

        if chunk_pos == self.prev_chunk_pos {
            return;
        } else if self.wait >= 120 {
            self.prev_chunk_pos = chunk_pos;
            self.wait = 0;
        } else {
            self.wait += 1;
            return;
        }

        let radial_pos = rhombus(chunk_pos, 16);

        let remove_pos = self.owned_chunk_pos.iter()
            .filter(|&pos| !radial_pos.contains(pos));
        for pos in remove_pos {
            chunk_map.unload_chunk(*pos);
        }

        for &pos in &radial_pos {
            chunk_map.load_chunk(pos);
        }

        self.owned_chunk_pos = radial_pos;
    }
}

fn rhombus(center: vec3i, radius: i32) -> HashSet<vec3i> {
    let mut pos = HashSet::new();

    for x in -radius..=radius {
        for y in -3i32..3 {
            for z in -radius..=radius {
                if x.abs() + y.abs() + z.abs() <= radius {
                    pos.insert(center + vec3i::new(x, y, z));
                }
            }
        }
    }

    pos
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