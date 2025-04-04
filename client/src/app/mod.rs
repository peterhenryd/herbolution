use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowAttributes;
use engine::Engine;
use math::color::{Color, Rgba};
use math::size::Size2;
use crate::session::GameSession;

pub mod handler;

pub struct App {
    pub(crate) engine: Engine,
    session: Option<GameSession>,
}

impl App {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        const RESOLUTION: (u32, u32) = (1920, 1080);

        let attributes = WindowAttributes::default()
            .with_title(format!("Herbolution {}", VERSION))
            .with_inner_size(PhysicalSize::<u32>::from(RESOLUTION));
        let window = event_loop
            .create_window(attributes)
            .expect("Failed to create window");

        let window = Arc::new(window);
        let engine = Engine::create(window).expect("Failed to create engine");

        Self { engine, session: None }
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.engine.set_size(size);

        if let Some(session) = &mut self.session {
            session.set_size(size);
        }
    }

    pub fn update(&mut self) {
        let mut frame = self.engine.create_frame().expect("Failed to create frame");

        if frame.input.key_events.contains(&KeyCode::Space) && self.session.is_none() {
            self.session = Some(GameSession::create(self.engine.gpu.size()));
        }

        if let Some(session) = &mut self.session {
            session.update(&frame, &mut self.engine);
        }
        self.engine.update();

        let mut render_pass = frame.gpu.start_pass(Some(Rgba::from_rgb(117, 255, 250).into()));

        if let Some(session) = &self.session {
            self.engine.renderer_3d.render(&mut render_pass, session.world.chunk_map.meshes());
        }
        self.engine.renderer_2d.render(&mut render_pass);

        drop(render_pass);
        frame.gpu.finish(&self.engine.gpu.handle);
        self.engine.window.request_redraw();

        self.engine.renderer_2d.cleanup();
    }

    pub fn exit(&self) {
        if let Some(session) = &self.session {
            session.exit();
        }
    }
}