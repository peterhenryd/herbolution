use crate::chunk::handle::ChunkLoad;
use crate::player::handle::ServerPlayerHandle;
use crossbeam::channel::{bounded, unbounded, Receiver, Sender};
use lib::motile::Motile;
use lib::point::ChunkPt;
use math::color::Rgba;
use math::vec::vec3d;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tracing::warn;

// Game-side handles

#[derive(Debug)]
pub struct GameHandle {
    pub chunks: GameChunksHandle,
    player_handle_rx: Receiver<ServerPlayerHandle>,
    pub particle_rx: Receiver<Particle>,
    exit_signal: Arc<AtomicBool>,
}

#[derive(Debug)]
pub struct Particle {
    pub position: vec3d,
    pub motile: Motile,
    pub lifetime: Duration,
    pub color: Rgba<f32>,
}

impl GameHandle {
    pub fn next_player_handle(&self) -> Option<ServerPlayerHandle> {
        self.player_handle_rx.try_recv().ok()
    }

    pub fn request_exit(&self) {
        self.exit_signal.store(true, Ordering::Relaxed);
    }
}

#[derive(Debug)]
pub struct GameChunksHandle {
    load_rx: Receiver<ChunkLoad>,
    unload_rx: Receiver<ChunkPt>,
}

impl GameChunksHandle {
    pub fn next_load(&self) -> Option<ChunkLoad> {
        self.load_rx.try_recv().ok()
    }

    pub fn next_unload(&self) -> Option<ChunkPt> {
        self.unload_rx.try_recv().ok()
    }
}

// Client-side handles

#[derive(Debug)]
pub struct ClientHandle {
    pub chunks: ClientChunksHandle,
    player_handle_tx: Sender<ServerPlayerHandle>,
    pub(crate) particle_tx: Sender<Particle>,
    exit_signal: Arc<AtomicBool>,
}

impl ClientHandle {
    pub fn send_player_handle(&self, handle: ServerPlayerHandle) {
        if let Err(e) = self.player_handle_tx.try_send(handle) {
            warn!("Failed to send player handle: {}", e);
        }
    }

    pub fn is_exit_requested(&self) -> bool {
        self.exit_signal.load(Ordering::Relaxed)
    }
}

#[derive(Debug)]
pub struct ClientChunksHandle {
    pub load_tx: Sender<ChunkLoad>,
    pub unload_tx: Sender<ChunkPt>,
}

impl ClientChunksHandle {
    pub fn load(&self, value: ChunkLoad) {
        if let Err(e) = self.load_tx.try_send(value) {
            warn!("Failed to send chunk load: {}", e);
        }
    }

    pub fn unload(&self, value: ChunkPt) {
        if let Err(e) = self.unload_tx.try_send(value) {
            warn!("Failed to send chunk unload: {}", e);
        }
    }
}

pub fn create() -> (ClientHandle, GameHandle) {
    let (load_tx, load_rx) = unbounded();
    let (unload_tx, unload_rx) = unbounded();
    let (player_handle_tx, player_handle_rx) = bounded(1);
    let (particle_tx, particle_rx) = unbounded();
    let exit_signal = Arc::new(AtomicBool::new(false));

    (
        ClientHandle {
            chunks: ClientChunksHandle { load_tx, unload_tx },
            player_handle_tx,
            particle_tx,
            exit_signal: Arc::clone(&exit_signal),
        },
        GameHandle {
            chunks: GameChunksHandle { load_rx, unload_rx },
            player_handle_rx,
            particle_rx,
            exit_signal,
        },
    )
}
