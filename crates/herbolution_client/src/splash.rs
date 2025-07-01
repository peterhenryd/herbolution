use std::time::Duration;

use math::color::{Color, Rgba};

use crate::app::state::Command;
use crate::app::{Render, Update};
use crate::menu::config::MenuConfig;

#[derive(Debug)]
pub struct Splash {
    lifetime: Duration,
}

impl Splash {
    pub fn update(&mut self, ctx: &mut Update) -> Option<Command> {
        self.lifetime = self.lifetime.saturating_sub(ctx.dt);

        if self.lifetime > Duration::ZERO {
            None
        } else {
            Some(Command::OpenMenu(MenuConfig::Title))
        }
    }

    pub fn render(&mut self, ctx: &mut Render) {
        ctx.frame.clear_color(Rgba::from_rgb(20u8, 40, 80).into())
    }
}

impl Default for Splash {
    fn default() -> Self {
        Self {
            lifetime: Duration::from_millis(650),
        }
    }
}
