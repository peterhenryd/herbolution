use engine::video::painter::Brush;
use crate::app::state::Command;
use crate::app::Update;

/// The options menu, where the user can view and modify the application settings.
#[derive(Debug)]
pub struct OptionsMenu {}

impl OptionsMenu {
    /// Creates a new instance of the options menu.
    pub fn new() -> Self {
        Self {}
    }

    /// Updates the options menu state.
    pub fn update(&mut self, _: &mut Update) -> Option<Command> {
        // TODO: update the options menu state

        None
    }

    /// Renders the options menu.
    pub fn render<'t>(&'t mut self, _: &mut Brush<'_, '_, '_>) {
        // TODO: render the options menu
    }
}
