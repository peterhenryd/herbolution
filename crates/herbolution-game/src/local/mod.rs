use crate::fs::save::LocalGameAddress;
use crate::game::chunk::material::Material;
use crate::game::chunk::ChunkLocalPosition;
use crate::game::message::{GameMessage, InputMessage};
use crate::game::server::GameServer;
use crate::world::chunk_mesh::ChunkMesh;
use crate::world::World;
use glam::IVec2;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

pub struct LocalGame {
    stx: mpsc::Sender<InputMessage>,
    crx: mpsc::Receiver<GameMessage>,
    abort_handle: AbortHandle,
}

impl LocalGame {
    pub fn connect(address: LocalGameAddress) -> Self {
        let (stx, mut srx) = mpsc::channel(64);
        let (ctx, crx) = mpsc::channel(64);

        let abort_handle = tokio::spawn(async move {
            let mut server = GameServer::new(address);

            // TODO: remove
            server.load_chunk(IVec2::new(0, 0));

            loop {
                while let Ok(input) = srx.try_recv() {
                    server.process_input(input);
                }

                let messages = server.update();
                let ctx = ctx.clone();
                tokio::spawn(async move {
                    for message in messages {
                        let _ = ctx.send(message).await;
                    }
                });
            }
        })
            .abort_handle();

        Self {
            stx,
            crx,
            abort_handle,
        }
    }

    pub fn send_input_message(&mut self, message: InputMessage) {
        let stx = self.stx.clone();
        tokio::spawn(async move {
            let _ = stx.send(message).await;
        });
    }

    pub fn update_world(&mut self, world: &mut World) {
        fn is_same_mesh(mesh: &Option<&mut ChunkMesh>, position: ChunkLocalPosition) -> bool {
            mesh.as_ref()
                .map(|x| x.position == position.chunk)
                .unwrap_or(false)
        }

        while let Ok(message) = self.crx.try_recv() {
            match message {
                GameMessage::CubeRemoved { position } => {
                    let Some(mesh) = world.chunk_meshes.get_mut(&position.chunk) else {
                        continue;
                    };
                    mesh.set_cube(position.local, Material::Air);
                }
                GameMessage::CubeAdded { position, material } => {
                    let Some(mesh) = world.chunk_meshes.get_mut(&position.chunk) else {
                        continue;
                    };
                    mesh.set_cube(position.local, material);
                }
                GameMessage::MovePlayer { velocity } => {
                    world.camera.view.position += velocity;
                }
                GameMessage::RotatePlayer { rotation } => {
                    world.camera.view.rotation = rotation;
                }
                GameMessage::Exit => self.exit(),
                GameMessage::ChunkLoaded { chunk } => {
                    for section in chunk.sections {
                        world.add_chunk_mesh(&section);
                    }
                }
                GameMessage::ChunkUnloaded { position } => {
                    world.chunk_meshes.remove(&position);
                }
            }
        }
    }

    pub fn exit(&self) {
        self.abort_handle.abort();
    }
}
