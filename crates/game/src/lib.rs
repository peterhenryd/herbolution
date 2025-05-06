#![feature(iter_array_chunks)]
#![feature(random)]
#![feature(array_chunks)]

use crate::channel::{ClientChannel, ServerChannel};
use crate::chunk::channel::{ClientChunkChannel, ServerChunkChannel};
use crate::client::{client_output_channel, Client, ClientInputReceiver};
use crate::entity::body::{Boundary, EntityBody};
use crate::entity::logic::player::PlayerLogic;
use crate::entity::{ChunkLoader, Entity, EntityAbilities, EntityData, EntityLogicVariant};
use crate::world::WorldMap;
use lib::geometry::cuboid::Cuboid;
use lib::time::{DeltaTime, TickTime};
use math::vector::Vec3;
use tokio::spawn;
use tokio::sync::Mutex;
use lib::fs::Save;
use math::num::traits::ConstZero;

pub mod client;
pub mod world;
pub mod channel;
pub mod chunk;
pub mod entity;

pub const TICKS_PER_SECOND: u64 = 60;
pub const DELTA_TIME: f32 = 1.0 / TICKS_PER_SECOND as f32;

pub struct Game {
    world_map: WorldMap,
    delta_time: DeltaTime,
    tick_time: TickTime,
    clients: Vec<Client>,
    channel: ServerChannel,
    //save: Save,
}

pub struct Options {
    pub save: Save,
}

impl Game {
    pub fn spawn(options: Options) -> (ClientChannel, ClientChunkChannel) {
        let (client_channel, server_channel) = channel::create();
        let (client_chunk_channel, server_chunk_channel) = chunk::channel::create();

        spawn(async move {
            let mut game = Game::new(options, server_channel, server_chunk_channel);

            loop {
                if game.channel.recv_exit() {
                    game.shutdown();
                    break;
                }

                game.update().await;
            }
        });

        (client_channel, client_chunk_channel)
    }

    fn shutdown(&self) {

    }

    fn add_client(&mut self, input_receiver: ClientInputReceiver) {
        let (output_sender, output_receiver) = client_output_channel();
        let entity_id = self.world_map.primary().entity_set.add(Entity {
            data: Mutex::new(EntityData {
                body: EntityBody::new(
                    Vec3::new(0., 96., 0.0),
                    Boundary {
                        cuboid: Cuboid::new(Vec3::ZERO, Vec3::new(0.9, 1.9, 0.9)),
                        eye_offset: Vec3::new(0., 1.0, 0.),
                    }
                ),
                abilities: EntityAbilities { is_affected_by_gravity: true },
                chunk_loader: Some(ChunkLoader::new()),
            }),
            logic: PlayerLogic::new(output_sender.clone()).into(),
        });

        self.clients.push(Client {
            entity_id,
            input_receiver,
            output_sender,
        });

        self.channel.send_client_output(output_receiver);
    }

    fn new(_: Options, channel: ServerChannel, chunk_channel: ServerChunkChannel) -> Self {
        Self {
            world_map: WorldMap::new(chunk_channel),
            delta_time: DeltaTime::new(),
            tick_time: TickTime::new(TICKS_PER_SECOND),
            clients: vec![],
            channel,
            //save: options.save,
        }
    }

    async fn update(&mut self) {
        while let Some(input_receiver) = self.channel.recv_client_input() {
            self.add_client(input_receiver);
        }

        let dt = self.delta_time.next();
        self.tick_time.increment(dt);

        while self.tick_time.is_ready() {
            self.tick_time.reduce();
            self.tick().await;
        }
    }

    async fn tick(&mut self) {
        let world = self.world_map.primary();
        for client in &mut self.clients {
            let Some(entity) = world.entity_set.get_mut(client.entity_id) else { continue };

            let mut data = entity.data.lock().await;
            client.input_receiver.dequeue_onto_body(&mut data.body);

            client.output_sender.send_camera_position(data.body.eye_pos());
            client.output_sender.send_camera_rotation(data.body.rotation);

            if let EntityLogicVariant::Player(logic) = &entity.logic {
                client.input_receiver.dequeue_onto_controller(&mut *logic.controller.lock().await);
            }
        }

        self.world_map.tick().await;
    }
}