pub mod camera;
pub mod chunk;
pub mod debugger;
pub mod entity;
pub mod lighting;
pub mod player;
pub mod position;
pub mod renderer;
pub mod transform;

use crate::engine::Engine;
use crate::game::fps::Fps;
use crate::listener::{InputEvent, Listener};
use crate::ui::Ui;
use crate::world::camera::frustum::Frustum;
use crate::world::chunk::map::ChunkMap;
use crate::world::debugger::Debugger;
use crate::world::entity::set::EntitySet;
use crate::world::player::Player;
use crate::world::renderer::Renderer;
use math::vector::vec3;
use std::time::Duration;
use wgpu::RenderPass;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton};
use winit::keyboard::KeyCode;
use math::color::Color3;
use crate::world::lighting::light::PointLight;

pub struct World {
    pub(crate) renderer: Renderer,
    chunk_map: ChunkMap,
    entity_set: EntitySet,
    player: Player,
    debugger: Debugger,
}

impl Listener for World {
    fn on_window_resized(&mut self, size: PhysicalSize<u32>) {
        self.renderer.on_window_resized(size);
        self.debugger.on_window_resized(size);
    }

    fn on_input(&mut self, event: &InputEvent) {
        self.debugger.on_input(event);

        match event {
            InputEvent::Key { code, state } => {
                let f = if state.is_pressed() { 1.0 } else { 0.0 };

                match code {
                    KeyCode::KeyW => self.player.motion.forward = f,
                    KeyCode::KeyS => self.player.motion.backward = f,
                    KeyCode::KeyA => self.player.motion.leftward = f,
                    KeyCode::KeyD => self.player.motion.rightward = f,
                    KeyCode::Space => self.player.motion.upward = f,
                    KeyCode::ShiftLeft => self.player.motion.downward = f,
                    _ => {}
                }
            }
            &InputEvent::MouseMoving { dx, dy } => {
                if self.player.motion.rotation.is_none() {
                    return self.player.motion.rotation = Some((dx, dy));
                }

                let (x, y) = self.player.motion.rotation.as_mut().unwrap();
                *x += dx;
                *y += dy;
            }
            InputEvent::MouseClick {
                button: MouseButton::Left,
                state: ElementState::Pressed,
            } => self.player.break_cube += 1,
            _ => {}
        }
    }
}

impl World {
    pub fn create(engine: &Engine) -> Self {
        let mut renderer = Renderer::create(engine.gpu.clone(), &engine.surface);
        renderer.lighting.point_light_set.edit(|set| *set = vec![PointLight {
            color: Color3::WHITE,
            intensity: 0.5,
            position: vec3::new(0., 128., 0.),
            range: 16.0,
        }]);
        let mut chunk_map = ChunkMap::new(engine.gpu.clone(), 48323);

        for x in -4..4 {
            for y in 1..3 {
                for z in -4..4 {
                    chunk_map.load_chunk(vec3::new(x, y, z));
                }
            }
        }

        let player = Player::new(vec3::new(0., 128., 0.));
        let entity_set = EntitySet::new();
        let debugger = Debugger::create();

        Self {
            chunk_map,
            renderer,
            player,
            entity_set,
            debugger,
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

    fn reset_input_trackers(&mut self) {
        self.player.motion.reset();
    }

    pub fn update(&mut self, dt: Duration, engine: &Engine, ui: &mut Ui, fps: &Fps) {
        if !engine.is_focused {
            self.reset_input_trackers();
        }

        self.player.update(dt, &mut self.chunk_map, &mut self.renderer.camera);
        self.debugger.update(ui, fps, &self.player);

        for chunk in self.chunk_map.iter_mut() {
            chunk.update(&self.renderer.gpu);
        }

        self.entity_set.tick_all();
    }

    pub fn render(&self, render_pass: &mut RenderPass<'_>) {
        self.renderer.pipeline.enable(render_pass);
        self.renderer.quad_mesh.load(render_pass);

        let frustum = Frustum::new(&self.renderer.camera);
        self.chunk_map.iter()
            .filter(|chunk| frustum.contains_chunk(chunk.position))
            .for_each(|chunk| chunk.render(render_pass));
    }
}