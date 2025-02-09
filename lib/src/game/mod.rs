pub mod time;
pub mod fps;

use crate::engine::Engine;
use crate::game::fps::Fps;
use crate::game::time::DeltaTime;
use crate::listener::{InputEvent, Listener};
use crate::ui::Ui;
use crate::world::World;
use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp, TextureView};
use winit::dpi::PhysicalSize;

pub struct Game {
    pub world: World,
    pub ui: Ui,
    pub time: DeltaTime,
    fps: Fps,
    is_focused: bool,
}

impl Game {
    pub fn create(engine: &Engine) -> Self {
        Self {
            world: World::create(engine),
            ui: Ui::create(engine),
            time: DeltaTime::default(),
            fps: Fps::new(),
            is_focused: false,
        }
    }

    pub fn update(&mut self, engine: &Engine) {
        self.is_focused = engine.is_focused;

        self.fps.update();

        let dt = self.time.next_delta();

        self.world.update(dt, engine, &mut self.ui, &self.fps);
        self.ui.update(dt);
    }

    fn render_world_pass(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        let mut render_pass = encoder
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("herbolution_render_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: self.world.renderer.depth_texture.as_ref(),
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
        self.world.render(&mut render_pass);
    }

    fn render_ui_pass(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        let mut render_pass = encoder
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("herbolution_ui_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load,
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
        self.ui.render(&mut render_pass);
    }

    pub fn render(&mut self, encoder: &mut CommandEncoder, view: &TextureView) {
        self.render_world_pass(encoder, view);
        self.render_ui_pass(encoder, view);
    }
}

impl Listener for Game {
    fn on_window_resized(&mut self, size: PhysicalSize<u32>) {
        self.world.on_window_resized(size);
    }

    fn on_input(&mut self, event: &InputEvent) {
        if !self.is_focused {
            return;
        }

        self.world.on_input(event);
    }
}