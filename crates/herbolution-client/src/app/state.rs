use cg_kiln::{Env2d, FontId};
use lib::save::Save;
use prims::camera::{Camera, Orthographic, View};
use prims::color::{Color, ColorConsts, Rgba};
use prims::vector::{Vec2, Vec3};
use time::Duration;

use crate::app::{Render, Update};
use crate::menu::{Menu, MenuConfig};
use crate::session::Session;

#[derive(Debug)]
pub enum State {
    Loading(Splash),
    Browsing(Menu),
    Playing(Session),
}

impl State {
    pub fn update(&mut self, ctx: &mut Update) {
        let command = match self {
            State::Loading(splash) => splash.update(ctx),
            State::Browsing(menu) => menu.update(ctx),
            State::Playing(session) => session.update(ctx),
        };

        let proj: Orthographic = ctx.app.video.size().into();
        ctx.app.core2d.write_env(
            &ctx.app.gpu,
            &Env2d {
                view_proj: Camera::new(Vec3::ZERO, View::Forward, proj).view_proj,
            },
        );

        if let Some(x) = command {
            self.transition(x, ctx);
        };
    }

    pub(crate) fn transition(&mut self, command: Command, ctx: &mut Update) {
        match command {
            Command::OpenMenu(config) => {
                *self = State::Browsing(Menu::new(config));
            }
            Command::StartGame { save } => {
                let session = Session::create(save, ctx.app.video.size().cast().aspect());
                *self = Self::Playing(session);
            }
            Command::Exit => {
                ctx.event_loop.exit();
            }
            Command::PauseGame => {
                if let State::Playing(session) = self {
                    session.pause()
                }
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn render(&mut self, ctx: &mut Render) {
        match self {
            State::Loading(splash) => splash.render(ctx),
            State::Browsing(menu) => menu.render(ctx),
            State::Playing(session) => session.render(ctx),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    OpenMenu(MenuConfig),
    StartGame { save: Save },
    PauseGame,
    Exit,
}

#[derive(Debug)]
pub struct Splash {
    lifetime: Duration,
}

impl Splash {
    pub fn new() -> Self {
        Self {
            lifetime: Duration::milliseconds(750),
        }
    }

    pub fn update(&mut self, ctx: &mut Update) -> Option<Command> {
        self.lifetime -= ctx.dt;

        if self.lifetime > Duration::ZERO {
            None
        } else {
            Some(Command::OpenMenu(MenuConfig::Title))
        }
    }

    pub fn render(&mut self, ctx: &mut Render<'_>) {
        ctx.frame
            .push_clear_color(Rgba::from_rgb(20u8, 40, 80).into());

        let mut p2d = ctx.frame.prepare_2d(ctx.app.core2d);
        let mut ui = p2d.draw_ui(ctx.app.core2d);

        ui.draw_text(
            (ctx.resolution.to_vec2().cast::<f32>() - Vec2::new(504.576, 56.0)) / 2.0,
            &kiln::Text {
                font_id: FontId::default(),
                content: "Herbolution".to_string(),
                font_size: 96.0,
                color: Rgba::WHITE,
            },
        );
    }
}
