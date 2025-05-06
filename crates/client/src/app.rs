use crate::engine::Engine;
use crate::session::GameSession;
use crate::Options;
use math::color::{Color, Rgba};
use math::size::Size2;
use math::vector::Vec2;
use std::path::Path;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{DeviceEvent, DeviceId, KeyEvent, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;
use winit::window::{WindowAttributes, WindowId};

pub struct App {
    engine: Engine,
    //fs: Fs,
    session: Option<GameSession>,
}

pub struct Handler {
    options: Options,
    app: Option<App>,
}

impl App {
    pub fn new(event_loop: &ActiveEventLoop, _: impl AsRef<Path>) -> Self {
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
        //let fs = Fs::new(data_dir.as_ref().to_path_buf());

        Self { engine, /*fs,*/ session: None }
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.engine.set_size(size);

        if let Some(session) = &mut self.session {
            session.set_size(size);
        }
    }

    pub fn update(&mut self) {
        let mut frame = self.engine.next_frame().expect("Failed to create frame");

        if self.session.is_none() {
            self.session = Some(GameSession::create(self.engine.surface.size()));
        }

        self.session.as_mut().unwrap().update(&frame, &mut self.engine);
        self.engine.update();

        let mut render_pass = frame.gpu.start_pass(Some(Rgba::from_rgb(117, 255, 250).into()));

        if let Some(session) = &self.session {
            let observer = self.engine.state3d.camera.position;
            self.engine.render(&mut render_pass, session.world.chunk_map.meshes(observer));
        }
        self.engine.state2d.render(&mut render_pass);

        drop(render_pass);
        frame.gpu.finish(&self.engine.gpu);
        self.engine.window.request_redraw();

        self.engine.state2d.cleanup();
    }

    pub fn exit(&mut self) {
        if let Some(session) = &mut self.session {
            session.exit();
        }
    }
}

impl Handler {
    pub fn new(options: Options) -> Self {
        Self { options, app: None }
    }
}

impl ApplicationHandler for Handler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.app = Some(App::new(event_loop, &self.options.data_dir));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let Some(app) = &mut self.app else { return };

        match event {
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                app.set_size(Size2::new(width, height));
            }
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                app.exit();
                event_loop.exit();
            }
            WindowEvent::CursorMoved { position: PhysicalPosition { x, y }, .. } => {
                app.engine.input.set_mouse_pos(Vec2::new(x, y));
            }
            WindowEvent::MouseInput { button, state, .. } => {
                app.engine.input.set_mouse_button_activity(button, state.is_pressed());
            }
            WindowEvent::MouseWheel { delta: MouseScrollDelta::PixelDelta(delta), .. } => {
                app.engine.input.add_mouse_scroll(delta.y as f32);
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                app.engine.input.set_modifiers(modifiers);
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state,
                    ..
                },
                ..
            } => {
                app.engine.input.set_key_activity(code, state.is_pressed());
            }
            WindowEvent::RedrawRequested => {
                app.update();
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        let Some(app) = &mut self.app else { return };

        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                app.engine.input.add_mouse_movement(Vec2::new(dx, dy));
            }
            _ => {}
        }
    }
}
