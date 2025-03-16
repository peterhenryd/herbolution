#![feature(iter_array_chunks)]

use std::f32::consts::FRAC_PI_2;
use kanal::{bounded, Receiver, Sender};
use crate::world::entity::body::{EntityBody, EntityGravity};
use crate::world::entity::logic::player::{PlayerController, PlayerLogic};
use crate::world::entity::set::EntityId;
use crate::world::entity::{Entity, EntityAbilities, EntityData};
use crate::world::map::WorldMap;
use lib::geometry::cuboid::Cuboid;
use lib::time::{DeltaTime, TickTime};
use math::angle::Rad;
use math::rotation::Euler;
use math::transform::Transform;
use math::vector::{vec3f, vec3i, Vec3};
use tokio::spawn;
use tokio::task::AbortHandle;
use crate::world::chunk::ChunkUpdate;

pub mod world;

pub const TICKS_PER_SECOND: u64 = 60;

pub struct Game {
    world_map: WorldMap,
    delta_time: DeltaTime,
    tick_time: TickTime,
    request_receiver: Receiver<Request>,
    response_sender: Sender<Response>,
    players: Vec<Player>,
}

impl Game {
    pub fn spawn() -> GameHandle {
        let (request_sender, request_receiver) = bounded(64);
        let (response_sender, response_receiver) = bounded(64);

        let abort_handle = spawn(async move {
            let mut game = Game::new(request_receiver, response_sender);

            loop {
                game.update();
            }
        }).abort_handle();

        GameHandle {
            abort_handle,
            request_sender,
            response_receiver,
        }
    }

    fn create_player(&mut self, action_receiver: Receiver<ActionImpulse>) {
        let world = self.world_map.primary();

        let transform = Transform::new(Vec3::new(0., 128., 0.0));

        // Channels are sized 1 to ensure that only the most recent message is queued.
        let (position_tx, position_rx) = bounded(1);
        let (rotation_tx, rotation_rx) = bounded(1);
        let (target_tx, target_rx) = bounded(1);

        let id = world.entity_set.add(Entity {
            data: EntityData {
                body: EntityBody {
                    transform: transform.clone(),
                    bounding_box: Cuboid::from_half(Vec3::new(0.4, 0.9, 0.4)),
                    eye_offset: Vec3::new(0., 0.9, 0.),
                    gravity: EntityGravity {
                        fall_acceleration: -9.81,
                        fall_speed: 0.0,
                        max_fall_speed: 16.0,
                    },
                },
                abilities: EntityAbilities { is_affected_by_gravity: false },
            },
            logic: Box::new(PlayerLogic {
                controller: PlayerController::new(target_tx.clone()),
            }),
        });

        self.players.push(Player {
            id,
            position_tx,
            rotation_tx,
            target_tx,
            action_receiver,
            prev_position: Default::default(),
            prev_rotation: Default::default(),
        });
        self.send_response(Response::PlayerCreated(PlayerHandle { transform, target: None, position_rx, rotation_rx, target_rx }));
    }

    fn new(request_receiver: Receiver<Request>, response_sender: Sender<Response>) -> Self {
        Self {
            world_map: WorldMap::new(response_sender.clone()),
            delta_time: DeltaTime::new(),
            tick_time: TickTime::new(TICKS_PER_SECOND),
            request_receiver,
            response_sender,
            players: vec![],
        }
    }

    fn update(&mut self) {
        while let Ok(Some(request)) = self.request_receiver.try_recv() {
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
            Request::CreatePlayer { receiver } => self.create_player(receiver)
        }
    }

    fn send_response(&self, response: Response) {
        self.response_sender.try_send(response).unwrap();
    }

    fn tick(&mut self) {
        self.world_map.tick();

        for player in &mut self.players {
            let Some(entity) = self.world_map.primary().entities_mut().get_mut(player.id) else { continue };

            while let Ok(Some(action_impulse)) = player.action_receiver.try_recv() {
                entity.logic.on_action_impulse(action_impulse);
            }

            let transform = &mut entity.data.body.transform;
            transform.rotation.pitch = Rad(transform.rotation.pitch.0.clamp(-FRAC_PI_2 + 0.001, FRAC_PI_2 - 0.001));

            // Errors are ignored as the intent is to replace the previously queued message,
            // which will almost always return an error.
            if transform.position != player.prev_position {
                let _ = player.position_tx.try_send(transform.position);
                player.prev_position = transform.position;
            }

            if transform.rotation != player.prev_rotation {
                let _ = player.rotation_tx.try_send(transform.rotation);
                player.prev_rotation = transform.rotation;
            }
        }
    }
}

pub struct Player {
    pub id: EntityId,
    pub position_tx: Sender<vec3f>,
    pub rotation_tx: Sender<Euler<Rad<f32>>>,
    pub target_tx: Sender<Option<vec3i>>,
    pub action_receiver: Receiver<ActionImpulse>,
    pub prev_position: vec3f,
    pub prev_rotation: Euler<Rad<f32>>,
}

pub struct GameHandle {
    abort_handle: AbortHandle,
    request_sender: Sender<Request>,
    response_receiver: Receiver<Response>,
}

impl GameHandle {
    pub fn exit(&self) {
        self.abort_handle.abort();
    }

    fn send_request(&self, request: Request) {
        if let Err(e) = self.request_sender.try_send(request) {
            eprintln!("Failed to send game request: {}", e);
        }
    }

    pub fn request_player(&self, receiver: Receiver<ActionImpulse>) {
        self.send_request(Request::CreatePlayer { receiver });
    }

    pub fn receive_response(&mut self) -> Option<Response> {
        self.response_receiver.try_recv().ok().flatten()
    }
}

pub enum Request {
    CreatePlayer { receiver: Receiver<ActionImpulse> },
}

pub enum ActionImpulse {
    Jump,
    Move { forces: vec3f },
    Rotate { delta_rotation: Euler<Rad<f32>> },
    Interact { hand: Hand, state: ActionState },
}

pub enum Hand {
    Left,
    Right,
}

pub enum ActionState {
    Start,
    Stop,
    Once,
}

pub enum Response {
    PlayerCreated(PlayerHandle),
    LoadChunk { position: vec3i, receiver: Receiver<ChunkUpdate> },
}

pub struct PlayerHandle {
    pub transform: Transform,
    pub target: Option<vec3i>,
    position_rx: Receiver<vec3f>,
    rotation_rx: Receiver<Euler<Rad<f32>>>,
    target_rx: Receiver<Option<vec3i>>,
}

impl PlayerHandle {
    pub fn update(&mut self) {
        if let Ok(Some(position)) = self.position_rx.try_recv() {
            self.transform.position = position;
        }

        if let Ok(Some(rotation)) = self.rotation_rx.try_recv() {
            self.transform.rotation = rotation;
        }

        if let Ok(Some(target)) = self.target_rx.try_recv() {
            self.target = target;
        }
    }
}