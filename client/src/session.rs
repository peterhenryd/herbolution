use std::path::Path;
use std::time::Duration;

use lib::color::{ColorConsts, Rgba};
use lib::save::Save;
use lib::size::size2u;
use lib::util::IntervalCounter;
use lib::vector::{vec3d, Vec2};
use server::handle::GameHandle;
use server::{Game, Options};
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;

use crate::app::{Command, Render, Update};
use crate::video::resource::{Mesh, MeshId, Meshes};
use crate::video::ui::text::{Text, TextBrush};
use crate::video::world::Vertex3d;
use crate::video::{world, Video};
use crate::world::World;

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
    pub fn create(save: Save, video: &mut Video, assets_path: &Path) -> Self {
        let handle = Game::spawn(Options { save });

        Self {
            world: World::new(video),
            handle,
            mesh_ids: MeshIds::from_insertion_into(video.sculptor.meshes(), assets_path),
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
            let mut chisel = ctx.frame.draw_3d(world::RenderType::Sky);
            chisel.load_mesh(self.mesh_ids.solid_quad);
            chisel.render_each_by_id(self.world.player.sky_box_id);
        }

        // Render world
        {
            ctx.frame.advance_pass(None, false);
            let mut chisel = ctx.frame.draw_3d(world::RenderType::Terrain);

            self.world.render(&self.mesh_ids, &mut chisel);

            chisel.load_mesh(self.mesh_ids.shell_quad);
            chisel.render_each_by_id(self.world.player.targeted_cube_shell_id);

            chisel.load_mesh(self.mesh_ids.wire_quad);
            chisel.render_each_by_id(self.world.player.targeted_cube_wireframe_id);
        }

        // Render debugger and cross-hair
        {
            let mut brush = ctx.frame.draw_2d();
            let mut text_brush = brush.draw_text();

            self.debugger
                .render(self.fps.get(), self.world.player.camera.position, &mut text_brush);

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

#[derive(Debug, Default)]
pub struct Debugger {
    is_enabled: bool,
}

impl Debugger {
    pub fn update(&mut self, ctx: &mut Update) {
        if ctx.input.key_events.contains(&KeyCode::Backquote) {
            self.is_enabled = !self.is_enabled;
        }
    }

    pub fn render(&mut self, fps: u64, player_position: vec3d, brush: &mut TextBrush) {
        if !self.is_enabled {
            return;
        }

        let font_id = brush.font_id();
        brush.add(
            Vec2::ZERO,
            &Text {
                font_id,
                content: format!("FPS: {fps}"),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );

        brush.add(
            Vec2::new(0., 40.),
            &Text {
                font_id,
                content: format!("X: {:.2}", player_position.x),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );

        brush.add(
            Vec2::new(0., 80.),
            &Text {
                font_id,
                content: format!("Y: {:.2}", player_position.y),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );

        brush.add(
            Vec2::new(0., 120.),
            &Text {
                font_id,
                content: format!("Z: {:.2}", player_position.z),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );
    }
}

/// A utility structure that holds the mesh handles used for rendering the server.
#[derive(Debug)]
pub struct MeshIds {
    pub(crate) solid_quad: MeshId,
    pub(crate) wire_quad: MeshId,
    pub(crate) shell_quad: MeshId,
}

impl MeshIds {
    pub fn from_insertion_into(meshes: &mut Meshes<Vertex3d>, asset_path: &Path) -> Self {
        Self {
            solid_quad: meshes.create_and_insert_from(|handle| Mesh::read(handle, asset_path.join("mesh/quad.toml"))),
            wire_quad: meshes.create_and_insert_from(|handle| Mesh::read(handle, asset_path.join("mesh/quad_wire.toml"))),
            shell_quad: meshes.create_and_insert_from(|handle| Mesh::read(handle, asset_path.join("mesh/quad_shell.toml"))),
        }
    }
}
