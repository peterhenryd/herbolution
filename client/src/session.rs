use lib::aabb::Aabb2;
use lib::color::{Color, ColorConsts, Rgba};
use lib::save::Save;
use lib::size::{size2u, Size2};
use lib::util::IntervalCounter;
use lib::vector::{vec3d, Vec2};
use server::handle::GameHandle;
use server::{Game, Options};
use std::path::Path;
use time::Duration;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;

use crate::app::{Command, Render, Update};
use crate::video::resource::{Mesh, MeshId, Meshes};
use crate::video::ui::brush::{Brush, Text};
use crate::video::world::Vertex3d;
use crate::video::{world, Video};
use crate::world::World;

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

    #[tracing::instrument(name = "render_session", skip_all)]
    pub fn render(&mut self, ctx: &mut Render) {
        {
            let mut chisel = ctx.frame.draw_3d(world::RenderType::Sky);
            chisel.load_mesh(self.mesh_ids.solid_quad);
            chisel.render_each_by_id(self.world.player.sky_box_id);
        }

        {
            let mut chisel = ctx.frame.draw_3d(world::RenderType::Terrain);

            self.world.render(&self.mesh_ids, &mut chisel);

            chisel.load_mesh(self.mesh_ids.shell_quad);
            chisel.render_each_by_id(self.world.player.targeted_cube_shell_id);

            chisel.load_mesh(self.mesh_ids.wire_quad);
            chisel.render_each_by_id(self.world.player.targeted_cube_wireframe_id);
        }

        {
            let mut brush = ctx.frame.draw_2d();

            self.render_hud(ctx.resolution, &mut brush);

            self.debugger
                .render(self.fps.get(), self.world.player.state.position, &mut brush);

            let font_id = brush.default_font_id();
            brush.draw_text(
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

    #[tracing::instrument(skip_all)]
    fn render_hud(&mut self, resolution: size2u, brush: &mut Brush) {
        let health = self.world.player.state.health.get();
        let health_factor = self.world.player.state.health.percent();

        let position = Vec2::new(resolution.width as f32 - 64. - 136., resolution.height as f32 - 48. - 64.);
        let scale = Size2::new(128., 48.);

        let filled_scale = Size2::new(scale.width * health_factor, scale.height);
        let unfilled_position = position + Vec2::new(filled_scale.width, 0.);
        let unfilled_scale = Size2::new(scale.width * (1.0 - health_factor), scale.height);

        brush.draw_rect(
            Aabb2::sized(
                Vec2::new(resolution.width as f32 - 68. - 136., resolution.height as f32 - 48. - 64. - 4.),
                Size2::new(136., 56.),
            ),
            Rgba::from_rgb(0, 0, 0).into(),
            6.0,
        );
        brush.draw_rect(Aabb2::sized(position, filled_scale), Rgba::from_rgb(255, 0, 0).into(), 4.0);
        brush.draw_rect(Aabb2::sized(unfilled_position, unfilled_scale), Rgba::BLACK, 4.0);

        brush.draw_text(
            position + Vec2::new(24., 16.),
            &Text {
                font_id: brush.default_font_id(),
                content: format!("{:.0} HP", health),
                font_size: 24.0,
                color: Rgba::WHITE,
            },
        );
    }

    pub fn set_resolution(&mut self, _: size2u) {}

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

    #[tracing::instrument(skip_all)]
    pub fn render(&mut self, fps: u64, player_position: vec3d, brush: &mut Brush) {
        if !self.is_enabled {
            return;
        }

        let font_id = brush.default_font_id();
        brush.draw_text(
            Vec2::ZERO,
            &Text {
                font_id,
                content: format!("FPS: {fps}"),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );

        brush.draw_text(
            Vec2::new(0., 40.),
            &Text {
                font_id,
                content: format!("X: {:.2}", player_position.x),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );

        brush.draw_text(
            Vec2::new(0., 80.),
            &Text {
                font_id,
                content: format!("Y: {:.2}", player_position.y),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );

        brush.draw_text(
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
