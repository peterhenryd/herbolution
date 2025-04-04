#![feature(iter_array_chunks)]
#![feature(random)]

use crate::client::input::ClientInputReceiver;
use crate::client::output::client_output_channel;
use crate::client::Client;
use crate::handle::{GameHandle, Request, Response};
use crate::world::entity::body::{Boundary, EntityBody};
use crate::world::entity::logic::player::PlayerLogic;
use crate::world::entity::{ChunkLoader, Entity, EntityAbilities, EntityData, EntityLogicVariant};
use crate::world::map::WorldMap;
use crossbeam::channel::{bounded, Receiver, Sender};
use lib::geometry::cuboid::Cuboid;
use lib::time::{DeltaTime, TickTime};
use math::vector::Vec3;
use tokio::spawn;

pub mod client;
pub mod world;
pub mod handle;

pub const TICKS_PER_SECOND: u64 = 60;
pub const DELTA_TIME: f32 = 1.0 / TICKS_PER_SECOND as f32;

pub struct Game {
    world_map: WorldMap,
    delta_time: DeltaTime,
    tick_time: TickTime,
    clients: Vec<Client>,
    request_rx: Receiver<Request>,
    response_tx: Sender<Response>,
}

impl Game {
    pub fn spawn() -> GameHandle {
        let (request_tx, request_rx) = bounded(128);
        let (response_tx, response_rx) = bounded(128);
        let (shutdown_tx, shutdown_rx) = bounded(1);

        spawn(async move {
            let mut game = Game::new(request_rx, response_tx);

            loop {
                if let Ok(()) = shutdown_rx.try_recv() {
                    game.shutdown();
                    break;
                }

                game.update();
            }
        });

        GameHandle {
            request_tx,
            response_rx,
            shutdown_tx,
        }
    }

    fn shutdown(&self) {

    }

    fn add_client(&mut self, input_receiver: ClientInputReceiver) {
        let (output_sender, output_receiver) = client_output_channel();
        let entity_id = self.world_map.primary().entity_set.add(Entity {
            data: EntityData {
                body: EntityBody::new(
                    Vec3::new(0., 128., 0.0),
                    Boundary {
                        cuboid: Cuboid::from_half(Vec3::new(0.4, 0.9, 0.4)),
                        eye_offset: Vec3::new(0., 0.9, 0.),
                    }
                ),
                abilities: EntityAbilities { is_affected_by_gravity: false },
                chunk_loader: Some(ChunkLoader::new()),
            },
            logic: PlayerLogic::new(output_sender.clone()).into(),
        });

        self.clients.push(Client {
            entity_id,
            input_receiver,
            output_sender,
        });

        self.send_response(Response::ClientAdded(output_receiver));
    }

    fn new(request_receiver: Receiver<Request>, response_sender: Sender<Response>) -> Self {
        Self {
            world_map: WorldMap::new(response_sender.clone()),
            delta_time: DeltaTime::new(),
            tick_time: TickTime::new(TICKS_PER_SECOND),
            clients: vec![],
            request_rx: request_receiver,
            response_tx: response_sender,
        }
    }

    fn update(&mut self) {
        while let Ok(request) = self.request_rx.try_recv() {
            self.handle_request(request);
        }

        let dt = self.delta_time.next();
        self.tick_time.increment(dt);

        while self.tick_time.is_ready() {
            self.tick_time.reduce();
            self.tick();
        }
    }

    fn handle_request(&mut self, request: Request) {
        match request {
            Request::AddClient(receiver) => self.add_client(receiver)
        }
    }

    fn send_response(&self, response: Response) {
        self.response_tx.try_send(response).unwrap();
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
        }

        self.world_map.tick();
    }
}