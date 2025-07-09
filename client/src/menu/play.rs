use crate::app::{Command, Render, Update};

/// The play menu, where the user can view pre-existing saves, create new ones, or start the server.
#[derive(Debug)]
pub struct PlayMenu {}

impl PlayMenu {
    /// Creates a new instance of the play menu.
    pub fn new() -> Self {
        Self {}
    }

    /// Updates the play menu state.
    pub fn update(&mut self, _: &mut Update) -> Option<Command> {
        // TODO: update the play menu state

        None
    }

    /// Renders the play menu.
    pub fn render(&mut self, _: &mut Render) {
        // TODO: video the play menu
    }
}
