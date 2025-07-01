use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::mem::take;

use hashbrown::HashMap;

use crate::chunk::map::ChunkMap;
use crate::entity::{ChunkLoader, EntityData};
use crate::handle::ClientHandle;
use crate::player::Player;

pub trait EntityBehavior: Debug + Send + Sync + Any {
    fn update(&mut self, ctx: &mut EntityContext<'_>);

    fn from_mut(behavior: &mut EntityBehaviorType) -> Option<&mut Self>
    where
        Self: Sized;
}

pub struct EntityContext<'a> {
    pub data: &'a mut EntityData,
    pub chunk_map: &'a mut ChunkMap,
    pub handle: &'a ClientHandle,
    pub behaviors: &'a mut EntityBehaviors,
}

#[non_exhaustive]
#[derive(Debug, Default)]
pub enum EntityBehaviorType {
    #[default]
    Noop,
    Player(Player),
    ChunkLoader(ChunkLoader),
    Dyn(Box<dyn EntityBehavior>),
}

impl EntityBehaviorType {
    pub fn update(&mut self, ctx: &mut EntityContext) {
        match self {
            EntityBehaviorType::Player(logic) => logic.update(ctx),
            EntityBehaviorType::ChunkLoader(loader) => loader.update(ctx),
            EntityBehaviorType::Dyn(logic) => logic.update(ctx),
            _ => {}
        }
    }
}

impl From<Player> for EntityBehaviorType {
    fn from(value: Player) -> Self {
        EntityBehaviorType::Player(value)
    }
}

impl From<ChunkLoader> for EntityBehaviorType {
    fn from(value: ChunkLoader) -> Self {
        EntityBehaviorType::ChunkLoader(value)
    }
}

#[derive(Debug)]
pub struct EntityBehaviors {
    vec: Vec<EntityBehaviorType>,
    indices: HashMap<TypeId, usize>,
}

impl EntityBehaviors {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            indices: HashMap::new(),
        }
    }

    pub fn update(&mut self, data: &mut EntityData, chunk_map: &mut ChunkMap, handle: &ClientHandle) {
        for i in 0..self.vec.len() {
            let mut behavior = take(&mut self.vec[i]);

            behavior.update(&mut EntityContext {
                data,
                chunk_map,
                handle,
                behaviors: self,
            });

            self.vec[i] = behavior;
        }
    }

    pub fn add<T: EntityBehavior + Into<EntityBehaviorType> + 'static>(&mut self, behavior: T) {
        let index = self.vec.len();
        self.vec.push(behavior.into());
        self.indices.insert(TypeId::of::<T>(), index);
    }

    pub fn with<T: EntityBehavior + Into<EntityBehaviorType> + 'static>(mut self, behavior: T) -> Self {
        self.add(behavior);
        self
    }

    pub fn get_mut<T: EntityBehavior>(&mut self) -> &mut T {
        let index = *self.indices.get(&TypeId::of::<T>()).unwrap();

        T::from_mut(&mut self.vec[index]).unwrap()
    }
}
