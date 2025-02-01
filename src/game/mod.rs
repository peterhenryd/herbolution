use crate::game::address::GameAddress;
use crate::world::World;

pub mod address;

pub struct LocalGame;

pub struct RemoteGame;

/// This represents an individual player's session connection to either a local or remote game.
///
/// It does not represent the game itself, but rather provides an interface for querying game state
/// changes as they occur. It also provides an interface for sending player input to the game.
pub enum Game {
    Local(LocalGame),
    Remote(RemoteGame),
}

impl Game {
    pub fn connect(address: GameAddress) -> Self {
        match address {
            GameAddress::Local(_) => Self::Local(LocalGame),
            GameAddress::Remote(_) => Self::Remote(RemoteGame),
        }
    }

    pub fn update(&mut self, _world: &mut World) {
        todo!()
    }
}