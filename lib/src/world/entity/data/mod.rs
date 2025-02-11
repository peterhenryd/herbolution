use crate::world::entity::data::player::PlayerEntityData;

pub mod player;

#[derive(Debug)]
pub enum EntityData {
    Player(PlayerEntityData),
}

impl From<PlayerEntityData> for EntityData {
    fn from(data: PlayerEntityData) -> Self {
        Self::Player(data)
    }
}