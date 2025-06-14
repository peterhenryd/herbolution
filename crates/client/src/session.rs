use std::time::Duration;

use engine::input::Frame;
use engine::video::v3d;
use engine::Engine;
use game::channel::ClientChannel;
use game::{Game, Options};
use lib::fps::IntervalCounter;
use lib::fs::save::Save;
use lib::time::DeltaTime;
use math::color::{Color, Rgb};
use math::size::Size2;
use math::vector::Vec3;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::window::{CursorGrabMode, Window};

use crate::app::state::Command;
use crate::app::{Render, Update};
use crate::debugger::Debugger;
use crate::mesh::MeshIds;
use crate::world::player::Player;
use crate::world::World;

/// The render-side representation of a game session.
#[derive(Debug)]
pub struct Session {
    fps: IntervalCounter,
    debugger: Debugger,
    delta_time: DeltaTime,
    pub(crate) world: World,
    pub(crate) player: Player,
    channel: ClientChannel,
    is_focused: bool,
    pub(crate) mesh_ids: MeshIds,
}

impl Session {
    /// Creates a new instance, and spawns an associated logic-side game.
    pub fn create(save: Save, engine: &mut Engine) -> Self {
        let (channel, chunk_channel) = Game::spawn(Options { save });
        let world_render_settings = v3d::World {
            ambient_light: Vec3::splat(0.5),
            light_dir: Vec3::new(0.2, 1.0, -0.7).normalize(),
            fog_color: Rgb::<u8>::from_rgb(177, 242, 255).into(),
            fog_distance: 150.0,
        };

        Self {
            debugger: Debugger::new(engine.video.resolution()),
            delta_time: DeltaTime::new(),
            fps: IntervalCounter::new(Duration::SECOND),
            player: Player::create(&channel, world_render_settings.fog_color.into_rgba(), engine),
            world: World::new(chunk_channel, world_render_settings),
            channel,
            is_focused: false,
            mesh_ids: MeshIds::from_insertion_into(engine.video.r3d.meshes()),
        }
    }

    /// Updates the game session state, processing input and updating the player and world.
    pub fn update(&mut self, context: &mut Update) -> Option<Command> {
        self.check_cursor_lock(&context.window, &context.input);

        if let Some(rx) = self.channel.recv_client_output() {
            self.player.rx = Some(rx);
        }

        let dt = self.delta_time.next();
        self.fps.update(dt);

        self.player.update(dt, &mut context.engine, &context.input, self.is_focused);
        self.world.update(dt, &mut context.engine.video);

        self.debugger.update(&context.input, &self.fps, self.player.camera.video.position);

        None
    }

    /// Renders the game.
    pub fn render(&mut self, context: &mut Render) {
        {
            let mut draw_sky = context.drawing.begin_3d(v3d::RenderType::Sky);
            draw_sky.load_mesh(self.mesh_ids.solid_quad);
            draw_sky.draw_from_set(self.player.sky_box_id);
        }

        {
            let mut draw_terrain = context.drawing.begin_3d(v3d::RenderType::Terrain);

            self.world.render(&self.player.camera, &self.mesh_ids, &mut draw_terrain);

            draw_terrain.load_mesh(self.mesh_ids.wireframe_quad);
            draw_terrain.draw_from_set(self.player.targeted_cube_wireframe_id);
        }
    }

    pub fn set_resolution(&mut self, size: Size2<u32>) {
        self.debugger.set_resolution(size);
    }

    /// Sends a signal to the logic-side game to exit.
    pub fn exit(&mut self) {
        self.channel.send_exit();
    }

    fn check_cursor_lock(&mut self, window: &Window, input_frame: &Frame) {
        if input_frame.key_events.contains(&KeyCode::Escape) {
            window.set_cursor_grab(CursorGrabMode::None).unwrap();
            window.set_cursor_visible(true);
            self.is_focused = false;
        } else if input_frame.click_events.iter().any(|x| x.button == MouseButton::Left) {
            window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
            window.set_cursor_visible(false);
            self.is_focused = true;
        }
    }
}
