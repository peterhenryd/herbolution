use std::net::SocketAddr;

pub enum GameAddress {
    Local(LocalGameAddress),
    Remote(RemoteGameAddress),
}

impl From<LocalGameAddress> for GameAddress {
    fn from(local_game_address: LocalGameAddress) -> Self {
        Self::Local(local_game_address)
    }
}

impl From<RemoteGameAddress> for GameAddress {
    fn from(remote_game_address: RemoteGameAddress) -> Self {
        Self::Remote(remote_game_address)
    }
}

pub struct RemoteGameAddress(SocketAddr);
