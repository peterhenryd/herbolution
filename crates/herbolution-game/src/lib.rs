use crate::address::GameAddress;
use crate::local::LocalGame;
use crate::message::InputMessage;
use crate::remote::RemoteGame;

pub mod address;
pub mod chunk;
pub mod local;
pub mod message;
pub mod remote;
pub mod server;

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
            GameAddress::Local(address) => Self::Local(LocalGame::connect(address)),
            GameAddress::Remote(_) => Self::Remote(RemoteGame),
        }
    }

    pub fn update_world(&mut self, world: &mut World) {
        match self {
            Game::Local(game) => game.update_world(world),
            Game::Remote(_) => {}
        }
    }

    pub fn send_input_message(&mut self, message: InputMessage) {
        match self {
            Game::Local(game) => game.send_input_message(message),
            Game::Remote(_) => {}
        }
    }
}
