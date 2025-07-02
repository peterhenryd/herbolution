#![feature(array_chunks)]
#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]
#![feature(random)]
#![feature(duration_constants)]
extern crate herbolution_lib as lib;
extern crate herbolution_math as math;

use std::thread;

use hashbrown::HashMap;
use herbolution_lib::util::time::DeltaTime;
use lib::save::Save;
use math::spatial::aabb::Aabb;
use math::vector::Vec3;

use crate::entity::behavior::EntityBehaviors;
use crate::entity::body::{Boundary, EntityAbilities, EntityBody};
use crate::entity::components::ChunkLoader;
use crate::entity::{Entity, EntityData};
use crate::handle::{ClientHandle, GameHandle};
use crate::player::Player;
use crate::world::World;

pub mod chunk;
pub mod entity;
pub mod generator;
pub mod handle;
pub mod player;
pub mod world;

pub struct Game {
    world_map: HashMap<String, World>,
    delta_time: DeltaTime,
    handle: ClientHandle,
    save: Save,
}

pub struct Options {
    pub save: Save,
}

impl Game {
    pub fn spawn(options: Options) -> GameHandle {
        let (client_handle, handle) = handle::create();

        thread::spawn(move || {
            let mut game = Game::new(options, client_handle);
            game.add_client();

            loop {
                if game.handle.is_exit_requested() {
                    game.exit();
                    break;
                }

                game.update();
            }
        });

        handle
    }

    fn exit(&self) {}

    fn add_client(&mut self) {
        let (client_handle, handle) = player::handle::create();
        let world = self
            .world_map
            .get_mut(&self.save.descriptor.default_world)
            .unwrap();
        world.entity_set.add(Entity {
            data: EntityData {
                body: EntityBody::new(
                    Vec3::new(0., 96., 0.0),
                    Boundary {
                        aabb: Aabb::new(Vec3::ZERO, Vec3::new(0.9, 1.9, 0.9)),
                        eye_offset: Vec3::new(0., 1.0, 0.),
                    },
                    EntityAbilities {
                        is_affected_by_gravity: false,
                        speed: 1.0,
                    },
                ),
            },
            behaviors: EntityBehaviors::new()
                .with(Player::new(client_handle))
                .with(ChunkLoader::new()),
        });

        self.handle.send_player_handle(handle);
    }

    fn new(Options { save }: Options, handle: ClientHandle) -> Self {
        let mut world_map = HashMap::new();
        let save_world = save.default_world().unwrap();
        world_map.insert(save.descriptor.default_world.clone(), World::from_save(save_world));

        Self {
            world_map,
            delta_time: DeltaTime::new(),
            handle,
            save,
        }
    }

    fn update(&mut self) {
        let dt = self.delta_time.next();

        for world in self.world_map.values_mut() {
            world.update(&self.handle, dt);
        }
    }
}
