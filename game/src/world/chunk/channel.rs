use crossbeam::channel::{unbounded, Receiver, Sender};
use math::vector::vec3i;
use crate::world::chunk::ChunkUpdate;

pub struct ServerChunkChannel {
    loaded: Sender<ChunkLoaded>,
    unloaded: Sender<vec3i>,
}

pub struct ClientChunkChannel {
    loaded: Receiver<ChunkLoaded>,
    unloaded: Receiver<vec3i>,
}

pub struct ChunkLoaded {
    pub position: vec3i,
    pub receiver: Receiver<ChunkUpdate>,
}

impl ServerChunkChannel {
    pub fn send_loaded(&self, position: vec3i, receiver: Receiver<ChunkUpdate>) {
        if let Err(e) = self.loaded.try_send(ChunkLoaded { position, receiver }) {
            eprintln!("Failed to send request to load chunk: {e}");
        }
    }

    pub fn send_unloaded(&self, position: vec3i) {
        if let Err(e) = self.unloaded.try_send(position) {
            eprintln!("Failed to send request to unload chunk: {e}");
        }
    }
}

impl ClientChunkChannel {
    pub fn recv_loaded(&self) -> Option<ChunkLoaded> {
        self.loaded.try_recv().ok()
    }

    pub fn recv_unloaded(&self) -> Option<vec3i> {
        self.unloaded.try_recv().ok()
    }
}

pub fn create() -> (ClientChunkChannel, ServerChunkChannel) {
    let (loaded_tx, loaded_rx) = unbounded();
    let (unloaded_tx, unloaded_rx) = unbounded();

    (
        ClientChunkChannel {
            loaded: loaded_rx,
            unloaded: unloaded_rx,
        },
        ServerChunkChannel {
            loaded: loaded_tx,
            unloaded: unloaded_tx,
        },
    )
}