pub mod time;

use wgpu::RenderPass;
use winit::dpi::PhysicalSize;
use crate::engine::Engine;
use crate::game::time::DeltaTime;
use crate::listener::{InputEvent, Listener};
use crate::ui::Ui;
use crate::world::World;

pub struct Game {
    pub world: World,
    pub ui: Ui,
    pub time: DeltaTime
}

impl Game {
    pub fn create(engine: &Engine) -> Self {
        Self {
            world: World::create(engine),
            ui: Ui::create(engine),
            time: DeltaTime::default(),
        }
    }

    pub fn update(&mut self) {
        let dt = self.time.next_delta();

        self.world.update(dt);
        self.ui.update(dt);
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        self.world.render(render_pass);
    }
}

impl Listener for Game {
    fn on_window_resized(&mut self, size: PhysicalSize<u32>) {
        self.world.on_window_resized(size);
    }

    fn on_input(&mut self, event: &InputEvent) {
        self.world.on_input(event);
    }
}