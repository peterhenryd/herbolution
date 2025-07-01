use engine::painter::brush::Brush;

use crate::app::Update;
use crate::app::state::Command;

/// The play menu, where the user can view pre-existing saves, create new ones, or start the herbolution_game.
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
    pub fn render<'t>(&'t mut self, _: &mut Brush<'_, '_, '_>) {
        // TODO: render the play menu
    }
}
