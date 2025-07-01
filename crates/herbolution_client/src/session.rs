use std::time::Duration;

use crate::app::state::Command;
use crate::app::{Render, Update};
use crate::debugger::Debugger;
use crate::mesh::MeshIds;
use crate::world::World;
use engine::painter::text::Text;
use engine::{sculptor, Engine};
use game::handle::GameHandle;
use game::{Game, Options};
use herbolution_lib::util::time::IntervalCounter;
use lib::save::Save;
use math::color::{ColorConsts, Rgba};
use math::size::size2u;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;

/// The render-side representation of a herbolution_game session.
#[derive(Debug)]
pub struct Session {
    world: World,
    handle: GameHandle,
    mesh_ids: MeshIds,

    is_paused: bool,
    is_focused: bool,
    fps: IntervalCounter,

    debugger: Debugger,
}

impl Session {
    /// Creates a new instance, and spawns an associated behavior-side herbolution_game.
    pub fn create(save: Save, engine: &mut Engine) -> Self {
        let handle = Game::spawn(Options { save });

        Self {
            world: World::new(engine),
            handle,
            mesh_ids: MeshIds::from_insertion_into(engine.video.sculptor.meshes()),
            is_paused: false,
            is_focused: false,
            fps: IntervalCounter::new(Duration::SECOND),
            debugger: Debugger::default(),
        }
    }

    /// Updates the herbolution_game session state, processing input and updating the player and world.
    pub fn update(&mut self, ctx: &mut Update) -> Option<Command> {
        self.fps.update(ctx.dt);

        if ctx.input.key_events.contains(&KeyCode::Escape) {
            ctx.window
                .set_cursor_grab(CursorGrabMode::None)
                .unwrap();
            ctx.window.set_cursor_visible(true);
            self.is_focused = false;
        } else if ctx
            .input
            .click_events
            .iter()
            .any(|x| x.button == MouseButton::Left)
        {
            ctx.window
                .set_cursor_grab(CursorGrabMode::Locked)
                .unwrap();
            ctx.window.set_cursor_visible(false);
            self.is_focused = true;
        }

        self.world
            .update(self.is_focused, &self.handle, ctx);
        self.debugger.update(ctx);

        None
    }

    /// Renders the herbolution_game.
    pub fn render(&mut self, ctx: &mut Render) {
        // Render sky box
        {
            let mut chisel = ctx.frame.draw_3d(sculptor::RenderType::Sky);
            chisel.load_mesh(self.mesh_ids.solid_quad);
            chisel.render_each_by_id(self.world.player.sky_box_id);
        }

        // Render world
        {
            ctx.frame.advance_pass(None, false);
            let mut chisel = ctx.frame.draw_3d(sculptor::RenderType::Terrain);

            self.world.render(&self.mesh_ids, &mut chisel);

            chisel.load_mesh(self.mesh_ids.wire_quad);
            chisel.render_each_by_id(self.world.player.targeted_cube_wireframe_id);
        }

        // Render debugger and cross-hair
        {
            let mut brush = ctx.frame.draw_2d();
            let mut text_brush = brush.draw_text();

            self.debugger
                .render(self.fps.get(), self.world.player.camera.video.position, &mut text_brush);

            let font_id = text_brush.font_id();
            text_brush.add(
                ctx.resolution.to_vec2().cast() / 2.0,
                &Text {
                    font_id,
                    content: "+".to_string(),
                    font_size: 24.0,
                    color: Rgba::WHITE,
                },
            );
        }
    }

    pub fn set_resolution(&mut self, _: size2u) {}

    /// Sends a signal to the behavior-side herbolution_game to exit.
    pub fn exit(&mut self) {
        self.handle.request_exit();
    }

    pub fn pause(&mut self) {
        self.is_paused = !self.is_paused;
    }
}
