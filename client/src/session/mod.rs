use crate::session::debugger::Debugger;
use engine::gpu::handle::Handle;
use engine::{Engine, EngineFrame};
use game::{Game, GameHandle, Response};
use lib::fps::Fps;
use lib::time::DeltaTime;
use math::size::Size2;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;
use crate::session::chunk::SessionChunk;
use crate::session::player::SessionPlayer;
use crate::session::world::SessionWorld;

pub mod chunk;
pub mod player;
pub mod debugger;
pub mod world;

pub struct GameSession {
    handle: GameHandle,
    fps: Fps,
    debugger: Debugger,
    delta_time: DeltaTime,
    player: SessionPlayer,
    is_focused: bool,
    pub(crate) world: SessionWorld,
}

impl GameSession {
    pub fn create(size: Size2<u32>) -> Self {
        let handle = Game::spawn();
        let player = SessionPlayer::new(&handle);

        Self {
            handle,
            debugger: Debugger::create(size),
            delta_time: DeltaTime::new(),
            fps: Fps::new(),
            player,
            is_focused: false,
            world: SessionWorld::new(),
        }
    }

    pub fn update(&mut self, frame: &EngineFrame, engine: &mut Engine) {
        self.world.chunk_map.update(&engine.gpu.handle);
        self.check_cursor_lock(frame, engine);

        while let Some(response) = self.handle.receive_response() {
            self.process_response(response, &engine.gpu.handle);
        }

        let dt = self.delta_time.next();

        self.player.update(&engine.gpu.handle, &mut engine.renderer_3d, &frame.input, &engine.input, self.is_focused);
        self.fps.update(dt);
        self.debugger.update(frame, &mut engine.renderer_2d, &self.fps, self.player.transform().position);
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.debugger.set_size(size);
    }

    pub fn exit(&self) {
        self.handle.exit();
    }

    fn process_response(&mut self, response: Response, handle: &Handle) {
        match response {
            Response::PlayerCreated(player_handle) => {
                self.player.handle = Some(player_handle);
            }
            Response::LoadChunk { position, receiver } => {
                self.world.chunk_map.insert(SessionChunk::create(position, handle, receiver));
            }
        }
    }

    fn check_cursor_lock(&mut self, frame: &EngineFrame, engine: &Engine) {
        if engine.input.is_key_active(KeyCode::Escape) {
            engine.window.set_cursor_grab(CursorGrabMode::None).unwrap();
            engine.window.set_cursor_visible(true);
            self.is_focused = false;
        } else if frame.input.click_events.iter().any(|x| x.button == MouseButton::Left) {
            engine.window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
            engine.window.set_cursor_visible(false);
            self.is_focused = true;
        }
    }
}