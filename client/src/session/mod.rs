use std::time::Duration;
use crate::session::debugger::Debugger;
use engine::{Engine, EngineFrame};
use lib::counter::IntervalCounter;
use lib::time::DeltaTime;
use math::size::Size2;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;
use game::channel::LoadChunkMsg;
use game::Game;
use game::handle::GameHandle;
use crate::session::chunk::SessionChunk;
use crate::session::player::SessionPlayer;
use crate::session::world::SessionWorld;

pub mod chunk;
pub mod player;
pub mod debugger;
pub mod world;

pub struct GameSession {
    handle: GameHandle,
    fps: IntervalCounter,
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
            fps: IntervalCounter::new(Duration::SECOND),
            player,
            is_focused: false,
            world: SessionWorld::new(),
        }
    }

    pub fn update(&mut self, frame: &EngineFrame, engine: &mut Engine) {
        self.world.chunk_map.update(&engine.gpu.handle);
        self.check_cursor_lock(frame, engine);

        while let Some(client) = self.handle.recv_client() {
            self.player.output_receiver = Some(client);
        }

        while let Some(LoadChunkMsg { position, receiver }) = self.handle.recv_load_chunk() {
            self.world.chunk_map.insert(SessionChunk::create(position, &engine.gpu.handle, receiver));
        }

        while let Some(position) = self.handle.recv_unload_chunk() {
            self.world.chunk_map.remove(position);
        }

        let dt = self.delta_time.next();

        self.player.update(&engine.gpu.handle, &mut engine.renderer_3d, (&frame.input, &engine.input), self.is_focused);
        self.fps.update(dt);
        self.debugger.update(frame, &mut engine.renderer_2d, &self.fps, engine.renderer_3d.camera.position);
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.debugger.set_size(size);
    }

    pub fn exit(&mut self) {
        self.handle.exit();
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