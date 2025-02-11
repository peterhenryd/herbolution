pub mod camera;
pub mod chunk;
pub mod debugger;
pub mod entity;
pub mod lighting;
pub mod position;
pub mod renderer;
pub mod transform;
pub mod observer;

use crate::engine::geometry::cuboid::Cuboid;
use crate::engine::Engine;
use crate::game::fps::Fps;
use crate::listener::{InputEvent, Listener};
use crate::ui::Ui;
use crate::world::camera::frustum::Frustum;
use crate::world::chunk::map::ChunkMap;
use crate::world::debugger::Debugger;
use crate::world::entity::data::player::PlayerEntityData;
use crate::world::entity::data::EntityData;
use crate::world::entity::physics::{EntityGravity, EntityPhysics};
use crate::world::entity::set::EntitySet;
use crate::world::entity::{Entity, EntityAbilities};
use crate::world::lighting::light::PointLight;
use crate::world::observer::{Abilities, Observer};
use crate::world::renderer::Renderer;
use crate::world::transform::{Rotation, Transform};
use math::color::Color3;
use math::vector::vec3;
use std::time::Duration;
use wgpu::RenderPass;
use winit::dpi::PhysicalSize;

pub struct World {
    pub(crate) renderer: Renderer,
    chunk_map: ChunkMap,
    entity_set: EntitySet,
    debugger: Debugger,
    observer: Observer,
}

impl Listener for World {
    fn on_window_resized(&mut self, size: PhysicalSize<u32>) {
        self.renderer.on_window_resized(size);
        self.debugger.on_window_resized(size);
    }

    fn on_input(&mut self, event: &InputEvent) {
        self.debugger.on_input(event);

        if let Some(Entity { data: box EntityData::Player(data), .. }) = self.entity_set.get_mut(self.observer.entity_id) {
            data.controller.on_input(event);
        }
    }
}

impl World {
    pub fn create(engine: &Engine) -> Self {
        let mut renderer = Renderer::create(engine.gpu.clone(), &engine.surface);
        renderer.lighting.point_light_set.edit(|set| {
            *set = vec![PointLight {
                color: Color3::WHITE,
                intensity: 0.5,
                position: vec3::new(0., 128., 0.),
                range: 16.0,
            }]
        });
        let mut chunk_map = ChunkMap::new(engine.gpu.clone(), 48323);

        for x in -4..4 {
            for y in 1..3 {
                for z in -4..4 {
                    chunk_map.load_chunk(vec3::new(x, y, z));
                }
            }
        }

        let mut entity_set = EntitySet::new();
        let entity_id = entity_set.add(Entity {
            physics: EntityPhysics {
                transform: Transform {
                    position: vec3::new(0., 128., 0.),
                    rotation: Rotation::default(),
                },
                bounding_box: Cuboid::new(vec3::new(-0.5, -1.0, -0.5), vec3::new(0.5, 1.0, 0.5)),
                eye_offset: vec3::ZERO,
                gravity: EntityGravity {
                    fall_acceleration: -9.81,
                    fall_speed: 0.0,
                    max_fall_speed: 16.0,
                },
            },
            data: Box::new(PlayerEntityData::default().into()),
            abilities: EntityAbilities {
                is_affected_by_gravity: true,
            },
        });
        let observer = Observer { entity_id, abilities: Abilities { accepts_input: true } };
        let debugger = Debugger::create();

        Self {
            chunk_map,
            renderer,
            entity_set,
            debugger,
            observer,
        }
    }

    #[inline]
    pub fn chunks(&self) -> &ChunkMap {
        &self.chunk_map
    }

    #[inline]
    pub fn chunks_mut(&mut self) -> &mut ChunkMap {
        &mut self.chunk_map
    }

    fn reset_input_trackers(&mut self) {}

    pub fn update(&mut self, dt: Duration, engine: &Engine, ui: &mut Ui, fps: &Fps) {
        if !engine.is_focused {
            self.reset_input_trackers();
        }

        self.entity_set.update(dt, &mut self.chunk_map);
        self.observer.update(&mut self.entity_set, &mut self.renderer.camera);
        self.debugger.update(ui, fps, self.entity_set.get(self.observer.entity_id).unwrap().physics.transform);

        for chunk in self.chunk_map.iter_mut() {
            chunk.update(&self.renderer.gpu);
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass<'_>) {
        self.renderer.pipeline.enable(render_pass);
        self.renderer.quad_mesh.load(render_pass);

        let frustum = Frustum::new(&self.renderer.camera);
        self.chunk_map
            .iter()
            .filter(|chunk| frustum.contains_chunk(chunk.position))
            .for_each(|chunk| chunk.render(render_pass));
    }
}