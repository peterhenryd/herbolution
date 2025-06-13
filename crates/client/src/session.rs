use crate::debugger::Debugger;
use crate::player::Player;
use crate::world::World;
use engine::input::InputFrame;
use engine::video::text::TextFrame;
use engine::{video, Engine};
use game::channel::ClientChannel;
use game::{Game, Options};
use lib::fps::IntervalCounter;
use lib::fs::save::Save;
use lib::time::DeltaTime;
use math::size::Size2;
use std::time::Duration;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::window::{CursorGrabMode, Window};
use engine::video::r3d::RenderType3d;
use math::color::Color;

pub struct Session {
    channel: ClientChannel,
    fps: IntervalCounter,
    debugger: Debugger,
    delta_time: DeltaTime,
    pub(crate) player: Player,
    is_focused: bool,
    pub(crate) world: World,
}

impl Session {
    pub fn create(save: Save, size: Size2<u32>, engine: &mut Engine) -> Self {
        let (channel, chunk_channel) = Game::spawn(Options { save });
        let world = World::new(chunk_channel, engine);
        let player = Player::create(&channel, size, world.render_settings.fog_color.into_rgba(), engine);

        Self {
            channel,
            debugger: Debugger::create(size),
            delta_time: DeltaTime::new(),
            fps: IntervalCounter::new(Duration::SECOND),
            player,
            is_focused: false,
            world,
        }
    }
    
    pub fn update(&mut self, engine: &mut Engine, input: &InputFrame, text: &mut TextFrame) {
        self.check_cursor_lock(&engine.window, input);
        
        if let Some(rx) = self.channel.recv_client_output() {
            self.player.rx = Some(rx);
        }

        let dt = self.delta_time.next();
        self.fps.update(dt);

        self.player.update(dt, engine, input, self.is_focused);
        self.world.update(dt, &mut engine.video);
        self.debugger.update(input, text, &self.fps, self.player.camera.video.position);
    }

    pub fn render(&mut self, frame: &mut video::Frame) {
        {
            let mut draw_sky = frame.draw_3d(RenderType3d::Sky);
            draw_sky.load_mesh(self.world.mesh_ids.solid_quad);
            draw_sky.draw_from_set(self.player.sky_box_id);
        }

        {
            let mut draw_terrain = frame.draw_3d(RenderType3d::Terrain);

            self.world.render(&self.player.camera, &mut draw_terrain);

            draw_terrain.load_mesh(self.world.mesh_ids.wireframe_quad);
            draw_terrain.draw_from_set(self.player.targeted_cube_wireframe_id);
        }
    }

    pub fn set_resolution(&mut self, size: Size2<u32>) {
        self.debugger.set_size(size);
    }

    pub fn exit(&mut self) {
        self.channel.send_exit();
    }

    fn check_cursor_lock(&mut self, window: &Window, input: &InputFrame) {
        if input.key_events.contains(&KeyCode::Escape) {
            window.set_cursor_grab(CursorGrabMode::None).unwrap();
            window.set_cursor_visible(true);
            self.is_focused = false;
        } else if input.click_events.iter().any(|x| x.button == MouseButton::Left) {
            window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
            window.set_cursor_visible(false);
            self.is_focused = true;
        }
    }
}
