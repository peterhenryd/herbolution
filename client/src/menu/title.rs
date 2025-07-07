use crate::app::{Command, Update};
use crate::input::ClickEvent;
use crate::video::ui::brush::{Brush, Text};
use lib::color::{ColorConsts, Rgba};
use lib::save::{SaveAttributes, WorldAttributes, WorldDescriptor};
use lib::size::Size2;
use lib::vector::Vec2;
use std::random::random;
use std::time::Duration;
use winit::event::MouseButton;

/// The title menu, where the user can view information about the server, navigate to other menus, or quit the application.
#[derive(Debug)]
pub struct TitleMenu {
    timer: Duration,
}

impl TitleMenu {
    /// Creates a new instance of the title menu.
    pub fn new() -> Self {
        Self { timer: Duration::ZERO }
    }

    /// Updates the title menu state.
    pub fn update(&mut self, context: &mut Update) -> Option<Command> {
        self.timer += context.dt;

        /*
        if self.timer.as_secs_f32() >= 2.0 {
        }

         */

        for &ClickEvent { button, position } in &context.input.click_events {
            if button != MouseButton::Left {
                continue;
            }

            if position.x < 128. || position.x > 256. || position.y < 192. || position.y > 256. {
                continue;
            }

            let save = context
                .store
                .fs
                .create_or_open_save(
                    "default",
                    SaveAttributes {
                        title: "Default".to_string(),
                        default_world: WorldAttributes {
                            name: "world".to_string(),
                            descriptor: WorldDescriptor {
                                title: "Overworld".to_string(),
                                seed: random(),
                            },
                        },
                    },
                )
                .unwrap();

            return Some(Command::StartGame { save });
        }

        None
    }

    /// Renders the title menu.
    pub fn render<'t>(&'t mut self, brush: &mut Brush<'_, '_, 't>) {
        let font_id = brush.default_font_id();

        brush.draw_text(
            Vec2::splat(128.),
            &Text {
                font_id,
                content: "Herbolution".to_string(),
                font_size: 96.0,
                color: Rgba::BLACK,
            },
        );

        brush.draw_rect(Vec2::new(128., 192.), Size2::new(208., 64.), Rgba::BLACK);
        brush.draw_text(
            Vec2::new(136., 212.),
            &Text {
                font_id,
                content: "Start Game".to_string(),
                font_size: 36.0,
                color: Rgba::WHITE,
            },
        );
    }
}
