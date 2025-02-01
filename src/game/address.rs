use std::net::SocketAddr;
use std::path::PathBuf;

pub enum GameAddress {
    Local(LocalGameAddress),
    Remote(RemoteGameAddress),
}

pub struct LocalGameAddress(PathBuf);

pub struct RemoteGameAddress(SocketAddr);