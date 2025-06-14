#![feature(iter_array_chunks)]
#![feature(random)]
#![feature(array_chunks)]

use std::thread;
use hashbrown::HashMap;
use lib::fs::save::Save;
use crate::channel::{ClientChannel, ServerChannel};
use crate::chunk::channel::{ClientChunkChannel, ServerChunkChannel};
use crate::client::{client_output_channel, Client, ClientInputReceiver};
use crate::entity::body::{Boundary, EntityBody};
use crate::entity::logic::player::PlayerLogic;
use crate::entity::{ChunkLoader, Entity, EntityAbilities, EntityData, EntityLogicVariant};
use lib::geo::cuboid::Cuboid;
use lib::time::{DeltaTime, TickTime};
use math::vector::Vec3;
use crate::world::World;

pub mod client;
pub mod world;
pub mod channel;
pub mod chunk;
pub mod entity;

pub const TICKS_PER_SECOND: u64 = 60;
pub const DELTA_TIME: f64 = 1.0 / TICKS_PER_SECOND as f64;

pub struct Game {
    world_map: HashMap<String, World>,
    delta_time: DeltaTime,
    tick_time: TickTime,
    clients: Vec<Client>,
    channel: ServerChannel,
    save: Save,
}

pub struct Options {
    pub save: Save,
}

impl Game {
    pub fn spawn(options: Options) -> (ClientChannel, ClientChunkChannel) {
        let (client_channel, server_channel) = channel::create();
        let (client_chunk_channel, server_chunk_channel) = chunk::channel::create();

        thread::spawn(|| {
            let mut game = Game::new(options, server_channel, server_chunk_channel);

            loop {
                if game.channel.recv_exit() {
                    game.shutdown();
                    break;
                }
                
                game.update();
            }
        });

        (client_channel, client_chunk_channel)
    }

    fn shutdown(&self) {

    }

    fn add_client(&mut self, input_receiver: ClientInputReceiver) {
        let (output_sender, output_receiver) = client_output_channel();
        let world = self.world_map.get_mut(&self.save.descriptor.default_world).unwrap();
        let entity_id = world.entity_set.add(Entity {
            data: EntityData {
                body: EntityBody::new(
                    Vec3::new(0., 96., 0.0),
                    Boundary {
                        cuboid: Cuboid::new(Vec3::ZERO, Vec3::new(0.9, 1.9, 0.9)),
                        eye_offset: Vec3::new(0., 1.0, 0.),
                    }
                ),
                abilities: EntityAbilities { is_affected_by_gravity: true },
                chunk_loader: Some(ChunkLoader::new()),
            },
            logic: PlayerLogic::new(output_sender.clone()).into(),
        });

        self.clients.push(Client {
            entity_id,
            input_receiver,
            output_sender,
        });

        self.channel.send_client_output(output_receiver);
    }

    fn new(Options { save }: Options, channel: ServerChannel, chunk_channel: ServerChunkChannel) -> Self {
        let mut world_map = HashMap::new();
        let save_world = save.default_world().unwrap();
        world_map.insert(save.descriptor.default_world.clone(), World::from_save(save_world, chunk_channel.clone()));

        Self {
            world_map,
            delta_time: DeltaTime::new(),
            tick_time: TickTime::new(TICKS_PER_SECOND),
            clients: vec![],
            channel,
            save,
        }
    }

    fn update(&mut self) {
        while let Some(input_receiver) = self.channel.recv_client_input() {
            self.add_client(input_receiver);
        }

        let dt = self.delta_time.next();
        self.tick_time.increment(dt);

        while self.tick_time.is_ready() {
            self.tick_time.reduce();
            self.tick();
        }
    }

    fn tick(&mut self) {
        let default_world = self.world_map.get_mut(&self.save.descriptor.default_world).unwrap();
        for client in &mut self.clients {
            let Some(entity) = default_world.entity_set.get_mut(client.entity_id) else { continue };

            client.input_receiver.dequeue_onto_body(&mut entity.data.body);

            client.output_sender.send_camera_position(entity.data.body.eye_pos());
            client.output_sender.send_camera_rotation(entity.data.body.rotation);

            if let EntityLogicVariant::Player(logic) = &mut entity.logic {
                client.input_receiver.dequeue_onto_controller(&mut logic.controller);
            }
        }
        
        for world in self.world_map.values_mut() {
            world.tick();
        }
    }
}
