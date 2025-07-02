use std::path::PathBuf;
use std::sync::Arc;

use crate::chunk::handle::{ChunkCube, ClientChunkHandle, CubeUpdate};
use crate::chunk::mesh::CubeMesh;
use lib::point::ChunkPt;
use parking_lot::RwLock;

pub mod codec;
pub mod cube;
pub mod handle;
pub mod map;
pub mod material;
pub mod mesh;
pub mod provider;

#[derive(Debug)]
pub struct Chunk {
    pub position: ChunkPt,
    mesh: Arc<RwLock<CubeMesh>>,
    handle: ClientChunkHandle,
}

impl Chunk {
    pub fn new(mesh: CubeMesh, handle: ClientChunkHandle) -> Self {
        Self {
            position: mesh.position,
            mesh: Arc::new(RwLock::new(mesh)),
            handle,
        }
    }

    fn sync_with_client(&self) {
        let Some(mesh) = self.mesh.try_read() else { return };

        if mesh.updated_positions.is_empty() {
            return;
        }

        self.handle
            .set_rendered(!mesh.exposed_faces.is_empty());

        mesh.palette.update(&self.handle);

        drop(mesh);

        let mesh = self.mesh.clone();
        let sender = self.handle.cube_update.clone();

        rayon::spawn(move || {
            let mut mesh = mesh.write();
            let mesh = &mut *mesh;

            let overwrites = mesh
                .updated_positions
                .drain(..)
                .map(|position| ChunkCube {
                    position,
                    cube: mesh.data[position.linearize()],
                })
                .collect();

            let _ = sender.send(CubeUpdate { overwrites });
        });
    }

    pub fn update(&self, _: PathBuf) {
        self.sync_with_client();
    }
}
