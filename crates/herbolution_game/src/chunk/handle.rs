use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crossbeam::channel::{unbounded, Receiver, Sender};
use lib::display::Join;
use lib::point::ChunkPt;
use math::vec::vec3u5;
use smallvec::SmallVec;
use tracing::error;

use crate::chunk::material::{Material, PaletteCube};

#[derive(Debug)]
pub struct ChunkLoad {
    pub position: ChunkPt,
    pub handle: GameChunkHandle,
}

#[derive(Debug)]
pub struct CubeUpdate {
    pub overwrites: SmallVec<ChunkCube, 64>,
}

pub struct PaletteUpdate {
    pub material: Arc<Material>,
}

#[derive(Debug)]
pub struct ChunkCube {
    pub position: vec3u5,
    pub cube: PaletteCube,
}

#[derive(Debug)]
pub struct GameChunkHandle {
    rendered: Arc<AtomicBool>,
    pub cube_update: Receiver<CubeUpdate>,
    palette_update: Receiver<PaletteUpdate>,
}

impl GameChunkHandle {
    pub fn is_rendered(&self) -> bool {
        self.rendered.load(Ordering::Relaxed)
    }

    pub fn next_cube_update(&self) -> Option<CubeUpdate> {
        self.cube_update.try_recv().ok()
    }

    pub fn next_palette_update(&self) -> Option<PaletteUpdate> {
        self.palette_update.try_recv().ok()
    }
}

#[derive(Debug)]
pub struct ClientChunkHandle {
    position: ChunkPt,
    rendered: Arc<AtomicBool>,
    pub(crate) cube_update: Sender<CubeUpdate>,
    palette_update: Sender<PaletteUpdate>,
}

impl ClientChunkHandle {
    pub fn set_rendered(&self, rendered: bool) {
        self.rendered.store(rendered, Ordering::Relaxed);
    }

    pub fn send_cube_update(&self, items: impl IntoIterator<Item = ChunkCube>) {
        if let Err(e) = self.cube_update.try_send(CubeUpdate {
            overwrites: SmallVec::<_, 64>::from_iter(items),
        }) {
            error!("Failed to send update for chunk at {}: {e}", Join::new(&self.position.0, ","));
        }
    }

    pub fn register_material(&self, material: Arc<Material>) {
        if let Err(e) = self
            .palette_update
            .try_send(PaletteUpdate { material })
        {
            error!("Failed to send update for chunk at {}: {e}", Join::new(&self.position.0, ","));
        }
    }
}

pub fn create(position: ChunkPt) -> (GameChunkHandle, ClientChunkHandle) {
    let rendered = Arc::new(AtomicBool::new(true));
    let (cube_update_tx, cube_update_rx) = unbounded();
    let (palette_update_tx, palette_update_rx) = unbounded();

    (
        GameChunkHandle {
            rendered: Arc::clone(&rendered),
            cube_update: cube_update_rx,
            palette_update: palette_update_rx,
        },
        ClientChunkHandle {
            position,
            rendered,
            cube_update: cube_update_tx,
            palette_update: palette_update_tx,
        },
    )
}
