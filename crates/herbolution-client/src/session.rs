use kiln::{DrawUi, FontId, Text};
use lib::save::Save;
use lib::util::IntervalCounter;
use prims::aabb::Aabb2;
use prims::color::{Color, ColorConsts, Rgba};
use prims::size::{Size2, size2u};
use prims::vector::{Vec2, vec3d};
use server::handle::GameHandle;
use server::{Game, Options};
use time::Duration;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;

use crate::app::state::Command;
use crate::app::{Render, Update};
use crate::world::World;

#[derive(Debug)]
pub struct Session {
    world: World,
    handle: GameHandle,
    is_paused: bool,
    is_focused: bool,
    debugger: Debugger,
}

impl Session {
    pub fn create(save: Save, camera_aspect: f32) -> Self {
        let handle = Game::spawn(Options { save });

        Self {
            world: World::new(camera_aspect),
            handle,
            is_paused: false,
            is_focused: false,
            debugger: Debugger::default(),
        }
    }

    pub fn update(&mut self, ctx: &mut Update) -> Option<Command> {
        let window = ctx.app.window;
        if ctx.input.key_events.contains(&KeyCode::Escape) {
            window
                .set_cursor_grab(CursorGrabMode::None)
                .unwrap();
            window.set_cursor_visible(true);
            self.is_focused = false;
        } else if ctx
            .input
            .click_events
            .iter()
            .any(|x| x.button == MouseButton::Left)
        {
            window
                .set_cursor_grab(CursorGrabMode::Locked)
                .unwrap();
            window.set_cursor_visible(false);
            self.is_focused = true;
        }

        self.world
            .update(self.is_focused, &self.handle, ctx);
        self.debugger.update(ctx);

        None
    }

    #[tracing::instrument(name = "render_session", skip_all)]
    pub fn render(&mut self, ctx: &mut Render) {
        ctx.frame
            .push_clear_color(Rgba::<u8>::from_rgb(167, 232, 250).into());

        let mesh = ctx
            .app
            .resources
            .mesh_buffers
            .get("quad")
            .unwrap();
        self.world.render(
            &mut ctx
                .frame
                .prepare_3d(ctx.app.core3d)
                .load_mesh(mesh),
        );

        let mut d2 = ctx.frame.prepare_2d(ctx.app.core2d);
        let mut ui = d2.draw_ui(ctx.app.core2d);

        self.render_hud(ctx.resolution, &mut ui);
        self.debugger
            .render(ctx.app.fps.get(), self.world.player.state.pos, &mut ui);
    }

    #[tracing::instrument(skip_all)]
    fn render_hud(&mut self, resolution: size2u, ui: &mut DrawUi) {
        let health = self.world.player.state.health.get();
        let health_factor = self.world.player.state.health.percent();

        let position = Vec2::new(resolution.width as f32 - 64. - 136., resolution.height as f32 - 48. - 64.);
        let scale = Size2::new(128., 48.);

        let filled_scale = Size2::new(scale.width * health_factor, scale.height);
        let unfilled_position = position + Vec2::new(filled_scale.width, 0.);
        let unfilled_scale = Size2::new(scale.width * (1.0 - health_factor), scale.height);

        ui.draw_rect(
            Aabb2::sized(
                Vec2::new(resolution.width as f32 - 68. - 136., resolution.height as f32 - 48. - 64. - 4.),
                Size2::new(136., 56.),
            ),
            Rgba::from_rgb(0, 0, 0).into(),
            6.0,
        );
        ui.draw_rect(Aabb2::sized(position, filled_scale), Rgba::from_rgb(255, 0, 0).into(), 4.0);
        ui.draw_rect(Aabb2::sized(unfilled_position, unfilled_scale), Rgba::BLACK, 4.0);

        ui.draw_text(
            position + Vec2::new(24., 16.),
            &Text {
                font_id: FontId::default(),
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
    pub fn render(&mut self, fps: u64, player_position: vec3d, ui: &mut DrawUi) {
        if !self.is_enabled {
            return;
        }

        ui.draw_text(
            Vec2::ZERO,
            &Text {
                font_id: FontId::default(),
                content: format!("FPS: {fps}"),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );

        ui.draw_text(
            Vec2::new(0., 40.),
            &Text {
                font_id: FontId::default(),
                content: format!("X: {:.2}", player_position.x),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );

        ui.draw_text(
            Vec2::new(0., 80.),
            &Text {
                font_id: FontId::default(),
                content: format!("Y: {:.2}", player_position.y),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );

        ui.draw_text(
            Vec2::new(0., 120.),
            &Text {
                font_id: FontId::default(),
                content: format!("Z: {:.2}", player_position.z),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );
    }
}
