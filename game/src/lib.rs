#![feature(iter_array_chunks)]
#![feature(random)]
#![feature(duration_constants)]

use crate::channel::{channel, Clientbound, ClientboundChunks};
use crate::client::input::ClientInputReceiver;
use crate::client::output::client_output_channel;
use crate::client::Client;
use crate::handle::GameHandle;
use crate::world::entity::body::{Boundary, EntityBody};
use crate::world::entity::logic::player::PlayerLogic;
use crate::world::entity::{ChunkLoader, Entity, EntityAbilities, EntityData, EntityLogicVariant};
use crate::world::map::WorldMap;
use lib::geometry::cuboid::Cuboid;
use lib::time::{DeltaTime, TickTime};
use math::vector::Vec3;
use tokio::spawn;

pub mod client;
pub mod world;
pub mod handle;
pub mod channel;

pub const TICKS_PER_SECOND: u64 = 60;
pub const DELTA_TIME: f32 = 1.0 / TICKS_PER_SECOND as f32;

pub struct Game {
    world_map: WorldMap,
    delta_time: DeltaTime,
    tick_time: TickTime,
    clients: Vec<Client>,
    clientbound: Clientbound,
}

impl Game {
    pub fn spawn() -> GameHandle {
        let (clientbound, serverbound) = channel();
        let (clientbound_chunks, serverbound_chunks) = channel::chunks();

        spawn(async move {
            let mut game = Game::new(clientbound, clientbound_chunks);

            loop {
                if game.clientbound.shutdown_requested() {
                    game.shutdown();
                    break;
                }

                game.update();
            }
        });

        GameHandle {
            channel: serverbound,
            chunks: serverbound_chunks,
        }
    }

    fn shutdown(&self) {

    }

    fn add_client(&mut self, input_receiver: ClientInputReceiver) {
        let (output_sender, output_receiver) = client_output_channel();
        let entity_id = self.world_map.primary().entity_set.add(Entity {
            data: EntityData {
                body: EntityBody::new(
                    Vec3::new(0., 72., 0.0),
                    Boundary {
                        cuboid: Cuboid::from_half(Vec3::new(0.4, 0.9, 0.4)),
                        eye_offset: Vec3::new(0., 0.9, 0.),
                    }
                ),
                abilities: EntityAbilities { is_affected_by_gravity: false },
                chunk_loader: Some(ChunkLoader::new(6)),
            },
            logic: PlayerLogic::new(output_sender.clone()).into(),
        });

        self.clients.push(Client {
            entity_id,
            input_receiver,
            output_sender,
        });

        self.clientbound.send_client(output_receiver);
    }

    fn new(clientbound: Clientbound, chunks: ClientboundChunks) -> Self {
        Self {
            world_map: WorldMap::new(chunks),
            delta_time: DeltaTime::new(),
            tick_time: TickTime::new(TICKS_PER_SECOND),
            clients: vec![],
            clientbound,
        }
    }

    fn update(&mut self) {
        while let Some(client) = self.clientbound.recv_client() {
            self.add_client(client);
        }

        let dt = self.delta_time.next();
        self.tick_time.increment(dt);

        while self.tick_time.is_ready() {
            self.tick_time.reduce();
            self.tick();
        }
    }

    fn tick(&mut self) {
        let world = self.world_map.primary();
        for client in &mut self.clients {
            let Some(entity) = world.entities_mut().get_mut(client.entity_id) else { continue };

            client.input_receiver.dequeue_onto_body(&mut entity.data.body);

            client.output_sender.set_camera_position(entity.data.body.eye_position());
            client.output_sender.set_camera_rotation(entity.data.body.rotation);

            if let EntityLogicVariant::Player(logic) = &mut entity.logic {
                client.input_receiver.dequeue_onto_controller(&mut logic.controller);
            }

            if let Some(command) = client.input_receiver.dequeue_render_distance_command() {
                let Some(chunk_loader) = &mut entity.data.chunk_loader else { continue };
                if command {
                    chunk_loader.radius += 1;
                } else if chunk_loader.radius > 1 {
                    chunk_loader.radius -= 1;
                }
            }
        }

        self.world_map.tick();
    }
}