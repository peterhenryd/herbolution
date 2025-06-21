use engine::video::painter::text::Text;
use lib::fs::save::Save;
use math::color::{ColorConsts, Rgba};
use math::vec::Vec2;

use crate::app::{Render, Update};
use crate::menu::Menu;
use crate::menu::config::MenuConfig;
use crate::session::Session;

/// The navigable state of the application. This structure uses the senses provided by the engine and persistent data during the update phase to mutate itself
/// for the following cycles.
#[derive(Debug, Default)]
pub enum State {
    /// The initial state of the application, where it is loading resources. A splash screen is rendering during this state. Upon completion, it transitions to
    /// the title menu.
    #[default]
    Loading,
    /// The state where the user is not actively playing, and is viewing and interacting with a given menu.
    Browsing(Menu),
    /// The state where the user is playing the game. It also manages a potential overlay menu and has pause mechanics.
    Playing {
        overlay: Option<Menu>,
        session: Session,
        is_paused: bool,
    },
    Exiting,
}

impl State {
    /// Updates the current state of the application using the provided context.
    pub fn update(&mut self, context: &mut Update) {
        let mut command = None;

        // Run the update logic for the current state.
        match self {
            State::Loading => {
                // Loading...

                command = Some(Command::OpenMenu(MenuConfig::Title));
            }
            State::Browsing(menu) => {
                command = menu.update(context, None);
            }
            State::Playing {
                overlay: menu,
                session,
                is_paused,
            } => {
                // First, update the game if it is not paused.
                if !*is_paused {
                    command = command.or(session.update(context));
                }

                // Then, update the menu if one exists.
                if let Some(menu) = menu {
                    command = menu.update(context, Some(is_paused));
                }
            }
            State::Exiting => {
                context.event_loop.exit();
            }
        }

        // If a command was returned, transition to its associated state.
        if let Some(x) = command {
            self.transition(x, context);
        };
    }

    // Replaces or mutates the current state based on the command.
    fn transition(&mut self, command: Command, context: &mut Update) {
        match command {
            Command::OpenMenu(config) => {
                *self = State::Browsing(config.into());
            }
            Command::StartGame { save } => {
                *self = Self::Playing {
                    overlay: None,
                    session: Session::create(save, &mut context.engine),
                    is_paused: false,
                };
            }
            Command::Exit => {
                *self = State::Exiting;
            }
            Command::PauseGame => {
                if let State::Playing { is_paused, .. } = self {
                    *is_paused = !*is_paused;
                }
            }
        }
    }

    /// Renders the current state of the application using the provided context.
    pub fn render<'a>(&mut self, context: &mut Render) {
        match self {
            State::Loading => {
                // TODO: render splash screen
            }
            State::Browsing(menu) => {
                menu.render(&mut context.drawing.draw_2d());
            }
            State::Playing {
                overlay: menu,
                session,
                is_paused,
            } => {
                // First, render the game if it is not paused.
                if !*is_paused {
                    session.render(context);
                }

                let mut drawing = context.drawing.draw_2d();
                let mut text_drawing = drawing.draw_text();

                let font_id = text_drawing
                    .atlas
                    .font_coords
                    .keys()
                    .next()
                    .unwrap()
                    .font_id;
                text_drawing.add(
                    Vec2::ZERO,
                    Text {
                        font_id,
                        content: format!("FPS: {}", context.persist.fps.get()),
                        font_size: 36.0,
                        color: Rgba::WHITE,
                    },
                );

                text_drawing.add(
                    context
                        .resolution
                        .to_vec2()
                        .cast()
                        .unwrap()
                        / 2.0,
                    Text {
                        font_id,
                        content: "+".to_owned(),
                        font_size: 36.0,
                        color: Rgba::WHITE,
                    },
                );

                drop(text_drawing);

                // Then, render the overlay over the game if one exists.
                if let Some(menu) = menu {
                    menu.render(&mut drawing);
                }
            }
            State::Exiting => {}
        }
    }
}

/// A state configuration used to construct the next state of the application.
#[derive(Debug, Clone)]
pub enum Command {
    /// Opens the specified menu.
    OpenMenu(MenuConfig),
    /// Starts the game with the specified save.
    StartGame {
        save: Save,
    },
    PauseGame,
    /// Exits the application.
    Exit,
}
