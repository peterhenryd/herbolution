use herbolution_lib::save::Save;

use crate::app::{Render, Update};
use crate::menu::config::MenuConfig;
use crate::menu::Menu;
use crate::session::Session;
use crate::splash::Splash;

/// The navigable state of the application. This structure uses the senses provided by the herbolution_engine and persistent data during the update phase to
/// mutate itself for the following cycles.
#[derive(Debug)]
pub enum State {
    /// The initial state of the application, where it is loading resources. A splash screen is rendering during this state. Upon completion, it transitions to
    /// the title menu.
    Loading(Splash),
    /// The state where the user is not actively playing, and is viewing and interacting with a given menu.
    Browsing(Menu),
    /// The state where the user is playing the herbolution_game. It also manages a potential overlay menu and has pause mechanics.
    Playing(Session),
    Exiting,
}

impl State {
    /// Updates the current state of the application using the provided context.
    pub fn update(&mut self, context: &mut Update) {
        // Run the update behavior for the current state.
        let command = match self {
            State::Loading(splash) => splash.update(context),
            State::Browsing(menu) => menu.update(context),
            State::Playing(session) => session.update(context),
            State::Exiting => return context.event_loop.exit(),
        };

        // If a command was returned, transition to its associated state.
        if let Some(x) = command {
            self.transition(x, context);
        };
    }

    // Replaces or mutates the current state based on the command.
    fn transition(&mut self, command: Command, ctx: &mut Update) {
        match command {
            Command::OpenMenu(config) => {
                *self = State::Browsing(config.into());
            }
            Command::StartGame { save } => {
                let session = Session::create(save, &mut ctx.engine);
                *self = Self::Playing(session);
            }
            Command::Exit => {
                *self = State::Exiting;
            }
            Command::PauseGame => {
                if let State::Playing(session) = self {
                    session.pause()
                }
            }
        }
    }

    /// Renders the current state of the application using the provided context.
    pub fn render(&mut self, ctx: &mut Render) {
        match self {
            State::Loading(splash) => splash.render(ctx),
            State::Browsing(menu) => menu.render(ctx),
            State::Playing(session) => session.render(ctx),
            State::Exiting => {}
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Loading(Splash::default())
    }
}

/// A state configuration used to construct the next state of the application.
#[derive(Debug, Clone)]
pub enum Command {
    /// Opens the specified menu.
    OpenMenu(MenuConfig),
    /// Starts the herbolution_game with the specified save.
    StartGame {
        save: Save,
    },
    PauseGame,
    /// Exits the application.
    Exit,
}
