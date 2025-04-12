use crate::session::debugger::Debugger;
use crate::session::player::SessionPlayer;
use crate::session::world::SessionWorld;
use engine::{Engine, EngineFrame};
use game::channel::ClientChannel;
use game::Game;
use lib::fps::Fps;
use lib::time::DeltaTime;
use math::size::Size2;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;

pub mod chunk;
pub mod player;
pub mod debugger;
pub mod world;

pub struct GameSession {
    channel: ClientChannel,
    fps: Fps,
    debugger: Debugger,
    delta_time: DeltaTime,
    player: SessionPlayer,
    is_focused: bool,
    pub(crate) world: SessionWorld,
}

impl GameSession {
    pub fn create(size: Size2<u32>) -> Self {
        let (channel, chunk_channel) = Game::spawn();
        let player = SessionPlayer::create(&channel);

        Self {
            channel,
            debugger: Debugger::create(size),
            delta_time: DeltaTime::new(),
            fps: Fps::new(),
            player,
            is_focused: false,
            world: SessionWorld::new(chunk_channel),
        }
    }

    pub fn update(&mut self, frame: &EngineFrame, engine: &mut Engine) {
        self.world.chunk_map.update(&engine.gpu.handle, &engine.renderer_3d.pipeline.texture_positions);
        self.check_cursor_lock(frame, engine);

        while let Some(output_receiver) = self.channel.recv_client_output() {
            self.player.output_receiver = Some(output_receiver);
        }

        let dt = self.delta_time.next();

        self.player.update(&engine.gpu.handle, &mut engine.renderer_3d, (&frame.input, &engine.input), self.is_focused);
        self.fps.update(dt);
        self.debugger.update(frame, &mut engine.renderer_2d, &self.fps, engine.renderer_3d.camera.pos);
    }

    pub fn set_size(&mut self, size: Size2<u32>) {
        self.debugger.set_size(size);
    }

    pub fn exit(&mut self) {
        self.channel.send_exit();
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