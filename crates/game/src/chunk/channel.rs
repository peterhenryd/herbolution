use crossbeam::channel::{unbounded, Receiver, Sender};
use hashbrown::HashMap;
use math::vector::{vec3i, vec3u5};
use crate::chunk::cube::Cube;
use crate::chunk::material::Material;

pub struct ClientChunkChannel {
    load: Receiver<(vec3i, Receiver<ChunkUpdate>)>,
    unload: Receiver<vec3i>,
}

#[derive(Debug, Clone)]
pub struct ServerChunkChannel {
    load: Sender<(vec3i, Receiver<ChunkUpdate>)>,
    unload: Sender<vec3i>,
}

pub struct ChunkUpdate {
    pub overwrites: HashMap<vec3u5, Cube<Option<Material>>>,
}

impl ClientChunkChannel {
    pub fn recv_load(&self) -> Option<(vec3i, Receiver<ChunkUpdate>)> {
        self.load.try_recv().ok()
    }

    pub fn recv_unload(&self) -> Option<vec3i> {
        self.unload.try_recv().ok()
    }
}

impl ServerChunkChannel {
    pub fn send_load(&self, pos: vec3i, receiver: Receiver<ChunkUpdate>) {
        if let Err(e) = self.load.try_send((pos, receiver)) {
            tracing::warn!("Failed to send chunk load: {}", e);
        }
    }

    pub fn send_unload(&self, chunk: vec3i) {
        if let Err(e) = self.unload.try_send(chunk) {
            tracing::warn!("Failed to send chunk unload: {}", e);
        }
    }
}

pub fn create() -> (ClientChunkChannel, ServerChunkChannel) {
    let (load_tx, load_rx) = unbounded();
    let (unload_tx, unload_rx) = unbounded();

    (
        ClientChunkChannel { load: load_rx, unload: unload_rx },
        ServerChunkChannel { load: load_tx, unload: unload_tx },
    )
}