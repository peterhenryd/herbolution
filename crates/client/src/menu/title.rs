use std::random::random;
use std::time::Duration;

use engine::video::v2d;
use lib::fs::save::{SaveAttributes, WorldAttributes, WorldDescriptor};

use crate::app::state::Command;
use crate::app::Update;

/// The title menu, where the user can view information about the game, navigate to other menus, or quit the application.
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

        if self.timer.as_secs_f32() >= 2.0 {
            let save = context
                .persist
                .fs
                .create_or_open_save(
                    "default",
                    SaveAttributes {
                        title: "Default".to_string(),
                        default_world: WorldAttributes {
                            name: "world".to_string(),
                            descriptor: WorldDescriptor { title: "Overworld".to_string(), seed: random() },
                        },
                    },
                )
                .unwrap();

            return Some(Command::StartGame { save });
        }

        None
    }

    /// Renders the title menu.
    pub fn render<'t>(&'t mut self, drawing: &mut v2d::Drawing<'_, '_, 't>) {
        let _ = drawing;

        // TODO: render the title menu
    }
}
