use lib::fs::save::Save;

use crate::app::{Render, Update};
use crate::menu::config::MenuConfig;
use crate::menu::Menu;
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
        let mut selection = None;
        // Run the update logic for the current state.
        match self {
            State::Loading => {
                // Loading...

                selection = Some(Command::OpenMenu(MenuConfig::Title));
            }
            State::Browsing(menu) => {
                selection = menu.update(context, None);
            }
            State::Playing {
                overlay: menu,
                session,
                is_paused,
            } => {
                // First, update the game if it is not paused.
                if !*is_paused {
                    selection = selection.or(session.update(context));
                }

                // Then, update the menu if one exists.
                if let Some(menu) = menu {
                    selection = menu.update(context, Some(is_paused));
                }
            }
            State::Exiting => {
                context.event_loop.exit();
            }
        }

        // If a command was returned, transition to its associated state.
        if let Some(selection) = selection {
            self.transition(selection, context);
        };
    }

    // Mutates the current state based on the command.
    fn transition(&mut self, selection: Command, context: &mut Update) {
        match selection {
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
                let mut drawing = context.drawing.begin_2d();
                menu.render(&mut drawing);
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

                // Then, render the menu over the game if one exists.
                if let Some(menu) = menu {
                    menu.render(&mut context.drawing.begin_2d());
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
