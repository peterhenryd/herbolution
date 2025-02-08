use crate::engine::Engine;
use crate::game::Game;
use crate::listener::{InputEvent, Listener};
use lazy_winit::ApplicationInit;
use std::sync::Arc;
use wgpu::{Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, DeviceId, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::{WindowAttributes, WindowId};
use crate::Options;

pub struct Handler {
    engine: Engine,
    game: Game,
}

impl Listener for Handler {
    fn on_window_resized(&mut self, size: PhysicalSize<u32>) {
        self.engine.on_window_resized(size);
        self.game.on_window_resized(size);
    }

    fn on_input(&mut self, event: &InputEvent) {
        self.engine.on_input(event);
        self.game.on_input(event);
    }
}

impl Handler {
    fn update(&mut self) {
        self.game.update();
    }

    fn render(&self) {
        let (surface_texture, surface_view) = self.engine.surface.prepare();
        let mut encoder = self.engine.gpu.device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("herbolution_renderer_command_encoder"),
            });
        let mut render_pass = encoder
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("herbolution_render_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: self.game.world.renderer.depth_texture.as_ref(),
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });

        self.game.render(&mut render_pass);

        drop(render_pass);

        self.engine.gpu.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}

impl ApplicationInit for Handler {
    type Args = Options;

    fn new(event_loop: &ActiveEventLoop, Options {}: Self::Args) -> Self {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        const RESOLUTION: (u32, u32) = (1920, 1080);

        let attributes = WindowAttributes::default()
            .with_title(format!("Herbolution {}", VERSION))
            .with_inner_size(PhysicalSize::<u32>::from(RESOLUTION));
        let window = event_loop
            .create_window(attributes)
            .expect("Failed to create window");
        let window = Arc::new(window);

        let engine = Engine::create(window.clone());
        let game = Game::create(&engine);

        Self { engine, game }
    }
}

impl ApplicationHandler for Handler {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(size) =>
                self.on_window_resized(size),
            WindowEvent::CloseRequested =>
                event_loop.exit(),
            WindowEvent::CursorMoved { position, .. } =>
                self.on_input(&InputEvent::MouseMoved(position)),
            WindowEvent::MouseInput { button, state, .. } =>
                self.on_input(&InputEvent::MouseClick { button, state }),
            WindowEvent::KeyboardInput { event: KeyEvent { physical_key: PhysicalKey::Code(code), state, .. }, .. } =>
                self.on_input(&InputEvent::Key { code, state }),
            WindowEvent::CursorEntered { ..} =>
                self.on_input(&InputEvent::MouseEntered),
            WindowEvent::CursorLeft { .. } =>
                self.on_input(&InputEvent::MouseLeft),
            WindowEvent::RedrawRequested => {
                self.update();
                self.render();
                self.engine.window.request_redraw();
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                self.on_input(&InputEvent::MouseMoving { dx, dy });
            }
            _ => {}
        }
    }
}